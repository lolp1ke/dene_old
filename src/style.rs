// SPDX-License-Identifier: Apache-2.0

pub trait Styled: Sized {
  fn style(&mut self) -> &mut taffy::Style;

  fn none(mut self) -> Self {
    self.style().display = taffy::Display::None;
    self
  }
  fn block(mut self) -> Self {
    self.style().display = taffy::Display::Block;
    self
  }
  fn flex(mut self) -> Self {
    self.style().display = taffy::Display::Flex;
    self
  }
  fn grid(mut self) -> Self {
    self.style().display = taffy::Display::Grid;
    self
  }

  fn flex_row(mut self) -> Self {
    self.style().flex_direction = taffy::FlexDirection::Row;
    self
  }
  fn flex_col(mut self) -> Self {
    self.style().flex_direction = taffy::FlexDirection::Column;
    self
  }
  fn flex_row_rev(mut self) -> Self {
    self.style().flex_direction = taffy::FlexDirection::RowReverse;
    self
  }
  fn flex_col_rev(mut self) -> Self {
    self.style().flex_direction = taffy::FlexDirection::ColumnReverse;
    self
  }

  fn items_none(mut self) -> Self {
    self.style().align_items = None;
    self
  }
  fn items_start(mut self) -> Self {
    self.style().align_items = Some(taffy::AlignItems::Start);
    self
  }
  fn items_end(mut self) -> Self {
    self.style().align_items = Some(taffy::AlignItems::End);
    self
  }
  fn items_flex_start(mut self) -> Self {
    self.style().align_items = Some(taffy::AlignItems::FlexStart);
    self
  }
  fn items_flex_end(mut self) -> Self {
    self.style().align_items = Some(taffy::AlignItems::FlexEnd);
    self
  }
  fn items_center(mut self) -> Self {
    self.style().align_items = Some(taffy::AlignItems::Center);
    self
  }
  fn items_baseline(mut self) -> Self {
    self.style().align_items = Some(taffy::AlignItems::Baseline);
    self
  }
  fn items_stretch(mut self) -> Self {
    self.style().align_items = Some(taffy::AlignItems::Stretch);
    self
  }

  fn content_none(mut self) -> Self {
    self.style().align_content = None;
    self
  }
  fn content_start(mut self) -> Self {
    self.style().align_content = Some(taffy::AlignContent::Start);
    self
  }
  fn content_end(mut self) -> Self {
    self.style().align_content = Some(taffy::AlignContent::End);
    self
  }
  fn content_flex_start(mut self) -> Self {
    self.style().align_content = Some(taffy::AlignContent::FlexStart);
    self
  }
  fn content_flex_end(mut self) -> Self {
    self.style().align_content = Some(taffy::AlignContent::FlexEnd);
    self
  }
  fn content_center(mut self) -> Self {
    self.style().align_content = Some(taffy::AlignContent::Center);
    self
  }
  fn content_stretch(mut self) -> Self {
    self.style().align_content = Some(taffy::AlignContent::Stretch);
    self
  }
  fn content_space_between(mut self) -> Self {
    self.style().align_content = Some(taffy::AlignContent::SpaceBetween);
    self
  }
  fn content_space_evenly(mut self) -> Self {
    self.style().align_content = Some(taffy::AlignContent::SpaceEvenly);
    self
  }
  fn content_space_around(mut self) -> Self {
    self.style().align_content = Some(taffy::AlignContent::SpaceAround);
    self
  }

  fn self_none(mut self) -> Self {
    self.style().align_self = None;
    self
  }
  fn self_start(mut self) -> Self {
    self.style().align_self = Some(taffy::AlignSelf::Start);
    self
  }
  fn self_end(mut self) -> Self {
    self.style().align_self = Some(taffy::AlignSelf::End);
    self
  }
  fn self_flex_start(mut self) -> Self {
    self.style().align_self = Some(taffy::AlignSelf::FlexStart);
    self
  }
  fn self_flex_end(mut self) -> Self {
    self.style().align_self = Some(taffy::AlignSelf::FlexEnd);
    self
  }
  fn self_center(mut self) -> Self {
    self.style().align_self = Some(taffy::AlignSelf::Center);
    self
  }
  fn self_baseline(mut self) -> Self {
    self.style().align_self = Some(taffy::AlignSelf::Baseline);
    self
  }
  fn self_stretch(mut self) -> Self {
    self.style().align_self = Some(taffy::AlignSelf::Stretch);
    self
  }

