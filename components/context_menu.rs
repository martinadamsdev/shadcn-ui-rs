//! ContextMenu component for shadcn-ui-rs
//!
//! A right-click triggered menu with items, separators, and labels.
//! Reuses `DropdownMenuEntry` and `DropdownMenuItem` from `dropdown_menu`.
//!
//! # Example
//! ```rust
//! ContextMenu::new("ctx")
//!     .open(is_open)
//!     .position(ctx_x, ctx_y)
//!     .on_open(|x, y, _window, _cx| {
//!         // Store position, set open = true
//!     })
//!     .on_close(|_window, _cx| {
//!         // Set open = false
//!     })
//!     .on_select(|value, _window, _cx| {
//!         println!("Selected: {value}");
//!     })
//!     .trigger(div().child("Right-click this area"))
//!     .item(DropdownMenuItem::new("cut", "Cut"))
//!     .item(DropdownMenuItem::new("copy", "Copy"))
//!     .separator()
//!     .item(DropdownMenuItem::new("paste", "Paste"))
//! ```

use std::rc::Rc;

use gpui::prelude::*;
use gpui::{
    deferred, div, px, AnyElement, App, ClickEvent, Div, ElementId, FontWeight, IntoElement,
    KeyDownEvent, MouseButton, MouseDownEvent, RenderOnce, SharedString, Stateful, Styled, Window,
};

use crate::dropdown_menu::{DropdownMenuEntry, DropdownMenuItem};
use crate::theme::Theme;

// ---------------------------------------------------------------------------
// ContextMenu
// ---------------------------------------------------------------------------

/// A right-click triggered context menu component.
///
/// This is a **controlled** component. The `open` state and `position` are
/// passed in as props; changes are communicated through callbacks.
#[derive(IntoElement)]
pub struct ContextMenu {
    id: ElementId,
    open: bool,
    position: (f32, f32),
    #[allow(clippy::type_complexity)]
    on_open: Option<Rc<dyn Fn(f32, f32, &mut Window, &mut App) + 'static>>,
    #[allow(clippy::type_complexity)]
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    #[allow(clippy::type_complexity)]
    on_select: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    entries: Vec<DropdownMenuEntry>,
    trigger: Vec<AnyElement>,
}

impl ContextMenu {
    /// Create a new context menu with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            position: (0.0, 0.0),
            on_open: None,
            on_close: None,
            on_select: None,
            entries: Vec::new(),
            trigger: Vec::new(),
        }
    }

    /// Set the open state of the context menu.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set the position where the context menu should appear.
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = (x, y);
        self
    }

    /// Register a callback for when the context menu is opened via right-click.
    /// The callback receives the (x, y) position of the right-click.
    pub fn on_open(
        mut self,
        handler: impl Fn(f32, f32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open = Some(Rc::new(handler));
        self
    }

    /// Register a callback for when the context menu is closed.
    pub fn on_close(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(Rc::new(handler));
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

    /// Add a trigger element that opens the context menu on right-click.
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

impl RenderOnce for ContextMenu {
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
        let (pos_x, pos_y) = self.position;
        let on_open = self.on_open;
        let on_close = self.on_close;
        let on_select = self.on_select;

        div()
            .id(self.id)
            .relative()
            // Trigger: right-click to open
            .child(
                div()
                    .id("context-trigger")
                    .on_mouse_down(MouseButton::Right, {
                        let on_open = on_open.clone();
                        move |event: &MouseDownEvent, window, cx| {
                            if let Some(ref handler) = on_open {
                                handler(
                                    event.position.x.0,
                                    event.position.y.0,
                                    window,
                                    cx,
                                );
                            }
                        }
                    })
                    .children(self.trigger),
            )
            // Context menu popover (only visible when open)
            .when(open, |el| {
                el.child(
                    deferred(
                        div()
                            .id("context-popover")
                            .absolute()
                            .left(px(pos_x))
                            .top(px(pos_y))
                            .min_w(px(160.0))
                            .rounded(px(radius))
                            .border_1()
                            .border_color(border)
                            .bg(popover_bg)
                            .shadow_lg()
                            .py(px(4.0))
                            // Close when clicking outside
                            .on_mouse_down_out({
                                let on_close = on_close.clone();
                                move |_event: &MouseDownEvent, window, cx| {
                                    if let Some(ref handler) = on_close {
                                        handler(window, cx);
                                    }
                                }
                            })
                            // Close on Escape key
                            .on_key_down({
                                let on_close = on_close.clone();
                                move |event: &KeyDownEvent, window, cx| {
                                    if event.keystroke.key.as_str() == "escape" {
                                        if let Some(ref handler) = on_close {
                                            handler(window, cx);
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
                                            let is_disabled = item.is_disabled();
                                            let is_destructive = item.is_destructive();
                                            let on_select = on_select.clone();
                                            let item_value = item.value().clone();

                                            div()
                                                .id(("context-item", i))
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
                                                            }
                                                        })
                                                })
                                                .child(item.label().clone())
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
    fn test_context_menu_defaults() {
        let menu = ContextMenu::new("test");
        assert!(!menu.open);
        assert_eq!(menu.position, (0.0, 0.0));
        assert!(menu.entries.is_empty());
        assert!(menu.trigger.is_empty());
        assert!(menu.on_open.is_none());
        assert!(menu.on_close.is_none());
        assert!(menu.on_select.is_none());
    }

    #[test]
    fn test_context_menu_builder() {
        let menu = ContextMenu::new("test")
            .open(true)
            .position(100.0, 200.0)
            .on_open(|_x, _y, _window, _cx| {})
            .on_close(|_window, _cx| {})
            .on_select(|_value, _window, _cx| {});
        assert!(menu.open);
        assert_eq!(menu.position, (100.0, 200.0));
        assert!(menu.on_open.is_some());
        assert!(menu.on_close.is_some());
        assert!(menu.on_select.is_some());
    }

    #[test]
    fn test_context_menu_entries() {
        let menu = ContextMenu::new("test")
            .item(DropdownMenuItem::new("cut", "Cut"))
            .item(DropdownMenuItem::new("copy", "Copy"))
            .separator()
            .label("Actions")
            .item(DropdownMenuItem::new("paste", "Paste"));
        assert_eq!(menu.entries.len(), 5);
    }
}
