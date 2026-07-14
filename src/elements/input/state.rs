// SPDX-License-Identifier: Apache-2.0

use ropey::Rope;

use super::*;

#[derive(Debug)]
pub struct InputState {
  pub(crate) text: Rope,

  pub(crate) disabled: bool,
}
impl InputState {
  pub fn new() -> Self {
    Self {
      text: Rope::new(),
      disabled: false,
    }
  }

  pub(crate) fn delete(
    &mut self,
    _: &Delete,
    window: &mut Window,
    cx: &mut Context<Self>,
  ) {
    panic!("YEY");
  }
}
