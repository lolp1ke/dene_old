// SPDX-License-Identifier: Apache-2.0

use std::{
  any::{Any, TypeId},
  marker::PhantomData,
  rc::Rc,
  sync::OnceLock,
};

use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use slotmap::new_key_type;

use crate::{
  Action, AnyView, App, AppContext, Frame, IntoElement, KeyEvent, LayoutEngine,
  PlatformInput, Rect, Terminal,
};

pub(crate) static TERM: OnceLock<RwLock<Terminal>> = OnceLock::new();

pub(crate) fn get_terminal() -> &'static RwLock<Terminal> {
  TERM.get().expect("terminal not initialized")
}

type ActionListener = Rc<dyn Fn(&dyn Action, &mut Window, &mut App)>;
type KeyListener = Rc<dyn Fn(&dyn Any, &mut Window, &mut App)>;

#[derive(derive_more::Debug)]
pub struct Window {
  handle: AnyWindowHandle,

  pub root: Option<AnyView>,

  pub(crate) bounds: Rect,
  pub(crate) dirty: bool,

  pub(crate) prev_frame: Frame,
  pub(crate) current_frame: Frame,

  #[debug(skip)]
  action_listeners: FxHashMap<TypeId, Vec<ActionListener>>,
  #[debug(skip)]
  key_events: Vec<KeyListener>,

  pub(crate) layout_engine: LayoutEngine,
}
impl Window {
  pub(crate) fn new(handle: AnyWindowHandle, config: WindowConfig) -> Self {
    let WindowConfig { bounds, .. } = config;

    Self {
      handle,
      root: None,
      bounds,
      dirty: false,
      prev_frame: Frame::new(bounds.width, bounds.height),
      current_frame: Frame::new(bounds.width, bounds.height),
      action_listeners: Default::default(),
      key_events: Default::default(),
      layout_engine: Default::default(),
    }
  }

  pub(crate) fn render(&mut self, cx: &mut App) {
    let Some(root_view) = self.root.as_ref().cloned() else {
      return;
    };
    let mut root_element = root_view.into_any_element();
    let mut layout_engine = std::mem::take(&mut self.layout_engine);
    let mut frame = std::mem::replace(
      &mut self.current_frame,
      Frame::new(self.bounds.width, self.bounds.height),
    );

    frame.clear();

    crate::render_element_with_layout(
      &mut root_element,
      &mut layout_engine,
      self.bounds.width,
      self.bounds.height,
      &mut frame,
      self,
      cx,
    );

    self.layout_engine = layout_engine;
    std::mem::swap(&mut self.prev_frame, &mut self.current_frame);
    self.current_frame = frame;

    let term = get_terminal();
    let mut term = term.write();
    term.flush_diff(&self.prev_frame, &self.current_frame);
    std::mem::swap(&mut self.prev_frame, &mut self.current_frame);
  }

  pub(crate) fn dispatch_action(&mut self, action: &dyn Action, cx: &mut App) {
    let action_ty = action.as_any().type_id();
    if let Some(global_listeners) =
      cx.global_action_listeners.remove(&action_ty)
    {
      for listener in global_listeners.iter() {
        (listener)(action, cx);
      }

      cx.global_action_listeners
        .insert(action_ty, global_listeners);
    };

    if let Some(listeners) = self.action_listeners.remove(&action_ty) {
      for listener in listeners.iter() {
        (listener)(action, self, cx);
      }

      self.action_listeners.insert(action_ty, listeners);
    };
  }
  pub(crate) fn dispatch_input(
    &mut self,
    platform_input: PlatformInput,
    cx: &mut App,
  ) {
    let key_events = std::mem::take(&mut self.key_events);

    for listener in key_events.iter() {
      match platform_input.clone() {
        PlatformInput::KeyDown(key_event) => {
          (listener)(&key_event, self, cx);
        }
        PlatformInput::KeyUp(key_event) => {
          (listener)(&key_event, self, cx);
        }
      };
    }

    // TODO: implement focused state for new render logic
    // if let Some(view) = self.root.as_ref().cloned() {
    //   (view.on_keystroke)(&view, keystroke, self, cx);
    // };

    if self.dirty {
      self.render(cx);
      self.dirty = false;
    };
  }

  pub fn on_action<F, A>(&mut self, f: F)
  where
    F: 'static + Fn(&A, &mut Self, &mut App),
    A: Action,
  {
    self
      .action_listeners
      .entry(TypeId::of::<A>())
      .or_default()
      .push(Rc::new(move |action, window, cx| {
        let action = action.as_any().downcast_ref().expect("wrong action");
        f(action, window, cx);
      }));
  }

  pub(crate) fn on_key_event<F, Event>(&mut self, listener: F)
  where
    F: 'static + Fn(&Event, &mut Window, &mut App),
    Event: KeyEvent,
  {
    self.key_events.push(Rc::new(move |key_event, window, cx| {
      if let Some(key_event) = key_event.downcast_ref::<Event>() {
        (listener)(key_event, window, cx);
      };
    }));
  }
}

#[derive(Debug)]
pub struct WindowConfig {
  pub bounds: Rect,
}
impl Default for WindowConfig {
  fn default() -> Self {
    let (width, height) = Terminal::size();

    Self {
      bounds: Rect {
        x: 0,
        y: 0,
        width,
        height,
      },
    }
  }
}

new_key_type! {
  pub struct WindowId;
}
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct AnyWindowHandle {
  pub(crate) window_id: WindowId,
  ty: TypeId,
}
impl AnyWindowHandle {
  fn new<W>(window_id: WindowId) -> Self
  where
    W: 'static,
  {
    Self {
      window_id,
      ty: TypeId::of::<W>(),
    }
  }

  pub(crate) fn update<C, F, R>(self, cx: &mut C, f: F) -> anyhow::Result<R>
  where
    C: AppContext,
    F: FnOnce(AnyView, &mut Window, &mut App) -> R,
  {
    cx.update_window(self, f)
  }
}
impl<W> From<WindowHandle<W>> for AnyWindowHandle {
  fn from(value: WindowHandle<W>) -> Self {
    value.handle
  }
}

#[derive(Debug)]
#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct WindowHandle<W> {
  #[deref]
  #[deref_mut]
  handle: AnyWindowHandle,
  _marker: PhantomData<W>,
}
impl<W> WindowHandle<W> {
  pub fn new(window_id: WindowId) -> Self
  where
    W: 'static,
  {
    Self {
      handle: AnyWindowHandle::new::<W>(window_id),
      _marker: PhantomData,
    }
  }
}
impl<W> Copy for WindowHandle<W> {}
impl<W> Clone for WindowHandle<W> {
  fn clone(&self) -> Self {
    *self
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
  Default,
  Rgb { r: u16, g: u16, b: u16 },
  Rgba { r: u16, g: u16, b: u16, a: u16 },
}