  fn justify_items_none(mut self) -> Self {
    self.style().justify_items = None;
    self
  }
  fn justify_items_start(mut self) -> Self {
    self.style().justify_items = Some(taffy::AlignItems::Start);
    self
  }
  fn justify_items_end(mut self) -> Self {
    self.style().justify_items = Some(taffy::AlignItems::End);
    self
  }
  fn justify_items_flex_start(mut self) -> Self {
    self.style().justify_items = Some(taffy::AlignItems::FlexStart);
    self
  }
  fn justify_items_flex_end(mut self) -> Self {
    self.style().justify_items = Some(taffy::AlignItems::FlexEnd);
    self
  }
  fn justify_items_center(mut self) -> Self {
    self.style().justify_items = Some(taffy::AlignItems::Center);
    self
  }
  fn justify_items_baseline(mut self) -> Self {
    self.style().justify_items = Some(taffy::AlignItems::Baseline);
    self
  }
  fn justify_items_stretch(mut self) -> Self {
    self.style().justify_items = Some(taffy::AlignItems::Stretch);
    self
  }

  fn justify_content_none(mut self) -> Self {
    self.style().justify_content = None;
    self
  }
  fn justify_content_start(mut self) -> Self {
    self.style().justify_content = Some(taffy::AlignContent::Start);
    self
  }
  fn justify_content_end(mut self) -> Self {
    self.style().justify_content = Some(taffy::AlignContent::End);
    self
  }
  fn justify_content_flex_start(mut self) -> Self {
    self.style().justify_content = Some(taffy::AlignContent::FlexStart);
    self
  }
  fn justify_content_flex_end(mut self) -> Self {
    self.style().justify_content = Some(taffy::AlignContent::FlexEnd);
    self
  }
  fn justify_content_center(mut self) -> Self {
    self.style().justify_content = Some(taffy::AlignContent::Center);
    self
  }
  fn justify_content_stretch(mut self) -> Self {
    self.style().justify_content = Some(taffy::AlignContent::Stretch);
    self
  }
  fn justify_content_space_between(mut self) -> Self {
    self.style().justify_content = Some(taffy::AlignContent::SpaceBetween);
    self
  }
  fn justify_content_space_evenly(mut self) -> Self {
    self.style().justify_content = Some(taffy::AlignContent::SpaceEvenly);
    self
  }
  fn justify_content_space_around(mut self) -> Self {
    self.style().justify_content = Some(taffy::AlignContent::SpaceAround);
    self
  }

  fn justify_self_none(mut self) -> Self {
    self.style().justify_self = None;
    self
  }
  fn justify_self_start(mut self) -> Self {
    self.style().justify_self = Some(taffy::AlignSelf::Start);
    self
  }
  fn justify_self_end(mut self) -> Self {
    self.style().justify_self = Some(taffy::AlignSelf::End);
    self
  }
  fn justify_self_flex_start(mut self) -> Self {
    self.style().justify_self = Some(taffy::AlignSelf::FlexStart);
    self
  }
  fn justify_self_flex_end(mut self) -> Self {
    self.style().justify_self = Some(taffy::AlignSelf::FlexEnd);
    self
  }
  fn justify_self_center(mut self) -> Self {
    self.style().justify_self = Some(taffy::AlignSelf::Center);
    self
  }
  fn justify_self_baseline(mut self) -> Self {
    self.style().justify_self = Some(taffy::AlignSelf::Baseline);
    self
  }
  fn justify_self_stretch(mut self) -> Self {
    self.style().justify_self = Some(taffy::AlignSelf::Stretch);
    self
  }

  fn gap(mut self, x: impl Into<f32>, y: impl Into<f32>) -> Self {
    self.style().gap.width = taffy::LengthPercentage::length(x.into());
    self.style().gap.height = taffy::LengthPercentage::length(y.into());
    self
  }
  fn gap_x(mut self, x: impl Into<f32>) -> Self {
    self.style().gap.width = taffy::LengthPercentage::length(x.into());
    self
  }
  fn gap_y(mut self, y: impl Into<f32>) -> Self {
    self.style().gap.height = taffy::LengthPercentage::length(y.into());
    self
  }
}
