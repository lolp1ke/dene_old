// SPDX-License-Identifier: Apache-2.0

use std::{
  any::{Any, TypeId},
  fmt::Debug,
};

use crate::{
  Action, App, DispatchPhase, Entity, FocusHandle, Frame,
  KeyBindingContextPredicate, KeyContext, KeyDownEvent, KeyUpEvent, Rect,
  Window,
};

pub trait IntoElement: Sized + Debug {
  type Element: Element;

  fn into_element(self) -> Self::Element;
  fn into_any_element(self) -> AnyElement {
    self.into_element().into_any()
  }
}
pub trait Element: 'static + IntoElement {
  fn layout_style(&self) -> taffy::Style {
    Default::default()
  }
  fn child_count(&self) -> Option<usize> {
    None
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
pub struct AnyElement(pub(crate) Box<dyn ElementObject>);
impl Element for AnyElement {
  fn layout_style(&self) -> taffy::Style {
    self.0.layout_style()
  }
  fn child_count(&self) -> Option<usize> {
    self.0.child_count()
  }
  fn get_child(&mut self, index: usize) -> &mut AnyElement {
    self.0.get_child(index)
  }
  fn render(
    &mut self,
    bounds: Rect,
    frame: &mut Frame,
    window: &mut Window,
    cx: &mut App,
  ) {
    self.0.render(bounds, frame, window, cx);
  }
}
impl IntoElement for AnyElement {
  type Element = Self;
  fn into_element(self) -> Self::Element {
    self
  }
}

#[derive(Debug)]
pub(crate) struct DrawableObject<E>
where
  E: Element,
{
  pub element: E,
}
impl<E> DrawableObject<E>
where
  E: Element,
{
  pub(crate) fn new(element: E) -> Self {
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
  fn child_count(&self) -> Option<usize> {
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

pub(crate) trait ElementObject {
  fn as_any_mut(&mut self) -> &mut dyn Any;

  fn layout_style(&self) -> taffy::Style {
    Default::default()
  }
  fn child_count(&self) -> Option<usize> {
    None
  }
  fn get_child(&mut self, _index: usize) -> &mut AnyElement {
    assert_eq!(self.child_count(), None);
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

pub trait InteractiveElement: Sized {
  fn interactivity(&mut self) -> &mut Interactivity;

  fn on_action<A, F>(mut self, listener: F) -> Self
  where
    A: Action,
    F: 'static + Fn(&A, &mut Window, &mut App),
  {
    self.interactivity().action_listeners.push((
      TypeId::of::<A>(),
      Box::new(move |action, phase, window, cx| {
        let action = action.downcast_ref::<A>().unwrap();
        if phase == DispatchPhase::Bubble {
          (listener)(action, window, cx);
        };
      }),
    ));
    self
  }

  fn on_key_down<F>(mut self, listener: F) -> Self
  where
    F: 'static + Fn(&KeyDownEvent, &mut Window, &mut App),
  {
    self.interactivity().key_down_listeners.push(Box::new(
      move |event, phase, window, cx| {
        if phase == DispatchPhase::Bubble {
          (listener)(event, window, cx);
        };
      },
    ));
    self
  }
  fn on_key_up<F>(mut self, listener: F) -> Self
  where
    F: 'static + Fn(&KeyUpEvent, &mut Window, &mut App),
  {
    self.interactivity().key_up_listeners.push(Box::new(
      move |event, phase, window, cx| {
        if phase == DispatchPhase::Bubble {
          (listener)(event, window, cx);
        };
      },
    ));
    self
  }

  fn track_focus(mut self, focus_handle: &FocusHandle) -> Self {
    self.interactivity().focusable = true;
    self.interactivity().focus_handle = Some(focus_handle.clone());
    self
  }
  fn tab_index(mut self, tab_index: u32) -> Self {
    self.interactivity().focusable = true;
    self.interactivity().tab_stop = true;
    self.interactivity().tab_index = Some(tab_index);
    self
  }
  fn tab_stop(mut self, tab_stop: bool) -> Self {
    self.interactivity().tab_stop = tab_stop;
    self
  }

  fn key_context(mut self, context: KeyContext) -> Self {
    self.interactivity().key_context = Some(context);
    self
  }
}

type ActionListener =
  Box<dyn 'static + Fn(&dyn Any, DispatchPhase, &mut Window, &mut App)>;
type KeyDownListener =
  Box<dyn 'static + Fn(&KeyDownEvent, DispatchPhase, &mut Window, &mut App)>;
type KeyUpListener =
  Box<dyn 'static + Fn(&KeyUpEvent, DispatchPhase, &mut Window, &mut App)>;

#[derive(derive_more::Debug)]
#[derive(Default)]
pub struct Interactivity {
  pub active: bool,
  pub hovered: bool,
  pub focusable: bool,

  #[debug(skip)]
  pub base_style: Box<taffy::Style>,

  pub focus_handle: Option<FocusHandle>,
  pub tab_index: Option<u32>,
  pub tab_stop: bool,

  #[debug(skip)]
  pub action_listeners: Vec<(TypeId, ActionListener)>,

  #[debug(skip)]
  pub key_down_listeners: Vec<KeyDownListener>,
  #[debug(skip)]
  pub key_up_listeners: Vec<KeyUpListener>,

  pub key_context: Option<KeyContext>,
}
impl Interactivity {
  pub(crate) fn apply_keyboard_listeners(
    &mut self,
    window: &mut Window,
    _cx: &mut App,
  ) {
    if let Some(context) = self.key_context.take() {
      // window.set_key_context(context);
    }

    let key_down_listeners = std::mem::take(&mut self.key_down_listeners);
    for listener in key_down_listeners.into_iter() {
      window.on_key_event(move |event: &KeyDownEvent, phase, window, cx| {
        (listener)(event, phase, window, cx);
      });
    }

    let key_up_listeners = std::mem::take(&mut self.key_up_listeners);
    for listener in key_up_listeners.into_iter() {
      window.on_key_event(move |event: &KeyUpEvent, phase, window, cx| {
        (listener)(event, phase, window, cx);
      });
    }

    let action_listeners = std::mem::take(&mut self.action_listeners);
    for (action_ty, listener) in action_listeners.into_iter() {
      window.on_action(action_ty, listener);
    }
  }
}

pub trait ElementExt {
  fn map<F, R>(self, f: F) -> R
  where
    Self: Sized,
    F: FnOnce(Self) -> R,
  {
    f(self)
  }

  fn when<F>(self, condition: bool, f: F) -> Self
  where
    Self: Sized,
    F: FnOnce(Self) -> Self,
  {
    self.map(|this| if condition { f(this) } else { this })
  }
}

impl<T: IntoElement> ElementExt for T {}
