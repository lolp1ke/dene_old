// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;

use crate::{Context, Element, IntoElement, Render, Window};

#[derive(Debug)]
pub struct Empty;

impl Element for Empty {}
impl IntoElement for Empty {
  type Element = Self;
  fn into_element(self) -> Self::Element {
    self
  }
}
impl Render for Empty {
  fn render(
    &mut self,
    _window: &mut Window,
    _cx: &mut Context<Self>,
  ) -> impl IntoElement {
    Self
  }
}
