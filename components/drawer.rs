//! Drawer component for shadcn-ui-rs
//!
//! A bottom sheet overlay panel with a drag handle indicator.
//!
//! # Example
//!
//! ```rust
//! // In a stateful view:
//! struct MyView {
//!     drawer_open: bool,
//! }
//!
//! impl Render for MyView {
//!     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!         let open = self.drawer_open;
//!         div()
//!             .child(
//!                 Button::new()
//!                     .on_click(cx.listener(|this, _event, _window, _cx| {
//!                         this.drawer_open = true;
//!                     }))
//!                     .child("Open Drawer")
//!             )
//!             .when(open, |el| {
//!                 el.child(
//!                     Drawer::new("my-drawer")
//!                         .open(true)
//!                         .on_close(cx.listener(|this, _window, _cx| {
//!                             this.drawer_open = false;
//!                         }))
//!                         .child(
//!                             DrawerContent::new()
//!                                 .child(
//!                                     DrawerHeader::new()
//!                                         .child(DrawerTitle::new("Drawer Title"))
//!                                         .child(DrawerDescription::new(
//!                                             "Drawer description text."
//!                                         ))
//!                                 )
//!                                 .child("Drawer body content")
//!                                 .child(
//!                                     DrawerFooter::new()
//!                                         .child("Submit")
//!                                 )
//!                         )
//!                 )
//!             })
//!     }
//! }
//! ```

use std::rc::Rc;

use gpui::{
    deferred, div, px, AnyElement, App, ClickEvent, ElementId, FontWeight, IntoElement,
    KeyDownEvent, ParentElement, RenderOnce, SharedString, Styled, Window,
};
use crate::theme::Theme;

/// Drawer root component.
///
/// A bottom sheet overlay panel. Uses GPUI's `deferred` element to render on
/// top of all other content with a semi-transparent backdrop.
///
/// Always slides in from the bottom of the screen. Clicking the backdrop or
/// pressing Escape closes the drawer.
#[derive(IntoElement)]
pub struct Drawer {
    id: ElementId,
    open: bool,
    #[allow(clippy::type_complexity)]
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl Drawer {
    /// Create a new drawer with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            on_close: None,
            children: Vec::new(),
        }
    }

    /// Set the open state of the drawer.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set the callback for when the drawer should close.
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }
}

impl ParentElement for Drawer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Drawer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let backdrop_color = gpui::hsla(0.0, 0.0, 0.0, 0.8);
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let popover_bg = colors.popover;
        let popover_fg = colors.popover_foreground;
        let border_color = colors.border;

        div().when(self.open, |el| {
            el.child(
                deferred(
                    div()
                        .id(self.id)
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .bg(backdrop_color)
                        .when_some(self.on_close, |el, on_close| {
                            let on_close: Rc<dyn Fn(&mut Window, &mut App)> = Rc::new(on_close);
                            let on_close_key = on_close.clone();
                            el.on_click({
                                move |_event: &ClickEvent, window: &mut Window, cx: &mut App| {
                                    on_close(window, cx);
                                }
                            })
                            .on_key_down(
                                move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
                                    if event.keystroke.key.as_str() == "escape" {
                                        on_close_key(window, cx);
                                    }
                                },
                            )
                        })
                        .child(
                            div()
                                .absolute()
                                .bottom_0()
                                .left_0()
                                .w_full()
                                .max_h(px(500.0))
                                .rounded_t_lg()
                                .bg(popover_bg)
                                .text_color(popover_fg)
                                .border_t_1()
                                .border_color(border_color)
                                .shadow_lg()
                                .children(self.children),
                        ),
                )
                .with_priority(100),
            )
        })
    }
}

/// Drawer content container.
///
/// The main content area of the drawer with a drag handle indicator at the top.
#[derive(IntoElement)]
pub struct DrawerContent {
    children: Vec<AnyElement>,
}

impl DrawerContent {
    /// Create a new drawer content container.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for DrawerContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DrawerContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        div()
            .id("drawer-content")
            .flex()
            .flex_col()
            .gap(px(16.0))
            .p(px(24.0))
            .h_full()
            .overflow_y_scroll()
            .bg(colors.popover)
            .text_color(colors.popover_foreground)
            // Prevent click-through to backdrop
            .on_click(|_event: &ClickEvent, _window: &mut Window, _cx: &mut App| {})
            // Drag handle indicator
            .child(
                div()
                    .mx_auto()
                    .mt(px(8.0))
                    .w(px(40.0))
                    .h(px(4.0))
                    .rounded_full()
                    .bg(colors.muted),
            )
            .children(self.children)
    }
}

/// Drawer header section.
///
/// Contains the title and description with vertical spacing.
#[derive(IntoElement)]
pub struct DrawerHeader {
    children: Vec<AnyElement>,
}

impl DrawerHeader {
    /// Create a new drawer header.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for DrawerHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DrawerHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .children(self.children)
    }
}

/// Drawer title text.
///
/// Renders with semibold weight and larger text.
#[derive(IntoElement)]
pub struct DrawerTitle {
    text: SharedString,
}

impl DrawerTitle {
    /// Create a new drawer title.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for DrawerTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .text_lg()
            .font_weight(FontWeight::SEMIBOLD)
            .line_height(gpui::rems(1.75))
            .child(self.text)
    }
}

/// Drawer description text.
///
/// Renders with muted foreground color and smaller text.
#[derive(IntoElement)]
pub struct DrawerDescription {
    text: SharedString,
}

impl DrawerDescription {
    /// Create a new drawer description.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for DrawerDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .text_sm()
            .text_color(theme.colors.muted_foreground)
            .child(self.text)
    }
}

/// Drawer footer section.
///
/// A flex row at the bottom of the drawer for action buttons.
#[derive(IntoElement)]
pub struct DrawerFooter {
    children: Vec<AnyElement>,
}

impl DrawerFooter {
    /// Create a new drawer footer.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for DrawerFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DrawerFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .justify_end()
            .gap(px(8.0))
            .children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawer_defaults() {
        let drawer = Drawer::new("test-drawer");
        assert!(!drawer.open);
        assert!(drawer.on_close.is_none());
    }

    #[test]
    fn test_drawer_builder() {
        let drawer = Drawer::new("test-drawer")
            .open(true)
            .on_close(|_window, _cx| {});
        assert!(drawer.open);
        assert!(drawer.on_close.is_some());
    }

    #[test]
    fn test_drawer_title() {
        let title = DrawerTitle::new("Test Title");
        assert_eq!(title.text, SharedString::from("Test Title"));
    }

    #[test]
    fn test_drawer_description() {
        let desc = DrawerDescription::new("Test description");
        assert_eq!(desc.text, SharedString::from("Test description"));
    }
}
