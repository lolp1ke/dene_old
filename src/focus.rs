// SPDX-License-Identifier: Apache-2.0

use std::sync::{
  Arc,
  atomic::{self, AtomicUsize},
};

use parking_lot::RwLock;
use slotmap::SlotMap;

use crate::{App, Entity, Window};

slotmap::new_key_type! {
  pub struct FocusId;
}

#[derive(Debug)]
pub(crate) struct FocusRef {
  ref_count: AtomicUsize,
}
pub(crate) type FocusMap = RwLock<SlotMap<FocusId, FocusRef>>;

#[derive(Debug)]
pub struct FocusHandle {
  pub(crate) id: FocusId,
  pub(crate) focus_map: Arc<FocusMap>,
}
impl FocusHandle {
  pub(crate) fn new(focus_map: &Arc<FocusMap>) -> Self {
    let id = focus_map.write().insert(FocusRef {
      ref_count: AtomicUsize::new(1),
    });
    Self {
      id,
      focus_map: focus_map.clone(),
    }
  }

  pub fn focus(&self, window: &mut Window) {
    window.focus(self);
  }
}
impl Clone for FocusHandle {
  fn clone(&self) -> Self {
    let lock = self.focus_map.read();
    let focus_ref = lock.get(self.id).unwrap();
    let mut loaded = focus_ref.ref_count.load(atomic::Ordering::SeqCst);
    if loop {
      if loaded == 0 {
        break 0;
      };

      match focus_ref.ref_count.compare_exchange_weak(
        loaded,
        loaded + 1,
        atomic::Ordering::SeqCst,
        atomic::Ordering::SeqCst,
      ) {
        Ok(x) => break x + 1,
        Err(actual) => loaded = actual,
      }
    } == 0
    {
      panic!();
    };

    Self {
      id: self.id,
      focus_map: self.focus_map.clone(),
    }
  }
}

pub trait Focusable: 'static {
  fn focus_handle(&self, cx: &App) -> FocusHandle;
}
impl<V> Focusable for Entity<V>
where
  V: Focusable,
{
  fn focus_handle(&self, cx: &App) -> FocusHandle {
    self.read(cx).focus_handle(cx)
  }
}
