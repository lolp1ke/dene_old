// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rect {
  pub x: u16,
  pub y: u16,
  pub width: u16,
  pub height: u16,
}
impl Rect {
  pub const ZERO: Self = Self {
    x: 0,
    y: 0,
    width: 0,
    height: 0,
  };

  pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
    Self {
      x,
      y,
      width,
      height,
    }
  }

  pub fn resize(self, width: u16, height: u16) -> Self {
    Self {
      x: self.x,
      y: self.y,
      width,
      height,
    }
  }

  pub fn contains(&self, x: u16, y: u16) -> bool {
    x >= self.x
      && x < self.x.saturating_add(self.width)
      && y >= self.y
      && y < self.y.saturating_add(self.height)
  }

  pub fn intersection(&self, other: &Self) -> Self {
    let x = self.x.max(other.x);
    let y = self.y.max(other.y);
    let x2 = self
      .x
      .saturating_add(self.width)
      .min(other.x.saturating_add(other.width));
    let y2 = self
      .y
      .saturating_add(self.height)
      .min(other.y.saturating_add(other.height));
    Self {
      x,
      y,
      width: x2.saturating_sub(x),
      height: y2.saturating_sub(y),
    }
  }

  pub fn is_empty(&self) -> bool {
    self.width == 0 || self.height == 0
  }
}
