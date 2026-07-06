// SPDX-License-Identifier: Apache-2.0

use crate::{App, Color, Element, Frame, IntoElement, Rect, Window};

#[derive(Debug)]
pub struct Text {
  text: String,
  style: taffy::Style,
}

impl Text {
  pub fn new(text: impl Into<String>) -> Self {
    let text = text.into();
    let mut style = taffy::Style::DEFAULT;
    style.size = taffy::Size {
      width: taffy::Dimension::length(text.len() as f32),
      height: taffy::Dimension::length(text.lines().count() as f32),
    };

    Self { text, style }
  }

  pub fn with_style(mut self, style: taffy::Style) -> Self {
    self.style = style;
    self
  }
}

impl Element for Text {
  fn layout_style(&self) -> taffy::Style {
    self.style.clone()
  }

  fn child_count(&self) -> usize {
    0
  }

  fn render(
    &mut self,
    bounds: Rect,
    frame: &mut Frame,
    _window: &mut Window,
    _cx: &mut App,
  ) {
    let lines = self.text.lines();
    for (i, line) in lines.enumerate() {
      let y = bounds.y + i as u16;
      if y >= bounds.y.saturating_add(bounds.height) {
        break;
      }

      frame.write_string(
        bounds.x,
        y,
        line,
        Color::Rgb {
          r: 255,
          g: 255,
          b: 255,
        },
        Color::Default,
      );
    }
  }
}

impl IntoElement for Text {
  type Element = Self;

  fn into_element(self) -> Self::Element {
    self
  }
}

pub fn text(content: impl Into<String>) -> Text {
  Text::new(content)
}

impl Element for String {}
impl IntoElement for String {
  type Element = Text;

  fn into_element(self) -> Self::Element {
    Text::new(self)
  }
}

impl Element for &'static str {}
impl IntoElement for &'_ str {
  type Element = Text;

  fn into_element(self) -> Self::Element {
    Text::new(self)
  }
}
