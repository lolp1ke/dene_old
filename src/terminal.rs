// SPDX-License-Identifier: Apache-2.0

use std::io::{Stdout, Write, stdout};

use crossterm::{
  cursor, execute,
  style::{SetBackgroundColor, SetForegroundColor},
  terminal::{self, Clear, ClearType},
};

use crate::{Color, DrawOp, Frame};

#[derive(Debug)]
pub struct Terminal {
  stdout: Stdout,
}
impl Terminal {
  pub fn init() -> Self {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(
      stdout,
      terminal::EnterAlternateScreen,
      cursor::Hide,
      terminal::Clear(ClearType::All),
    )
    .unwrap();
    crossterm::execute!(
      stdout,
      crossterm::event::PushKeyboardEnhancementFlags(
        crossterm::event::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
      )
    )
    .unwrap();
    Self { stdout }
  }

  pub fn restore(&self) {
    let _ = terminal::disable_raw_mode();
    let mut stdout = &self.stdout;
    let _ = execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen,);
  }

  pub fn size() -> (u16, u16) {
    terminal::size().expect("failed to retrieve the size of the terminal")
  }

  pub fn flush_diff(&mut self, prev: &Frame, current: &Frame) {
    let ops = prev.diff(current);
    if ops.is_empty() {
      return;
    }

    let mut last_x = u16::MAX;
    let mut last_y = u16::MAX;

    for op in &ops {
      match op {
        DrawOp::Cell { x, y, cell } => {
          if *y != last_y || *x != last_x {
            let _ = execute!(self.stdout, cursor::MoveTo(*x, *y));
          }
          let fg = cell.fg.into();
          let bg = cell.bg.into();
          let _ = execute!(
            self.stdout,
            SetForegroundColor(fg),
            SetBackgroundColor(bg),
          );
          let _ = write!(self.stdout, "{}", cell.symbol);
          last_x = x + 1;
          last_y = *y;
        }
        DrawOp::Flush => {
          let _ = self.stdout.flush();
          last_x = u16::MAX;
          last_y = u16::MAX;
        }
      }
    }
    let _ = self.stdout.flush();
  }

  pub fn draw_frame(&mut self, frame: &Frame) {
    let _ = execute!(self.stdout, cursor::MoveTo(0, 0), Clear(ClearType::All),);

    for y in 0..frame.height {
      let _ = execute!(self.stdout, cursor::MoveTo(0, y));
      for x in 0..frame.width {
        let cell = frame.get_cell(x, y);
        let fg = cell.fg.into();
        let bg = cell.bg.into();
        let _ = execute!(
          self.stdout,
          SetForegroundColor(fg),
          SetBackgroundColor(bg),
        );
        let _ = write!(self.stdout, "{}", cell.symbol);
      }
    }
    let _ = self.stdout.flush();
  }
}

impl From<Color> for crossterm::style::Color {
  fn from(value: Color) -> Self {
    use crossterm::style::Color::*;

    match value {
      Color::Default => Reset,
      Color::Rgb { r, g, b } => Rgb {
        r: r as u8,
        g: g as u8,
        b: b as u8,
      },
      Color::Rgba { r, g, b, a: _ } => Rgb {
        r: r as u8,
        g: g as u8,
        b: b as u8,
      },
    }
  }
}
