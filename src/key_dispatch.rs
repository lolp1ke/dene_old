// SPDX-License-Identifier: Apache-2.0

use std::{
  any::{Any, TypeId},
  cell::RefCell,
  rc::Rc,
};

use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::{
  ActionRegistry, App, FocusId, KeyContext, Keybinds, Keystroke, Window,
};

type KeyListener = Rc<dyn Fn(&dyn Any, DispatchPhase, &mut Window, &mut App)>;
type ActionListener =
  Rc<dyn Fn(&dyn Any, DispatchPhase, &mut Window, &mut App)>;

#[derive(Debug)]
pub(crate) struct DispatchTree {
  pub(crate) node_stack: Vec<DispatchNodeId>,
  pub(crate) context_stack: Vec<KeyContext>,
  pub(crate) nodes: Vec<DispatchNode>,
  pub(crate) focusable_node_ids: FxHashMap<FocusId, DispatchNodeId>,
  pub(crate) keybinds: Rc<RefCell<Keybinds>>,
  pub(crate) actions: Rc<ActionRegistry>,
}
impl DispatchTree {
  pub(crate) fn new(
    keybinds: Rc<RefCell<Keybinds>>,
    actions: Rc<ActionRegistry>,
  ) -> Self {
    Self {
      node_stack: vec![DispatchNodeId(0)],
      // node_stack: Vec::new(),
      context_stack: Vec::new(),
      nodes: vec![Default::default()],
      // nodes: Vec::new(),
      focusable_node_ids: FxHashMap::default(),
      keybinds,
      actions,
    }
  }

  pub(crate) fn on_action(
    &mut self,
    action_ty: TypeId,
    listener: ActionListener,
  ) {
    self
      .active_node()
      .action_listeners
      .insert(action_ty, listener);
  }
  pub(crate) fn on_key_event(&mut self, listener: KeyListener) {
    self.active_node().key_listeners.push(listener);
  }

  pub(crate) fn set_key_context(&mut self, context: KeyContext) {
    let had_context = self.active_node().context.is_some();
    if had_context {
      self.context_stack.pop();
    }
    self.active_node().context = Some(context.clone());
    self.context_stack.push(context);
  }

  pub(crate) fn push_node(&mut self) -> DispatchNodeId {
    let parent = self.node_stack.last().copied();
    let node_id = DispatchNodeId(self.nodes.len());
    self.nodes.push(DispatchNode {
      parent,
      ..Default::default()
    });
    self.node_stack.push(node_id);
    node_id
  }
  pub(crate) fn pop_node(&mut self) {
    let node = &self.nodes[self.active_node_id().unwrap().0];
    if node.context.is_some() {
      self.context_stack.pop();
    };
    self.node_stack.pop();
  }

  pub(crate) fn dispatch_path(
    &self,
    target: DispatchNodeId,
  ) -> SmallVec<[DispatchNodeId; 32]> {
    let mut path = SmallVec::new();
    let mut current = Some(target);
    while let Some(node_id) = current {
      path.push(node_id);
      current = self.nodes[node_id.0].parent;
    }

    path.reverse();
    path
  }

  pub(crate) fn focusable_node_id(
    &self,
    focus_id: FocusId,
  ) -> Option<DispatchNodeId> {
    self.focusable_node_ids.get(&focus_id).copied()
  }
  pub(crate) fn root_node_id(&self) -> DispatchNodeId {
    DispatchNodeId(0)
  }
  pub(crate) fn node(&self, id: DispatchNodeId) -> &DispatchNode {
    &self.nodes[id.0]
  }
  pub(crate) fn active_node(&mut self) -> &mut DispatchNode {
    let active_node_id = self.active_node_id().unwrap();
    &mut self.nodes[active_node_id.0]
  }
  pub(crate) fn active_node_id(&self) -> Option<DispatchNodeId> {
    self.node_stack.last().copied()
  }

  pub(crate) fn bindings_for_input(
    &self,
    keystroke: &Keystroke,
    path: &[DispatchNodeId],
  ) -> SmallVec<[(usize, usize); 4]> {
    let contexts = path
      .iter()
      .filter_map(|id| self.nodes[id.0].context.clone())
      .collect::<Vec<_>>();

    let mut matched = SmallVec::new();
    // let keybinds = self.keybinds.borrow();
    // for (idx, keybind) in keybinds.iter().enumerate() {
    //   let first_keystroke = match keybind.keystrokes.first() {
    //     Some(k) => k,
    //     None => continue,
    //   };
    //   if *first_keystroke != *keystroke {
    //     continue;
    //   }
    //   match &keybind.key_context {
    //     None => {
    //       matched.push((idx, 0));
    //     }
    //     Some(ctx) => {
    //       if let Some(depth) = ctx.depth_of(&contexts) {
    //         matched.push((idx, depth));
    //       }
    //     }
    //   }
    // }
    matched
  }

  pub(crate) fn dispatch_key_event_to_all(
    &self,
    event: &dyn Any,
    phase: DispatchPhase,
    window: &mut Window,
    cx: &mut App,
  ) {
    let listeners = self
      .nodes
      .iter()
      .flat_map(|node| node.key_listeners.iter().cloned())
      .collect::<Vec<_>>();
    for listener in listeners {
      listener(event, phase, window, cx);
    }
  }

  pub(crate) fn all_action_listeners_for(
    &self,
    action_ty: TypeId,
  ) -> Vec<ActionListener> {
    self
      .nodes
      .iter()
      .filter_map(|node| node.action_listeners.get(&action_ty).cloned())
      .collect()
  }

  pub(crate) fn match_bindings_for_all_nodes(
    &self,
    keystroke: &Keystroke,
  ) -> SmallVec<[(usize, usize, DispatchNodeId); 4]> {
    let mut matched = SmallVec::new();
    let keybinds = self.keybinds.borrow();

    for (node_idx, _node) in self.nodes.iter().enumerate() {
      let node_id = DispatchNodeId(node_idx);
      let path = self.dispatch_path(node_id);
      let contexts = path
        .iter()
        .filter_map(|id| self.nodes[id.0].context.clone())
        .collect::<Vec<_>>();

      //   for (bind_idx, keybind) in keybinds.iter().enumerate() {
      //     let first_keystroke = match keybind.keystrokes.first() {
      //       Some(k) => k,
      //       None => continue,
      //     };
      //     if *first_keystroke != *keystroke {
      //       continue;
      //     }
      //     match &keybind.key_context {
      //       None => {
      //         matched.push((bind_idx, 0, node_id));
      //       }
      //       Some(ctx) => {
      //         if let Some(depth) = ctx.depth_of(&contexts) {
      //           matched.push((bind_idx, depth, node_id));
      //         }
      //       }
      //     }
      //   }
    }
    matched
  }
}

#[derive(derive_more::Debug)]
#[derive(Default)]
pub(crate) struct DispatchNode {
  #[debug(skip)]
  pub(crate) key_listeners: Vec<KeyListener>,
  #[debug(skip)]
  pub(crate) action_listeners: FxHashMap<TypeId, ActionListener>,
  pub(crate) focus_id: Option<FocusId>,
  pub(crate) context: Option<KeyContext>,
  parent: Option<DispatchNodeId>,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum DispatchPhase {
  Capture,
  Bubble,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub(crate) struct DispatchNodeId(pub(crate) usize);
