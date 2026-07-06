// SPDX-License-Identifier: Apache-2.0

use crate::{
  AnyElement, App, Element, Frame, Interactivity, IntoElement, Rect, Styled,
  Window,
};

#[derive(Debug)]
#[derive(Default)]
pub struct Div {
  interactivity: Interactivity,
  children: Vec<AnyElement>,
}
impl Styled for Div {
  fn style(&mut self) -> &mut taffy::Style {
    &mut self.interactivity.base_style
  }
}

impl Div {
  pub fn child(mut self, child: impl IntoElement) -> Self {
    self.children.push(child.into_any_element());
    self
  }

  pub fn children(
    mut self,
    children: impl IntoIterator<Item = AnyElement>,
  ) -> Self {
    self.children.extend(children);
    self
  }

  pub fn with_style(mut self, style: taffy::Style) -> Self {
    self.interactivity.base_style = Box::new(style);
    self
  }
}

impl Element for Div {
  fn layout_style(&self) -> taffy::Style {
    // let mut style: taffy::Style = (&*self.interactivity.base_style).into();
    let mut style = self.interactivity.base_style.clone();
    if !self.children.is_empty() {
      style.display = taffy::Display::Flex;
    }
    *style
  }

  fn child_count(&self) -> usize {
    self.children.len()
  }

  fn get_child(&mut self, index: usize) -> &mut AnyElement {
    &mut self.children[index]
  }

  fn render(
    &mut self,
    bounds: Rect,
    frame: &mut Frame,
    window: &mut Window,
    cx: &mut App,
  ) {
    if matches!(self.interactivity.base_style.display, taffy::Display::None) {
      return;
    }

    for child in &mut self.children {
      child.render(bounds, frame, window, cx);
    }
  }
}
impl IntoElement for Div {
  type Element = Self;
  fn into_element(self) -> Self::Element {
    self
  }
}

pub fn div() -> Div {
  Default::default()
}
