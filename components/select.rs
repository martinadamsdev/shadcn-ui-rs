//! Select (dropdown) component for shadcn-ui-rs
//!
//! A dropdown menu for selecting a single value from a list of options.
//!
//! # Example
//! ```rust
//! Select::new("fruit-select")
//!     .placeholder("Pick a fruit...")
//!     .value("apple")
//!     .open(is_open)
//!     .on_open_change(|open, _window, _cx| {
//!         // Toggle dropdown visibility
//!     })
//!     .on_change(|value, _window, _cx| {
//!         println!("Selected: {value}");
//!     })
//!     .child(SelectItem::new("apple", "Apple"))
//!     .child(SelectItem::new("banana", "Banana"))
//!     .child(SelectItem::new("cherry", "Cherry"))
//! ```

use std::rc::Rc;

use gpui::prelude::*;
use gpui::{
    px, App, ClickEvent, ElementId, IntoElement, MouseDownEvent, SharedString, Window,
};

use crate::theme::Theme;

// ---------------------------------------------------------------------------
// SelectItem (data only)
// ---------------------------------------------------------------------------

/// Describes a single option within a [`Select`] dropdown.
#[derive(Clone)]
pub struct SelectItem {
    value: SharedString,
    label: SharedString,
    disabled: bool,
}

impl SelectItem {
    /// Create a new select item with the given value and display label.
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    /// Mark this item as disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

// ---------------------------------------------------------------------------
// Select
// ---------------------------------------------------------------------------

/// A dropdown select component.
///
/// This is a **controlled** component. Both the selected `value` and the `open`
/// state are passed in as props; changes are communicated through callbacks.
#[derive(IntoElement)]
pub struct Select {
    id: ElementId,
    value: Option<SharedString>,
    placeholder: SharedString,
    open: bool,
    disabled: bool,
    items: Vec<SelectItem>,
    on_change: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_open_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl Select {
    /// Create a new select with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: None,
            placeholder: "Select...".into(),
            open: false,
            disabled: false,
            items: Vec::new(),
            on_change: None,
            on_open_change: None,
        }
    }

    /// Set the currently selected value.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set the placeholder text shown when no value is selected.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Control whether the dropdown is open.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Disable the select so it cannot be opened.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Add a single item to the dropdown.
    pub fn child(mut self, item: SelectItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add multiple items to the dropdown.
    pub fn children(mut self, items: impl IntoIterator<Item = SelectItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// Register a callback for when the selected value changes.
    pub fn on_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Register a callback for when the dropdown open state changes.
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }

    /// Find the label for the currently selected value.
    fn selected_label(&self) -> Option<&SharedString> {
        let val = self.value.as_ref()?;
        self.items
            .iter()
            .find(|item| item.value.as_ref() == val.as_ref())
            .map(|item| &item.label)
    }
}

impl RenderOnce for Select {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;
        let radius = theme.radius.to_px();

        let bg = colors.background;
        let fg = colors.foreground;
        let border = colors.border;
        let popover_bg = colors.popover;
        let popover_fg = colors.popover_foreground;
        let muted_fg = colors.muted_foreground;
        let accent = colors.accent;
        let accent_fg = colors.accent_foreground;

        // Resolve display text before destructuring
        let display_text = self
            .selected_label()
            .cloned()
            .unwrap_or_else(|| self.placeholder.clone());
        let has_value = self.value.is_some();

        // Destructure self so all fields are moved cleanly
        let id = self.id;
        let value = self.value;
        let open = self.open;
        let disabled = self.disabled;
        let items = self.items;
        let on_change = self.on_change;
        let on_open_change = self.on_open_change;

