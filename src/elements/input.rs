// SPDX-License-Identifier: Apache-2.0

mod actions;
mod state;

pub use actions::*;
pub use state::*;

use taffy::Style;

use crate::{
  App, AppContext, Context, Element, ElementExt, Entity, Frame,
  InteractiveElement, IntoElement, KeyBindingContextPredicate, KeyContext,
  Rect, Render, Window, elements::div,
};

#[derive(Debug)]
pub struct Input {
  state: Entity<InputState>,
  style: Style,
}
impl Input {
  pub fn new(state: Entity<InputState>) -> Self {
    Self {
      state,
      style: Style::DEFAULT,
    }
  }
}
impl Render for Input {
  fn render(
    &mut self,
    window: &mut Window,
    cx: &mut Context<Self>,
  ) -> impl IntoElement {
    let state = self.state.read(cx);

    div()
      .when(!state.disabled, |this| {
        this.on_action(window.listener(&self.state, InputState::delete))
      })
      .child("placeholder")
  }
}
