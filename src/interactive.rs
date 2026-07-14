// SPDX-License-Identifier: Apache-2.0

use std::any::Any;

use crate::Keystroke;

#[derive(Debug)]
#[derive(Clone)]
pub enum PlatformInput {
  KeyDown(KeyDownEvent),
  KeyUp(KeyUpEvent),
  MouseButtonDown(()),
  MouseButtonUp(()),
  MouseMove(()),
}
impl PlatformInput {
  pub(crate) fn keyboard_event(&self) -> Option<&dyn Any> {
    match self {
      Self::KeyDown(event) => Some(event),
      Self::KeyUp(event) => Some(event),
      Self::MouseButtonDown(..)
      | Self::MouseButtonUp(..)
      | Self::MouseMove(..) => None,
    }
  }
  pub(crate) fn mouse_event(&self) -> Option<&dyn Any> {
    match self {
      Self::KeyDown(..) | Self::KeyUp(..) => None,
      Self::MouseButtonDown(event) => Some(event),
      Self::MouseButtonUp(event) => Some(event),
      Self::MouseMove(event) => Some(event),
    }
  }
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
