// SPDX-License-Identifier: Apache-2.0

use crate::{Color, DispatchTree};

#[derive(Debug)]
pub struct Frame {
  cells: Vec<FrameCell>,
  pub(crate) width: u16,
  pub(crate) height: u16,

  pub(crate) dispatch_tree: DispatchTree,
}
impl Frame {
  pub(crate) fn new(
    width: u16,
    height: u16,
    dispatch_tree: DispatchTree,
  ) -> Self {
    let len = (width as usize) * (height as usize);
    Self {
      cells: vec![Default::default(); len],
      width,
      height,
      dispatch_tree,
    }
  }

  pub(crate) fn size(&self) -> (u16, u16) {
    (self.width, self.height)
  }

  pub(crate) fn clear(&mut self) {
    for cell in &mut self.cells {
      *cell = Default::default();
    }
  }

  pub(crate) fn resize(&mut self, width: u16, height: u16) {
    self.width = width;
    self.height = height;
    let len = (width as usize) * (height as usize);
    self.clear();
    self.cells.resize_with(len, FrameCell::default);
    self.clear();
  }

  pub(crate) fn set_cell(&mut self, x: u16, y: u16, cell: FrameCell) {
    if x < self.width && y < self.height {
      let idx = (y as usize) * (self.width as usize) + (x as usize);
      self.cells[idx] = cell;
    }
  }

  pub(crate) fn get_cell(&self, x: u16, y: u16) -> &FrameCell {
    if x < self.width && y < self.height {
      let idx = (y as usize) * (self.width as usize) + (x as usize);
      &self.cells[idx]
    } else {
      static DEFAULT: FrameCell = FrameCell {
        symbol: String::new(),
        fg: Color::Rgb {
          r: 255,
          g: 255,
          b: 255,
        },
        bg: Color::Default,
      };
      &DEFAULT
    }
  }

  pub(crate) fn write_string(
    &mut self,
    x: u16,
    y: u16,
    s: &str,
    fg: Color,
    bg: Color,
  ) {
    for (i, ch) in s.chars().enumerate() {
      let cx = x + i as u16;
      if cx >= self.width {
        break;
      }
      self.set_cell(
        cx,
        y,
        FrameCell {
          symbol: ch.to_string(),
          fg,
          bg,
        },
      );
    }
  }

  pub(crate) fn diff(&self, other: &Self) -> Vec<DrawOp> {
    let mut ops = Vec::new();
    for y in 0..self.height.min(other.height) {
      for x in 0..self.width.min(other.width) {
        let a = self.get_cell(x, y);
        let b = other.get_cell(x, y);
        if a != b {
          ops.push(DrawOp::Cell {
            x,
            y,
            cell: b.clone(),
          });
        }
      }
    }
    ops
  }
}

#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub(crate) struct FrameCell {
  pub(crate) symbol: String,
  pub(crate) fg: Color,
  pub(crate) bg: Color,
}
impl Default for FrameCell {
  fn default() -> Self {
    Self {
      symbol: " ".into(),
      fg: Color::Rgb {
        r: 255,
        g: 255,
        b: 255,
      },
      bg: Color::Default,
    }
  }
}
#[derive(Debug, Clone)]
pub(crate) enum DrawOp {
  Cell { x: u16, y: u16, cell: FrameCell },
  Flush,
}
