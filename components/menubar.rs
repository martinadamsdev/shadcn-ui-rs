//! Menubar component for shadcn-ui-rs
//!
//! An application menu bar with multiple dropdown menus.
//! Reuses `DropdownMenuEntry` and `DropdownMenuItem` from `dropdown_menu`.
//!
//! # Example
//! ```rust
//! Menubar::new("main-menu")
//!     .child(
//!         MenubarMenu::new("file")
//!             .trigger("File")
//!             .open(self.file_menu_open)
//!             .on_open_change(|open, _window, _cx| { /* toggle */ })
//!             .on_select(|value, _window, _cx| { /* handle */ })
//!             .item(DropdownMenuItem::new("new", "New File"))
//!             .item(DropdownMenuItem::new("open", "Open..."))
//!             .separator()
//!             .item(DropdownMenuItem::new("quit", "Quit"))
//!     )
//! ```

use std::rc::Rc;

use gpui::prelude::*;
use gpui::{
    deferred, div, px, AnyElement, App, ClickEvent, Div, ElementId, FontWeight, IntoElement,
    KeyDownEvent, MouseDownEvent, ParentElement, RenderOnce, SharedString, Stateful, Styled,
    Window,
};

use crate::dropdown_menu::{DropdownMenuEntry, DropdownMenuItem};
use crate::theme::Theme;

// ---------------------------------------------------------------------------
// Menubar
// ---------------------------------------------------------------------------

/// A horizontal application menu bar containing [`MenubarMenu`] items.
#[derive(IntoElement)]
pub struct Menubar {
    id: ElementId,
    children: Vec<AnyElement>,
}

impl Menubar {
    /// Create a new menubar with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
        }
    }
}

impl ParentElement for Menubar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Menubar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let background = colors.background;
        let border = colors.border;

        div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .h(px(40.0))
            .border_b_1()
            .border_color(border)
            .bg(background)
            .px(px(8.0))
            .children(self.children)
    }
}

// ---------------------------------------------------------------------------
// MenubarMenu
// ---------------------------------------------------------------------------

/// A single menu within a [`Menubar`], with a trigger label and dropdown entries.
#[derive(IntoElement)]
pub struct MenubarMenu {
    id: ElementId,
    open: bool,
    #[allow(clippy::type_complexity)]
    on_open_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
    #[allow(clippy::type_complexity)]
    on_select: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    trigger_label: SharedString,
    entries: Vec<DropdownMenuEntry>,
}

impl MenubarMenu {
    /// Create a new menubar menu with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            on_open_change: None,
            on_select: None,
            trigger_label: SharedString::default(),
            entries: Vec::new(),
        }
    }

    /// Set the open state of the menu.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Register a callback for when the menu open state changes.
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

    /// Set the trigger label displayed in the menu bar.
    pub fn trigger(mut self, label: impl Into<SharedString>) -> Self {
        self.trigger_label = label.into();
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

impl RenderOnce for MenubarMenu {
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
            // Trigger button
            .child(
                div()
                    .id("menubar-trigger")
                    .px(px(12.0))
                    .py(px(4.0))
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .rounded(px(radius))
                    .cursor_pointer()
                    .when(open, |el: Stateful<Div>| {
                        el.bg(accent).text_color(accent_fg)
                    })
                    .when(!open, |el: Stateful<Div>| {
                        el.hover(|style| style.bg(accent))
                    })
                    .on_click({
                        let on_open_change = on_open_change.clone();
                        move |_event: &ClickEvent, window, cx| {
                            if let Some(ref handler) = on_open_change {
                                handler(!open, window, cx);
                            }
                        }
                    })
                    .child(self.trigger_label),
            )
            // Dropdown (only visible when open)
            .when(open, |el| {
                el.child(
                    deferred(
                        div()
                            .id("menubar-popover")
                            .absolute()
                            .top(px(32.0))
                            .left_0()
                            .min_w(px(160.0))
                            .rounded(px(radius))
                            .border_1()
                            .border_color(border)
                            .bg(popover_bg)
                            .shadow_lg()
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
                                            let is_disabled = item.is_disabled();
                                            let is_destructive = item.is_destructive();
                                            let on_select = on_select.clone();
                                            let on_open_change = on_open_change.clone();
                                            let item_value = item.value().clone();

                                            div()
                                                .id(("menubar-item", i))
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
// MenubarSeparator
// ---------------------------------------------------------------------------

/// A vertical separator between [`MenubarMenu`] items.
#[derive(IntoElement)]
pub struct MenubarSeparator;

impl MenubarSeparator {
    /// Create a new menubar separator.
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for MenubarSeparator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let border = theme.colors.border;

        div()
            .w(px(1.0))
            .h(px(16.0))
            .bg(border)
            .mx(px(4.0))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menubar_defaults() {
        let bar = Menubar::new("test");
        assert!(bar.children.is_empty());
    }

    #[test]
    fn test_menubar_menu_defaults() {
        let menu = MenubarMenu::new("test");
        assert!(!menu.open);
        assert!(menu.entries.is_empty());
        assert!(menu.on_open_change.is_none());
        assert!(menu.on_select.is_none());
        assert_eq!(menu.trigger_label.as_ref(), "");
    }

    #[test]
    fn test_menubar_menu_builder() {
        let menu = MenubarMenu::new("test")
            .open(true)
            .trigger("File")
            .on_open_change(|_open, _window, _cx| {})
            .on_select(|_value, _window, _cx| {})
            .item(DropdownMenuItem::new("new", "New"))
            .separator()
            .label("Section")
            .item(DropdownMenuItem::new("quit", "Quit"));
        assert!(menu.open);
        assert_eq!(menu.trigger_label.as_ref(), "File");
        assert!(menu.on_open_change.is_some());
        assert!(menu.on_select.is_some());
        assert_eq!(menu.entries.len(), 4);
    }

    #[test]
    fn test_menubar_menu_entries() {
        let menu = MenubarMenu::new("test")
            .item(DropdownMenuItem::new("a", "Alpha"))
            .separator()
            .label("Group")
            .item(DropdownMenuItem::new("b", "Beta"));
        assert_eq!(menu.entries.len(), 4);
    }
}
