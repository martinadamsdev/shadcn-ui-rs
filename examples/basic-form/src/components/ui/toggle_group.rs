//! ToggleGroup component for shadcn-ui-rs
//!
//! A set of two-state buttons that can be toggled on or off.
//! Supports single selection (radio-like) and multiple selection modes.
//!
//! # Example
//!
//! ```rust
//! ToggleGroup::new("alignment")
//!     .type_(ToggleGroupType::Single)
//!     .value(vec!["left".into()])
//!     .on_change(|values, window, cx| {
//!         println!("Selected: {:?}", values);
//!     })
//!     .child(
//!         ToggleGroupItem::new("left", "left")
//!             .child("Left")
//!     )
//!     .child(
//!         ToggleGroupItem::new("center", "center")
//!             .child("Center")
//!     )
//!     .child(
//!         ToggleGroupItem::new("right", "right")
//!             .child("Right")
//!     )
//! ```

use crate::theme::Theme;
use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, Hsla, IntoElement, ParentElement,
    RenderOnce, SharedString, Styled, Window, div, prelude::*,
};

/// Selection mode for the toggle group
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToggleGroupType {
    /// Only one item can be selected at a time
    #[default]
    Single,
    /// Multiple items can be selected simultaneously
    Multiple,
}

/// Toggle group variant (applied to all items)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToggleGroupVariant {
    /// Default variant
    #[default]
    Default,
    /// Outline variant with border
    Outline,
}

/// Toggle group size (applied to all items)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToggleGroupSize {
    /// Small
    Sm,
    /// Default
    #[default]
    Default,
    /// Large
    Lg,
}

/// A container for toggle group items with single or multiple selection.
#[derive(IntoElement)]
pub struct ToggleGroup {
    id: ElementId,
    type_: ToggleGroupType,
    variant: ToggleGroupVariant,
    size: ToggleGroupSize,
    value: Vec<SharedString>,
    disabled: bool,
    #[allow(clippy::type_complexity)]
    on_change: Option<Box<dyn Fn(Vec<SharedString>, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl ToggleGroup {
    /// Create a new toggle group with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            type_: ToggleGroupType::Single,
            variant: ToggleGroupVariant::Default,
            size: ToggleGroupSize::Default,
            value: Vec::new(),
            disabled: false,
            on_change: None,
            children: Vec::new(),
        }
    }

    /// Set the selection type (single or multiple).
    pub fn type_(mut self, type_: ToggleGroupType) -> Self {
        self.type_ = type_;
        self
    }

    /// Set the visual variant for all items.
    pub fn variant(mut self, variant: ToggleGroupVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the size for all items.
    pub fn size(mut self, size: ToggleGroupSize) -> Self {
        self.size = size;
        self
    }

    /// Set the currently selected values.
    pub fn value(mut self, value: Vec<SharedString>) -> Self {
        self.value = value;
        self
    }

    /// Set the disabled state for the entire group.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the callback for when the selection changes.
    pub fn on_change(
        mut self,
        handler: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl ParentElement for ToggleGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ToggleGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .gap(gpui::px(4.0))
            .children(self.children)
    }
}

/// An individual item within a ToggleGroup.
///
/// Each item has a value and renders like a Toggle button.
#[derive(IntoElement)]
pub struct ToggleGroupItem {
    id: ElementId,
    value: SharedString,
    pressed: bool,
    variant: ToggleGroupVariant,
    size: ToggleGroupSize,
    disabled: bool,
    #[allow(clippy::type_complexity)]
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl ToggleGroupItem {
    /// Create a new toggle group item with an ID and value.
    pub fn new(id: impl Into<ElementId>, value: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
            pressed: false,
            variant: ToggleGroupVariant::Default,
            size: ToggleGroupSize::Default,
            disabled: false,
            on_click: None,
            children: Vec::new(),
        }
    }

    /// Set the pressed (selected) state.
    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    /// Set the visual variant.
    pub fn variant(mut self, variant: ToggleGroupVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the size.
    pub fn size(mut self, size: ToggleGroupSize) -> Self {
        self.size = size;
        self
    }

    /// Set the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the click handler.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Get the value of this item.
    pub fn value(&self) -> &SharedString {
        &self.value
    }
}

impl ParentElement for ToggleGroupItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ToggleGroupItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        // Determine colors based on pressed state
        let (bg_color, text_color): (Hsla, Hsla) = if self.pressed {
            (colors.accent, colors.accent_foreground)
        } else {
            (gpui::transparent_black(), colors.foreground)
        };

        let mut el = div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .cursor_pointer()
            .bg(bg_color)
            .text_color(text_color);

        // Apply size
        match self.size {
            ToggleGroupSize::Sm => {
                el = el
                    .h(gpui::px(36.0))
                    .px(gpui::px(10.0))
                    .min_w(gpui::px(36.0));
            }
            ToggleGroupSize::Default => {
                el = el
                    .h(gpui::px(40.0))
                    .px(gpui::px(12.0))
                    .min_w(gpui::px(40.0));
            }
            ToggleGroupSize::Lg => {
                el = el
                    .h(gpui::px(44.0))
                    .px(gpui::px(20.0))
                    .min_w(gpui::px(44.0));
            }
        }

        // Apply variant
        match self.variant {
            ToggleGroupVariant::Default => {}
            ToggleGroupVariant::Outline => {
                el = el.border_1().border_color(colors.input);
                if self.pressed {
                    el = el.bg(colors.accent);
                }
            }
        }

        // Disabled state
        if self.disabled {
            el = el.opacity(0.5).cursor_default();
        }

        // Hover effect
        let hover_bg = colors.muted;
        let accent_bg = colors.accent;
        let pressed = self.pressed;
        if !self.disabled {
            el = el.hover(move |style| {
                if pressed {
                    style.bg(accent_bg)
                } else {
                    style.bg(hover_bg)
                }
            });
        }

        // Click handler
        if !self.disabled
            && let Some(on_click) = self.on_click
        {
            el = el.on_click(
                move |event: &ClickEvent, window: &mut Window, cx: &mut App| {
                    on_click(event, window, cx);
                },
            );
        }

        el.children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_group_builder() {
        let group = ToggleGroup::new("test")
            .type_(ToggleGroupType::Multiple)
            .variant(ToggleGroupVariant::Outline)
            .size(ToggleGroupSize::Lg)
            .value(vec!["a".into(), "b".into()])
            .disabled(false);

        assert_eq!(group.type_, ToggleGroupType::Multiple);
        assert_eq!(group.variant, ToggleGroupVariant::Outline);
        assert_eq!(group.size, ToggleGroupSize::Lg);
        assert_eq!(group.value.len(), 2);
        assert!(!group.disabled);
    }

    #[test]
    fn test_toggle_group_item_builder() {
        let item = ToggleGroupItem::new("test-item", "value1")
            .pressed(true)
            .variant(ToggleGroupVariant::Outline)
            .size(ToggleGroupSize::Sm)
            .disabled(false);

        assert!(item.pressed);
        assert_eq!(item.value, SharedString::from("value1"));
        assert_eq!(item.variant, ToggleGroupVariant::Outline);
        assert_eq!(item.size, ToggleGroupSize::Sm);
        assert!(!item.disabled);
    }
}
