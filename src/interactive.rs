// SPDX-License-Identifier: Apache-2.0

use std::any::Any;

use crate::Keystroke;

#[derive(Debug)]
#[derive(Clone)]
pub enum PlatformInput {
  KeyDown(KeyDownEvent),
  KeyUp(KeyUpEvent),
}
impl InputEvent for PlatformInput {
  fn to_platform_event(self) -> PlatformInput {
    self
  }
}

pub trait InputEvent: 'static + Any {
  fn to_platform_event(self) -> PlatformInput;
}
pub trait KeyEvent: InputEvent {}

#[derive(Debug)]
#[derive(Clone)]
pub struct KeyDownEvent {
  pub keystroke: Keystroke,
  pub is_held: bool,
}
impl InputEvent for KeyDownEvent {
  fn to_platform_event(self) -> PlatformInput {
    PlatformInput::KeyDown(self)
  }
}
impl KeyEvent for KeyDownEvent {}

#[derive(Debug)]
#[derive(Clone)]
pub struct KeyUpEvent {
  pub keystroke: Keystroke,
}
impl InputEvent for KeyUpEvent {
  fn to_platform_event(self) -> PlatformInput {
    PlatformInput::KeyUp(self)
  }
}
impl KeyEvent for KeyUpEvent {}
