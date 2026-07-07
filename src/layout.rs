// SPDX-License-Identifier: Apache-2.0

use taffy::{AvailableSpace, Layout, NodeId, Size, TaffyTree};

use crate::{AnyElement, Element};

#[derive(Debug)]
pub struct LayoutEngine {
  taffy: TaffyTree<()>,
}
impl LayoutEngine {
  pub fn new() -> Self {
    Self {
      taffy: TaffyTree::new(),
    }
  }

  pub fn layout(&self, node_id: NodeId) -> &Layout {
    self.taffy.layout(node_id).unwrap()
  }
  pub fn children(&self, node_id: NodeId) -> Vec<NodeId> {
    self.taffy.children(node_id).unwrap()
  }

  pub fn compute(&mut self, root_id: NodeId, width: f32, height: f32) {
    self
      .taffy
      .compute_layout(
        root_id,
        Size {
          width: AvailableSpace::Definite(width),
          height: AvailableSpace::Definite(height),
        },
      )
      .unwrap();
  }

  pub fn clear(&mut self) {
    self.taffy.clear();
  }

  pub(crate) fn build_from_root_element(
    &mut self,
    element: &mut AnyElement,
    width: f32,
    height: f32,
  ) -> NodeId {
    self.taffy.clear();
    let root_id = self.build_element_node(element);
    let mut style = self.taffy.style(root_id).unwrap().clone();
    style.size = taffy::Size::from_lengths(width, height);
    self.taffy.set_style(root_id, style).unwrap();
    root_id
  }

  fn build_element_node(&mut self, element: &mut AnyElement) -> NodeId {
    let style = element.layout_style();
    let count = element.child_count();

    let Some(count) = count else {
      return self.taffy.new_leaf(style).unwrap();
    };

    let child_ids = (0..count)
      .map(|idx| {
        let child = element.get_child(idx);
        self.build_element_node(child)
      })
      .collect::<Vec<_>>();
    self.taffy.new_with_children(style, &child_ids).unwrap()
  }
}
impl Default for LayoutEngine {
  fn default() -> Self {
    Self::new()
  }
}
