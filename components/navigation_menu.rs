//! NavigationMenu component for shadcn-ui-rs
//!
//! A multi-level navigation with wide dropdown panels.
//!
//! # Example
//! ```rust
//! NavigationMenu::new("main-nav")
//!     .child(
//!         NavigationMenuItem::new("products")
//!             .trigger("Products")
//!             .open(self.products_open)
//!             .on_open_change(|open, _window, _cx| { /* toggle */ })
//!             .child(
//!                 div().flex().gap(px(16.0))
//!                     .child(NavigationMenuLink::new("Widget A").description("Our flagship widget"))
//!                     .child(NavigationMenuLink::new("Widget B").description("Budget option"))
//!             )
//!     )
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
// NavigationMenu
// ---------------------------------------------------------------------------

/// A horizontal navigation menu containing [`NavigationMenuItem`] items.
#[derive(IntoElement)]
pub struct NavigationMenu {
    id: ElementId,
    children: Vec<AnyElement>,
}

impl NavigationMenu {
    /// Create a new navigation menu with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
        }
    }
}

impl ParentElement for NavigationMenu {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for NavigationMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .children(self.children)
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuItem
// ---------------------------------------------------------------------------

/// A single item within a [`NavigationMenu`], with a trigger and wide content panel.
#[derive(IntoElement)]
pub struct NavigationMenuItem {
    id: ElementId,
    open: bool,
    #[allow(clippy::type_complexity)]
    on_open_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
    trigger_label: SharedString,
    content: Vec<AnyElement>,
}

impl NavigationMenuItem {
    /// Create a new navigation menu item with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            on_open_change: None,
            trigger_label: SharedString::default(),
            content: Vec::new(),
        }
    }

    /// Set the open state of the menu item.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Register a callback for when the open state changes.
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }

    /// Set the trigger label displayed in the navigation bar.
    pub fn trigger(mut self, label: impl Into<SharedString>) -> Self {
        self.trigger_label = label.into();
        self
    }
}

impl ParentElement for NavigationMenuItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.content.extend(elements);
    }
}

impl RenderOnce for NavigationMenuItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;
        let radius = theme.radius.to_px();

        let border = colors.border;
        let popover_bg = colors.popover;
        let popover_fg = colors.popover_foreground;
        let accent = colors.accent;
        let accent_fg = colors.accent_foreground;

        let open = self.open;
        let on_open_change = self.on_open_change;

        div()
            .id(self.id)
            .relative()
            // Trigger button
            .child(
                div()
                    .id("nav-trigger")
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
            // Wide panel dropdown (only visible when open)
            .when(open, |el| {
                el.child(
                    deferred(
                        div()
                            .id("nav-panel")
                            .absolute()
                            .top(px(44.0))
                            .left_0()
                            .min_w(px(400.0))
                            .p(px(16.0))
                            .rounded(px(radius))
                            .border_1()
                            .border_color(border)
                            .bg(popover_bg)
                            .text_color(popover_fg)
                            .shadow_lg()
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
                            .children(self.content),
                    )
                    .with_priority(200),
                )
            })
    }
}

// ---------------------------------------------------------------------------
// NavigationMenuLink
// ---------------------------------------------------------------------------

/// A clickable link card within a [`NavigationMenuItem`] panel.
///
/// Displays a title and optional description. Highlights on hover.
#[derive(IntoElement)]
pub struct NavigationMenuLink {
    label: SharedString,
    description: Option<SharedString>,
    #[allow(clippy::type_complexity)]
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl NavigationMenuLink {
    /// Create a new navigation menu link with the given label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            description: None,
            on_click: None,
        }
    }

    /// Set a description for this link.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Register a click handler.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for NavigationMenuLink {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;
        let radius = theme.radius.to_px();

        let accent = colors.accent;
        let muted_foreground = colors.muted_foreground;

        let label_el = div()
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .child(self.label);

        let desc_el = self.description.map(|desc| {
            div()
                .text_sm()
                .text_color(muted_foreground)
                .child(desc)
        });

        if let Some(handler) = self.on_click {
            let mut el = div()
                .id("nav-link")
                .flex()
                .flex_col()
                .gap(px(2.0))
                .p(px(12.0))
                .rounded(px(radius))
                .cursor_pointer()
                .hover(|style| style.bg(accent))
                .on_click(move |event: &ClickEvent, window, cx| {
                    handler(event, window, cx);
                })
                .child(label_el);
            if let Some(desc) = desc_el {
                el = el.child(desc);
            }
            el.into_any_element()
        } else {
            let mut el = div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .p(px(12.0))
                .rounded(px(radius))
                .hover(|style| style.bg(accent))
                .child(label_el);
            if let Some(desc) = desc_el {
                el = el.child(desc);
            }
            el.into_any_element()
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_menu_defaults() {
        let menu = NavigationMenu::new("test");
        assert!(menu.children.is_empty());
    }

    #[test]
    fn test_navigation_menu_item() {
        let item = NavigationMenuItem::new("test")
            .trigger("Products")
            .open(true)
            .on_open_change(|_open, _window, _cx| {});
        assert!(item.open);
        assert_eq!(item.trigger_label.as_ref(), "Products");
        assert!(item.on_open_change.is_some());
        assert!(item.content.is_empty());
    }

    #[test]
    fn test_navigation_menu_link() {
        let link = NavigationMenuLink::new("Widget A")
            .description("Our flagship widget");
        assert_eq!(link.label.as_ref(), "Widget A");
        assert_eq!(link.description.as_deref(), Some("Our flagship widget"));
        assert!(link.on_click.is_none());
    }

    #[test]
    fn test_navigation_menu_link_builder() {
        let link = NavigationMenuLink::new("Widget B")
            .description("Budget option")
            .on_click(|_event, _window, _cx| {});
        assert_eq!(link.label.as_ref(), "Widget B");
        assert!(link.description.is_some());
        assert!(link.on_click.is_some());
    }
}
