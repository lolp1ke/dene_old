// SPDX-License-Identifier: Apache-2.0

use std::{any::Any, fmt::Debug};

use crate::{App, Frame, Rect, Window};

pub trait IntoElement: Sized {
  type Element: Element;

  fn into_element(self) -> Self::Element;
  fn into_any_element(self) -> AnyElement {
    self.into_element().into_any()
  }
}
pub trait Element: 'static + IntoElement {
  fn layout_style(&self) -> taffy::Style {
    taffy::Style::default()
  }
  fn child_count(&self) -> usize {
    0
  }
  fn get_child(&mut self, _index: usize) -> &mut AnyElement {
    panic!("no children");
  }
  fn render(
    &mut self,
    _bounds: Rect,
    _frame: &mut Frame,
    _window: &mut Window,
    _cx: &mut App,
  ) {
  }

  fn into_any(self) -> AnyElement
  where
    Self: Sized,
  {
    AnyElement(Box::new(DrawableObject::new(self)))
  }
}

#[derive(Debug)]
pub struct AnyElement(pub Box<dyn ElementObject>);
impl AnyElement {
  pub fn layout_style(&self) -> taffy::Style {
    self.0.layout_style()
  }
  pub fn child_count(&self) -> usize {
    self.0.child_count()
  }
  pub fn get_child(&mut self, index: usize) -> &mut AnyElement {
    self.0.get_child(index)
  }
  pub fn render(
    &mut self,
    bounds: Rect,
    frame: &mut Frame,
    window: &mut Window,
    cx: &mut App,
  ) {
    self.0.render(bounds, frame, window, cx);
  }
}

#[derive(Debug)]
pub struct DrawableObject<E>
where
  E: Element,
{
  pub element: E,
}
impl<E> DrawableObject<E>
where
  E: Element,
{
  pub fn new(element: E) -> Self {
    Self { element }
  }
}
impl<E> ElementObject for DrawableObject<E>
where
  E: Element,
{
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn layout_style(&self) -> taffy::Style {
    self.element.layout_style()
  }
  fn child_count(&self) -> usize {
    self.element.child_count()
  }
  fn get_child(&mut self, index: usize) -> &mut AnyElement {
    self.element.get_child(index)
  }
  fn render(
    &mut self,
    bounds: Rect,
    frame: &mut Frame,
    window: &mut Window,
    cx: &mut App,
  ) {
    self.element.render(bounds, frame, window, cx);
  }
}

pub trait ElementObject {
  fn as_any_mut(&mut self) -> &mut dyn Any;

  fn layout_style(&self) -> taffy::Style {
    taffy::Style::default()
  }
  fn child_count(&self) -> usize {
    0
  }
  fn get_child(&mut self, _index: usize) -> &mut AnyElement {
    assert_eq!(self.child_count(), 0);
    unreachable!("leaf should be created");
  }
  fn render(
    &mut self,
    _bounds: Rect,
    _frame: &mut Frame,
    _window: &mut Window,
    _cx: &mut App,
  ) {
  }
}
impl Debug for dyn ElementObject {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("dyn ElementObject").finish_non_exhaustive()
  }
}

#[derive(Debug)]
#[derive(Default)]
pub struct Interactivity {
  pub active: bool,
  pub hovered: bool,
  pub focusable: bool,
  pub base_style: Box<taffy::Style>,
}
