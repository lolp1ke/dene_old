// SPDX-License-Identifier: Apache-2.0

use std::{io::Stdout, sync::OnceLock};

use parking_lot::RwLock;
use ratatui::{
  Terminal, crossterm, layout::Rect as RatRect, prelude::CrosstermBackend,
};

use crate::{
  AnyElement, AnyEntity, App, Context, Element, Empty, Entity, EntityId, Frame,
  IntoElement, Keystroke, LayoutEngine, Rect, Window,
};

#[expect(unused_variables, reason = "default noop implementation")]
pub trait Render: 'static + Sized {
  #[deprecated(note = "use _render instead")]
  fn render(
    &mut self,
    frame: &mut ratatui::Frame,
    area: RatRect,
    window: &mut Window,
    cx: &mut Context<Self>,
  ) {
  }

  fn _render(
    &mut self,
    window: &mut Window,
    cx: &mut Context<Self>,
  ) -> impl IntoElement {
    Empty
  }
}
#[expect(unused_variables, reason = "default noop implementation")]
pub trait Interactive: 'static + Sized {
  fn on_keystroke(
    &mut self,
    keystroke: Keystroke,
    window: &mut Window,
    cx: &mut Context<Self>,
  ) {
  }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct AnyView {
  entity: AnyEntity,
  #[deprecated(note = "use _render path")]
  pub render: fn(&Self, &mut ratatui::Frame, RatRect, &mut Window, &mut App),
  pub _render: fn(&Self, &mut Window, &mut App) -> AnyElement,
  pub on_keystroke: fn(&Self, Keystroke, &mut Window, &mut App),
}
impl AnyView {
  pub fn downcast<E>(self) -> Option<Entity<E>>
  where
    E: 'static,
  {
    self.entity.downcast()
  }

  pub fn entity_id(&self) -> EntityId {
    self.entity.entity_id
  }
}
impl<V> From<Entity<V>> for AnyView
where
  V: Render + Interactive,
{
  fn from(value: Entity<V>) -> Self {
    Self {
      entity: value.into(),
      #[expect(deprecated, reason = "will be replaced soon")]
      render: render::<V>,
      _render: _render::<V>,
      on_keystroke: on_keystroke::<V>,
    }
  }
}
impl Element for AnyView {
  fn layout_style(&self) -> taffy::Style {
    taffy::Style {
      display: taffy::Display::Flex,
      size: taffy::Size {
        width: taffy::Dimension::percent(1.0),
        height: taffy::Dimension::percent(1.0),
      },
      ..Default::default()
    }
  }

  fn child_count(&self) -> usize {
    0
  }

  fn render(
    &mut self,
    bounds: Rect,
    frame: &mut Frame,
    window: &mut Window,
    cx: &mut App,
  ) {
    let mut child = (self._render)(self, window, cx);
    let mut engine = LayoutEngine::new();
    let root_id = engine.build_from_root_element(
      &mut child,
      window._bounds.width as f32,
      window._bounds.height as f32,
    );
    engine.compute(root_id, bounds.width as f32, bounds.height as f32);
    render_with_layout(&mut child, root_id, &engine, bounds, frame, window, cx);
  }
}
impl IntoElement for AnyView {
  type Element = Self;

  fn into_element(self) -> Self::Element {
    self
  }
}

fn render<V>(
  any_view: &AnyView,
  frame: &mut ratatui::Frame,
  area: RatRect,
  window: &mut Window,
  cx: &mut App,
) where
  V: 'static + Render,
{
  let view = any_view.clone().downcast::<V>().unwrap().clone();
  view.update(cx, |view, cx| {
    #[expect(deprecated, reason = "will be replaced soon")]
    view.render(frame, area, window, cx);
  });
}
fn _render<V>(
  any_view: &AnyView,
  window: &mut Window,
  cx: &mut App,
) -> AnyElement
where
  V: 'static + Render,
{
  let view = any_view.clone().downcast::<V>().unwrap().clone();
  view.update(cx, |view, cx| view._render(window, cx).into_any_element())
}

fn on_keystroke<V>(
  any_view: &AnyView,
  keystroke: Keystroke,
  window: &mut Window,
  cx: &mut App,
) where
  V: 'static + Interactive,
{
  let view = any_view.clone().downcast::<V>().unwrap();
  view.update(cx, |view, cx| view.on_keystroke(keystroke, window, cx));
}

pub(crate) fn render_element_with_layout(
  root_element: &mut AnyElement,
  layout_engine: &mut LayoutEngine,
  width: u16,
  height: u16,
  frame: &mut Frame,
  window: &mut Window,
  cx: &mut App,
) {
  let root_node = layout_engine.build_from_root_element(
    root_element,
    width as f32,
    height as f32,
  );
  layout_engine.compute(root_node, width as f32, height as f32);
  render_with_layout(
    root_element,
    root_node,
    layout_engine,
    Rect::new(0, 0, width, height),
    frame,
    window,
    cx,
  );
}

fn render_with_layout(
  element: &mut AnyElement,
  node_id: taffy::NodeId,
  engine: &LayoutEngine,
  parent_bounds: Rect,
  frame: &mut Frame,
  window: &mut Window,
  cx: &mut App,
) {
  let layout = engine.layout(node_id);
  let bounds = Rect {
    x: parent_bounds.x + layout.location.x as u16,
    y: parent_bounds.y + layout.location.y as u16,
    width: layout.size.width.ceil() as u16,
    height: layout.size.height.ceil() as u16,
  };

  if element.child_count() == 0 {
    element.render(bounds, frame, window, cx);
  };

  let child_ids = engine.children(node_id);
  for (idx, child_id) in child_ids.into_iter().enumerate() {
    let child = element.get_child(idx);
    render_with_layout(child, child_id, engine, bounds, frame, window, cx);
  }
}

static TERM: OnceLock<RwLock<Terminal<CrosstermBackend<Stdout>>>> =
  OnceLock::new();

#[deprecated(note = "use Terminal")]
pub(crate) fn init_term() {
  let term = ratatui::init();
  crossterm::execute!(
    std::io::stdout(),
    crossterm::event::PushKeyboardEnhancementFlags(
      crossterm::event::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
    )
  )
  .unwrap();

  TERM.set(RwLock::new(term)).unwrap();
}

#[deprecated(note = "use Terminal")]
pub(crate) fn draw<F, R>(f: F) -> R
where
  F: FnOnce(&mut ratatui::Frame) -> R,
{
  let terminal = TERM.get().unwrap();
  let mut terminal = terminal.write();
  let mut result = None;
  terminal
    .draw(|frame| {
      result = Some(f(frame));
    })
    .unwrap();
  result.unwrap()
}
