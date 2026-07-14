// SPDX-License-Identifier: Apache-2.0

use std::{
  any::{Any, TypeId},
  marker::PhantomData,
  rc::Rc,
  sync::OnceLock,
};

use parking_lot::RwLock;
use slotmap::new_key_type;
use smallvec::SmallVec;

use crate::{
  Action, AnyView, App, AppContext, Context, DispatchNodeId, DispatchPhase,
  DispatchTree, Entity, FocusHandle, FocusId, Frame, IntoElement,
  KeyBindingContextPredicate, KeyContext, KeyDownEvent, KeyEvent, Keystroke,
  LayoutEngine, PlatformInput, Rect, Terminal,
};

pub(crate) static TERM: OnceLock<RwLock<Terminal>> = OnceLock::new();

pub(crate) fn get_terminal() -> &'static RwLock<Terminal> {
  TERM.get().expect("terminal not initialized")
}

#[derive(derive_more::Debug)]
pub struct Window {
  handle: AnyWindowHandle,

  pub(crate) root: Option<AnyView>,

  pub(crate) focus: Option<FocusId>,
  pub(crate) bounds: Rect,
  pub(crate) dirty: bool,

  pub(crate) prev_frame: Frame,
  pub(crate) current_frame: Frame,

  pub(crate) layout_engine: LayoutEngine,
}
impl Window {
  pub(crate) fn new(
    handle: AnyWindowHandle,
    config: WindowConfig,
    cx: &mut App,
  ) -> Self {
    let WindowConfig { bounds, .. } = config;

    Self {
      handle,
      root: None,
      focus: None,
      bounds,
      dirty: false,
      prev_frame: Frame::new(
        bounds.width,
        bounds.height,
        DispatchTree::new(cx.keybinds.clone(), cx.actions.clone()),
      ),
      current_frame: Frame::new(
        bounds.width,
        bounds.height,
        DispatchTree::new(cx.keybinds.clone(), cx.actions.clone()),
      ),
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
      Frame::new(
        self.bounds.width,
        self.bounds.height,
        DispatchTree::new(cx.keybinds.clone(), cx.actions.clone()),
      ),
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

  pub(crate) fn dispatch_input(&mut self, input: PlatformInput, cx: &mut App) {
    if let Some(key_event) = input.keyboard_event() {
      self.dispatch_key_event(key_event, cx);
    } else if let Some(mouse_event) = input.mouse_event() {
      todo!();
    };
  }
  pub(crate) fn dispatch_key_event(
    &mut self,
    key_event: &dyn Any,
    cx: &mut App,
  ) {
    if self.dirty {
      self.render(cx);
    };

    let node_id = self
      .focus
      .and_then(|focus_id| {
        self.current_frame.dispatch_tree.focusable_node_id(focus_id)
      })
      .unwrap_or_else(|| self.current_frame.dispatch_tree.root_node_id());

    let dispatch_path =
      &self.current_frame.dispatch_tree.dispatch_path(node_id);

    if let Some(key_event) = key_event.downcast_ref::<KeyDownEvent>() {
      self.dispatch_key_down_up_event(key_event, dispatch_path, cx);
    };
  }

  pub(crate) fn dispatch_key_down_up_event(
    &mut self,
    key_event: &dyn Any,
    dispatch_path: &SmallVec<[DispatchNodeId; 32]>,
    cx: &mut App,
  ) {
    // capture
    for node_id in dispatch_path.iter() {
      let node = self.current_frame.dispatch_tree.node(*node_id);

      for listener in node.key_listeners.clone() {
        (listener)(key_event, DispatchPhase::Capture, self, cx);
      }
    }

    // bubble
    for node_id in dispatch_path.iter().rev() {
      let node = self.current_frame.dispatch_tree.node(*node_id);

      for listener in node.key_listeners.clone() {
        (listener)(key_event, DispatchPhase::Bubble, self, cx);
      }
    }
  }

  // pub(crate) fn dispatch_keystroke(
  //   &mut self,
  //   keystroke: Keystroke,
  //   cx: &mut App,
  // ) {
  //   if let Some(node_id) = self.focused_dispatch_node() {
  //     let path = self.current_frame.dispatch_tree.dispatch_path(node_id);
  //     let mut matched = self
  //       .current_frame
  //       .dispatch_tree
  //       .bindings_for_input(&keystroke, &path);
  //     matched.sort_by(|a, b| b.1.cmp(&a.1));

  //     let keybinds = self.current_frame.dispatch_tree.keybinds.clone();
  //     let keybinds = keybinds.borrow();
  //     if let Some((bind_idx, _depth)) = matched.first() {
  //       let keybind = &keybinds[*bind_idx];
  //       let action = &*keybind.action;
  //       self.dispatch_action_on_node(action, node_id, cx);
  //     }
  //   } else {
  //     let mut matched = self
  //       .current_frame
  //       .dispatch_tree
  //       .match_bindings_for_all_nodes(&keystroke);
  //     matched.sort_by(|a, b| b.1.cmp(&a.1));

  //     let keybinds = self.current_frame.dispatch_tree.keybinds.clone();
  //     let keybinds = keybinds.borrow();
  //     if let Some((bind_idx, _depth, target)) = matched.first() {
  //       let keybind = &keybinds[*bind_idx];
  //       let action = &*keybind.action;
  //       self.dispatch_action_on_node(action, *target, cx);
  //     }
  //   }
  // }

  // fn dispatch_action_on_node(
  //   &mut self,
  //   action: &dyn Action,
  //   target: DispatchNodeId,
  //   cx: &mut App,
  // ) {
  //   let action_ty = action.as_any().type_id();
  //   let path = self.current_frame.dispatch_tree.dispatch_path(target);

  //   for &node_id in &path {
  //     let listener = self
  //       .current_frame
  //       .dispatch_tree
  //       .node(node_id)
  //       .action_listeners
  //       .get(&action_ty)
  //       .cloned();
  //     if let Some(listener) = listener {
  //       listener(action, DispatchPhase::Capture, self, cx);
  //     }
  //   }

  //   if let Some(listeners) = cx.global_action_listeners.remove(&action_ty) {
  //     for listener in listeners.iter() {
  //       (listener)(action, cx);
  //     }
  //     cx.global_action_listeners.insert(action_ty, listeners);
  //   };

  //   for &node_id in path.iter().rev() {
  //     let listener = self
  //       .current_frame
  //       .dispatch_tree
  //       .node(node_id)
  //       .action_listeners
  //       .get(&action_ty)
  //       .cloned();
  //     if let Some(listener) = listener {
  //       listener(action, DispatchPhase::Bubble, self, cx);
  //     }
  //   }
  // }

  pub fn on_action<F>(&mut self, action_ty: TypeId, listener: F)
  where
    F: 'static + Fn(&dyn Any, DispatchPhase, &mut Self, &mut App),
  {
    self
      .current_frame
      .dispatch_tree
      .on_action(action_ty, Rc::new(listener));
  }

  pub(crate) fn on_key_event<F, Event>(&mut self, listener: F)
  where
    F: 'static + Fn(&Event, DispatchPhase, &mut Window, &mut App),
    Event: KeyEvent,
  {
    self.current_frame.dispatch_tree.on_key_event(Rc::new(
      move |event, phase, window, cx| {
        if let Some(event) = event.downcast_ref::<Event>() {
          (listener)(event, phase, window, cx);
        };
      },
    ));
  }

  pub(crate) fn listener<E, A, F>(
    &self,
    view: &Entity<E>,
    listener: F,
  ) -> impl 'static + Fn(&A, &mut Self, &mut App)
  where
    E: 'static,
    F: 'static + Fn(&mut E, &A, &mut Self, &mut Context<E>),
  {
    let view = view.clone();
    move |e: &A, window: &mut Self, cx: &mut App| {
      view.update(cx, |view, cx| listener(view, e, window, cx))
    }
  }

  pub(crate) fn focus(&mut self, focus_handle: &FocusHandle) {
    self.focus = Some(focus_handle.id);
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
