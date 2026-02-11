//! DropdownMenu component for shadcn-ui-rs
//!
//! A click-triggered menu with items, separators, and labels.
//!
//! # Example
//! ```rust
//! DropdownMenu::new("my-menu")
//!     .open(is_open)
//!     .on_open_change(|open, _window, _cx| {
//!         // Toggle dropdown visibility
//!     })
//!     .on_select(|value, _window, _cx| {
//!         println!("Selected: {value}");
//!     })
//!     .trigger(Button::new("Actions"))
//!     .label("Appearance")
//!     .item(DropdownMenuItem::new("light", "Light"))
//!     .item(DropdownMenuItem::new("dark", "Dark"))
//!     .separator()
//!     .item(DropdownMenuItem::new("delete", "Delete").destructive(true))
//! ```

use std::rc::Rc;

use gpui::prelude::*;
use gpui::{
    deferred, div, px, AnyElement, App, ClickEvent, Div, ElementId, FontWeight, IntoElement,
    KeyDownEvent, MouseDownEvent, ParentElement, RenderOnce, SharedString, Stateful, Styled,
    Window,
};

use crate::theme::Theme;

// ---------------------------------------------------------------------------
// DropdownMenuItem (data only)
// ---------------------------------------------------------------------------

/// Describes a single actionable item within a [`DropdownMenu`].
#[derive(Clone)]
pub struct DropdownMenuItem {
    label: SharedString,
    value: SharedString,
    disabled: bool,
    destructive: bool,
}

impl DropdownMenuItem {
    /// Create a new menu item with the given value and display label.
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
            destructive: false,
        }
    }

    /// Mark this item as disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Mark this item as destructive (renders with destructive color).
    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuSeparator
// ---------------------------------------------------------------------------

/// A visual separator line between menu items.
pub struct DropdownMenuSeparator;

// ---------------------------------------------------------------------------
// DropdownMenuLabel
// ---------------------------------------------------------------------------

/// A non-interactive text header for grouping menu items.
pub struct DropdownMenuLabel {
    text: SharedString,
}

impl DropdownMenuLabel {
    /// Create a new menu label.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

// ---------------------------------------------------------------------------
// DropdownMenuEntry
// ---------------------------------------------------------------------------

/// An entry in the dropdown menu: an item, separator, or label.
pub enum DropdownMenuEntry {
    Item(DropdownMenuItem),
    Separator,
    Label(SharedString),
}

// ---------------------------------------------------------------------------
// DropdownMenu
// ---------------------------------------------------------------------------

/// A click-triggered dropdown menu component.
///
/// This is a **controlled** component. The `open` state is passed in as a prop;
/// changes are communicated through the `on_open_change` callback.
#[derive(IntoElement)]
pub struct DropdownMenu {
    id: ElementId,
    open: bool,
    #[allow(clippy::type_complexity)]
    on_open_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
    #[allow(clippy::type_complexity)]
    on_select: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    entries: Vec<DropdownMenuEntry>,
    trigger: Vec<AnyElement>,
}

impl DropdownMenu {
    /// Create a new dropdown menu with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            on_open_change: None,
            on_select: None,
            entries: Vec::new(),
            trigger: Vec::new(),
        }
    }

    /// Set the open state of the dropdown.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
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

    /// Register a callback for when an item is selected.
    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    /// Add a trigger element that opens/closes the dropdown when clicked.
    pub fn trigger(mut self, element: impl IntoElement) -> Self {
        self.trigger.push(element.into_any_element());
        self
    }

    /// Add a menu item entry.
    pub fn item(mut self, item: DropdownMenuItem) -> Self {
        self.entries.push(DropdownMenuEntry::Item(item));
        self
    }

    /// Add a separator entry.
    pub fn separator(mut self) -> Self {
        self.entries.push(DropdownMenuEntry::Separator);
        self
    }

    /// Add a label entry.
    pub fn label(mut self, text: impl Into<SharedString>) -> Self {
        self.entries.push(DropdownMenuEntry::Label(text.into()));
        self
    }
}

