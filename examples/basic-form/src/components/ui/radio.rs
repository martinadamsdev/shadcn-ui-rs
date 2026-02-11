//! Radio group component for shadcn-ui-rs
//!
//! A set of mutually exclusive radio buttons where only one can be selected at a time.
//!
//! # Example
//! ```rust
//! RadioGroup::new("size-group")
//!     .value("medium")
//!     .on_change(|value, _window, _cx| {
//!         println!("Selected: {value}");
//!     })
//!     .child(RadioItem::new("small", "Small"))
//!     .child(RadioItem::new("medium", "Medium"))
//!     .child(RadioItem::new("large", "Large"))
//! ```

use std::rc::Rc;

use gpui::prelude::*;
use gpui::{App, ClickEvent, ElementId, IntoElement, SharedString, Window, div, px};

use crate::theme::Theme;

// ---------------------------------------------------------------------------
// RadioItem (data only)
// ---------------------------------------------------------------------------

/// Describes a single option within a [`RadioGroup`].
#[derive(Clone)]
pub struct RadioItem {
    value: SharedString,
    label: SharedString,
    disabled: bool,
}

impl RadioItem {
    /// Create a new radio item with the given value and display label.
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    /// Mark this item as disabled so it cannot be selected.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

// ---------------------------------------------------------------------------
// RadioGroup
// ---------------------------------------------------------------------------

/// A group of mutually exclusive radio buttons.
///
/// The currently selected value is controlled via `.value()` (controlled component).
/// Selection changes are reported through the `.on_change()` callback.
#[derive(IntoElement)]
pub struct RadioGroup {
    id: ElementId,
    value: Option<SharedString>,
    #[allow(clippy::type_complexity)]
    on_change: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    items: Vec<RadioItem>,
    disabled: bool,
}

impl RadioGroup {
    /// Create a new radio group with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: None,
            on_change: None,
            items: Vec::new(),
            disabled: false,
        }
    }

    /// Set the currently selected value.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Register a callback invoked when the user selects a different option.
    /// The callback receives the value string of the newly selected item.
    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Add a single radio item to this group.
    pub fn child(mut self, item: RadioItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add multiple radio items at once.
    pub fn children(mut self, items: impl IntoIterator<Item = RadioItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// Disable the entire radio group.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for RadioGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let primary = theme.colors.primary;
        let border = theme.colors.border;
        let foreground = theme.colors.foreground;
        let muted_fg = theme.colors.muted_foreground;

        let selected = self.value;
        let on_change = self.on_change;
        let group_disabled = self.disabled;

        div().id(self.id).flex().flex_col().gap(px(8.0)).children(
            self.items.into_iter().enumerate().map(move |(i, item)| {
                let is_selected = selected
                    .as_ref()
                    .is_some_and(|v| v.as_ref() == item.value.as_ref());
                let is_disabled = group_disabled || item.disabled;
                let on_change = on_change.clone();
                let item_value = item.value.clone();

                div()
                    .id(("radio-item", i))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .when(!is_disabled, |el| el.cursor_pointer())
                    .when(is_disabled, |el| el.opacity(0.5))
                    .when(!is_disabled, |el| {
                        el.on_click({
                            let val = item_value.clone();
                            move |_event: &ClickEvent, window, cx| {
                                if let Some(ref handler) = on_change {
                                    handler(val.as_ref(), window, cx);
                                }
                            }
                        })
                    })
                    .child(
                        // Outer circle
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .size(px(16.0))
                            .rounded(px(9999.0))
                            .border_1()
                            .border_color(if is_selected { primary } else { border })
                            .when(is_selected, |el| {
                                el.child(
                                    // Inner filled dot
                                    div().size(px(8.0)).rounded(px(9999.0)).bg(primary),
                                )
                            }),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(if is_disabled { muted_fg } else { foreground })
                            .child(item.label.clone()),
                    )
            }),
        )
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_item_creation() {
        let item = RadioItem::new("opt1", "Option 1");
        assert_eq!(item.value.as_ref(), "opt1");
        assert_eq!(item.label.as_ref(), "Option 1");
        assert!(!item.disabled);
    }

    #[test]
    fn test_radio_item_disabled() {
        let item = RadioItem::new("opt1", "Option 1").disabled(true);
        assert!(item.disabled);
    }

    #[test]
    fn test_radio_group_builder() {
        let group = RadioGroup::new("test-group")
            .value("opt2")
            .child(RadioItem::new("opt1", "Option 1"))
            .child(RadioItem::new("opt2", "Option 2"))
            .child(RadioItem::new("opt3", "Option 3"));

        assert_eq!(group.items.len(), 3);
        assert_eq!(group.value.as_ref().unwrap().as_ref(), "opt2");
    }

    #[test]
    fn test_radio_group_disabled() {
        let group = RadioGroup::new("test-group").disabled(true);
        assert!(group.disabled);
    }

    #[test]
    fn test_radio_group_children() {
        let items = vec![RadioItem::new("a", "A"), RadioItem::new("b", "B")];
        let group = RadioGroup::new("g").children(items);
        assert_eq!(group.items.len(), 2);
    }
}