        div()
            .id(id)
            .relative()
            .child(
                // Trigger button
                div()
                    .id("select-trigger")
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .h(px(36.0))
                    .px(px(12.0))
                    .rounded(px(radius))
                    .border_1()
                    .border_color(border)
                    .bg(bg)
                    .text_sm()
                    .text_color(if has_value { fg } else { muted_fg })
                    .when(!disabled, |el| el.cursor_pointer())
                    .when(disabled, |el| el.opacity(0.5))
                    .when(!disabled, |el| {
                        let on_open_change = on_open_change.clone();
                        el.on_click(move |_event: &ClickEvent, window, cx| {
                            if let Some(ref handler) = on_open_change {
                                handler(!open, window, cx);
                            }
                        })
                    })
                    .child(display_text)
                    // Chevron indicator
                    .child(
                        div()
                            .text_xs()
                            .text_color(muted_fg)
                            .child(if open { "\u{25B2}" } else { "\u{25BC}" }),
                    ),
            )
            // Dropdown popover (only visible when open)
            .when(open, |el| {
                el.child(
                    div()
                        .id("select-popover")
                        .absolute()
                        .top(px(40.0))
                        .left_0()
                        .w_full()
                        .min_w(px(160.0))
                        .rounded(px(radius))
                        .border_1()
                        .border_color(border)
                        .bg(popover_bg)
                        .py(px(4.0))
                        .overflow_y_scroll()
                        // Close when clicking outside
                        .on_mouse_down_out({
                            let on_open_change = on_open_change.clone();
                            move |_event: &MouseDownEvent, window, cx| {
                                if let Some(ref handler) = on_open_change {
                                    handler(false, window, cx);
                                }
                            }
                        })
                        .children(items.into_iter().enumerate().map(move |(i, item)| {
                            let is_selected = value
                                .as_ref()
                                .is_some_and(|v| v.as_ref() == item.value.as_ref());
                            let is_disabled = item.disabled;
                            let on_change = on_change.clone();
                            let on_open_change = on_open_change.clone();
                            let item_value = item.value.clone();

                            div()
                                .id(("select-item", i))
                                .flex()
                                .flex_row()
                                .items_center()
                                .px(px(12.0))
                                .py(px(6.0))
                                .text_sm()
                                .text_color(if is_disabled { muted_fg } else { popover_fg })
                                .when(is_selected, |el| el.bg(accent).text_color(accent_fg))
                                .when(!is_disabled, |el| {
                                    el.cursor_pointer().hover(|style| style.bg(accent))
                                })
                                .when(is_disabled, |el| el.opacity(0.5))
                                .when(!is_disabled, |el| {
                                    el.on_click({
                                        let val = item_value.clone();
                                        let on_change = on_change.clone();
                                        let on_open_change = on_open_change.clone();
                                        move |_event: &ClickEvent, window, cx| {
                                            if let Some(ref handler) = on_change {
                                                handler(val.as_ref(), window, cx);
                                            }
                                            // Close dropdown after selection
                                            if let Some(ref handler) = on_open_change {
                                                handler(false, window, cx);
                                            }
                                        }
                                    })
                                })
                                .child(item.label.clone())
                        })),
                )
            })
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_item_creation() {
        let item = SelectItem::new("val", "Label");
        assert_eq!(item.value.as_ref(), "val");
        assert_eq!(item.label.as_ref(), "Label");
        assert!(!item.disabled);
    }

    #[test]
    fn test_select_item_disabled() {
        let item = SelectItem::new("val", "Label").disabled(true);
        assert!(item.disabled);
    }

    #[test]
    fn test_select_defaults() {
        let select = Select::new("test");
        assert!(select.value.is_none());
        assert_eq!(select.placeholder.as_ref(), "Select...");
        assert!(!select.open);
        assert!(!select.disabled);
        assert!(select.items.is_empty());
    }

    #[test]
    fn test_select_builder() {
        let select = Select::new("test")
            .placeholder("Choose...")
            .value("b")
            .open(true)
            .disabled(false)
            .child(SelectItem::new("a", "Alpha"))
            .child(SelectItem::new("b", "Beta"))
            .child(SelectItem::new("c", "Gamma"));

        assert_eq!(select.value.as_ref().unwrap().as_ref(), "b");
        assert_eq!(select.placeholder.as_ref(), "Choose...");
        assert!(select.open);
        assert_eq!(select.items.len(), 3);
    }

    #[test]
    fn test_select_selected_label() {
        let select = Select::new("test")
            .value("b")
            .child(SelectItem::new("a", "Alpha"))
            .child(SelectItem::new("b", "Beta"));

        assert_eq!(select.selected_label().unwrap().as_ref(), "Beta");
    }

    #[test]
    fn test_select_selected_label_none() {
        let select = Select::new("test")
            .child(SelectItem::new("a", "Alpha"));

        assert!(select.selected_label().is_none());
    }

    #[test]
    fn test_select_children() {
        let items = vec![
            SelectItem::new("a", "A"),
            SelectItem::new("b", "B"),
            SelectItem::new("c", "C"),
        ];
        let select = Select::new("test").children(items);
        assert_eq!(select.items.len(), 3);
    }
}