impl RenderOnce for DropdownMenu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;
        let radius = theme.radius.to_px();

        let border = colors.border;
        let popover_bg = colors.popover;
        let popover_fg = colors.popover_foreground;
        let accent = colors.accent;
        let accent_fg = colors.accent_foreground;
        let muted_foreground = colors.muted_foreground;
        let destructive = colors.destructive;

        let open = self.open;
        let on_open_change = self.on_open_change;
        let on_select = self.on_select;

        div()
            .id(self.id)
            .relative()
            // Trigger
            .child(
                div()
                    .id("dropdown-trigger")
                    .cursor_pointer()
                    .on_click({
                        let on_open_change = on_open_change.clone();
                        move |_event: &ClickEvent, window, cx| {
                            if let Some(ref handler) = on_open_change {
                                handler(!open, window, cx);
                            }
                        }
                    })
                    .children(self.trigger),
            )
            // Dropdown popover (only visible when open)
            .when(open, |el| {
                el.child(
                    deferred(
                        div()
                            .id("dropdown-popover")
                            .absolute()
                            .top(px(40.0))
                            .left_0()
                            .min_w(px(160.0))
                            .rounded(px(radius))
                            .border_1()
                            .border_color(border)
                            .bg(popover_bg)
                            .py(px(4.0))
                            // Close when clicking outside
                            .on_mouse_down_out({
                                let on_open_change = on_open_change.clone();
                                move |_event: &MouseDownEvent, window, cx| {
                                    if let Some(ref handler) = on_open_change {
                                        handler(false, window, cx);
                                    }
                                }
                            })
                            // Close on Escape key
                            .on_key_down({
                                let on_open_change = on_open_change.clone();
                                move |event: &KeyDownEvent, window, cx| {
                                    if event.keystroke.key.as_str() == "escape" {
                                        if let Some(ref handler) = on_open_change {
                                            handler(false, window, cx);
                                        }
                                    }
                                }
                            })
                            .children(
                                self.entries
                                    .into_iter()
                                    .enumerate()
                                    .map(|(i, entry)| match entry {
                                        DropdownMenuEntry::Item(item) => {
                                            let is_disabled = item.disabled;
                                            let is_destructive = item.destructive;
                                            let on_select = on_select.clone();
                                            let on_open_change = on_open_change.clone();
                                            let item_value = item.value.clone();

                                            div()
                                                .id(("dropdown-item", i))
                                                .px(px(12.0))
                                                .py(px(6.0))
                                                .text_sm()
                                                .text_color(if is_destructive {
                                                    destructive
                                                } else {
                                                    popover_fg
                                                })
                                                .when(is_disabled, |el: Stateful<Div>| {
                                                    el.opacity(0.5)
                                                })
                                                .when(!is_disabled, |el: Stateful<Div>| {
                                                    el.cursor_pointer()
                                                        .hover(|style| {
                                                            style.bg(accent).text_color(
                                                                if is_destructive {
                                                                    destructive
                                                                } else {
                                                                    accent_fg
                                                                },
                                                            )
                                                        })
                                                        .on_click({
                                                            let val = item_value.clone();
                                                            let on_select = on_select.clone();
                                                            let on_open_change =
                                                                on_open_change.clone();
                                                            move |_event: &ClickEvent,
                                                                  window,
                                                                  cx| {
                                                                if let Some(ref handler) =
                                                                    on_select
                                                                {
                                                                    handler(
                                                                        val.as_ref(),
                                                                        window,
                                                                        cx,
                                                                    );
                                                                }
                                                                if let Some(ref handler) =
                                                                    on_open_change
                                                                {
                                                                    handler(false, window, cx);
                                                                }
                                                            }
                                                        })
                                                })
                                                .child(item.label.clone())
                                                .into_any_element()
                                        }
                                        DropdownMenuEntry::Separator => div()
                                            .h(px(1.0))
                                            .bg(border)
                                            .mx(px(8.0))
                                            .my(px(4.0))
                                            .into_any_element(),
                                        DropdownMenuEntry::Label(text) => div()
                                            .px(px(12.0))
                                            .py(px(6.0))
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(muted_foreground)
                                            .child(text)
                                            .into_any_element(),
                                    }),
                            ),
                    )
                    .with_priority(200),
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
    fn test_dropdown_menu_item_creation() {
        let item = DropdownMenuItem::new("val", "Label");
        assert_eq!(item.value.as_ref(), "val");
        assert_eq!(item.label.as_ref(), "Label");
        assert!(!item.disabled);
        assert!(!item.destructive);
    }

    #[test]
    fn test_dropdown_menu_item_builder() {
        let item = DropdownMenuItem::new("val", "Label")
            .disabled(true)
            .destructive(true);
        assert!(item.disabled);
        assert!(item.destructive);
    }

    #[test]
    fn test_dropdown_menu_defaults() {
        let menu = DropdownMenu::new("test");
        assert!(!menu.open);
        assert!(menu.entries.is_empty());
        assert!(menu.trigger.is_empty());
        assert!(menu.on_open_change.is_none());
        assert!(menu.on_select.is_none());
    }

    #[test]
    fn test_dropdown_menu_entries() {
        let menu = DropdownMenu::new("test")
            .item(DropdownMenuItem::new("a", "Alpha"))
            .separator()
            .label("Group")
            .item(DropdownMenuItem::new("b", "Beta"));
        assert_eq!(menu.entries.len(), 4);
    }

    #[test]
    fn test_dropdown_menu_builder() {
        let menu = DropdownMenu::new("test")
            .open(true)
            .item(DropdownMenuItem::new("a", "Alpha"))
            .separator()
            .label("Section")
            .item(DropdownMenuItem::new("delete", "Delete").destructive(true));
        assert!(menu.open);
        assert_eq!(menu.entries.len(), 4);
    }
}
