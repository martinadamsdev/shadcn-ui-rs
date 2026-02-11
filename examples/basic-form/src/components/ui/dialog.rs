//! Dialog component for shadcn-ui-rs
//!
//! A modal dialog overlay that displays content on top of the main interface.
//!
//! # Example
//!
//! ```rust
//! // In a stateful view:
//! struct MyView {
//!     dialog_open: bool,
//! }
//!
//! impl Render for MyView {
//!     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!         let open = self.dialog_open;
//!         div()
//!             .child(
//!                 DialogTrigger::new("open-dialog")
//!                     .on_click(cx.listener(|this, _event, _window, _cx| {
//!                         this.dialog_open = true;
//!                     }))
//!                     .child("Open Dialog")
//!             )
//!             .when(open, |el| {
//!                 el.child(
//!                     Dialog::new("my-dialog")
//!                         .open(true)
//!                         .on_close(cx.listener(|this, _window, _cx| {
//!                             this.dialog_open = false;
//!                         }))
//!                         .child(
//!                             DialogContent::new()
//!                                 .child(
//!                                     DialogHeader::new()
//!                                         .child(DialogTitle::new("Edit Profile"))
//!                                         .child(DialogDescription::new(
//!                                             "Make changes to your profile here."
//!                                         ))
//!                                 )
//!                                 .child("Dialog body content")
//!                                 .child(
//!                                     DialogFooter::new()
//!                                         .child("Save changes")
//!                                 )
//!                         )
//!                 )
//!             })
//!     }
//! }
//! ```

use std::rc::Rc;

use crate::theme::Theme;
use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, IntoElement, KeyDownEvent, ParentElement,
    RenderOnce, SharedString, Styled, Window, deferred, div, prelude::*,
};

/// Dialog root component.
///
/// Manages the open/close state and renders a full-screen overlay with content.
/// Uses GPUI's `deferred` element to render on top of all other content.
///
/// When open, renders a semi-transparent backdrop that centers its children.
/// Clicking the backdrop or pressing Escape closes the dialog.
#[derive(IntoElement)]
pub struct Dialog {
    id: ElementId,
    open: bool,
    #[allow(clippy::type_complexity)]
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl Dialog {
    /// Create a new dialog with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            on_close: None,
            children: Vec::new(),
        }
    }

    /// Set the open state of the dialog.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set the callback for when the dialog should close.
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }
}

impl ParentElement for Dialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Dialog {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let backdrop_color = gpui::hsla(0.0, 0.0, 0.0, 0.8);

        div().when(self.open, |el| {
            el.child(
                deferred(
                    div()
                        .id(self.id)
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(backdrop_color)
                        .when_some(self.on_close, |el, on_close| {
                            let on_close: Rc<dyn Fn(&mut Window, &mut App)> = Rc::new(on_close);
                            let on_close_key = on_close.clone();
                            el.on_click({
                                // Backdrop click closes the dialog
                                move |_event: &ClickEvent, window: &mut Window, cx: &mut App| {
                                    on_close(window, cx);
                                }
                            })
                            .on_key_down(
                                // Escape key closes the dialog
                                move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
                                    if event.keystroke.key.as_str() == "escape" {
                                        on_close_key(window, cx);
                                    }
                                },
                            )
                        })
                        .children(self.children),
                )
                .with_priority(100),
            )
        })
    }
}

/// Dialog trigger button.
///
/// A wrapper element that triggers the dialog to open when clicked.
#[derive(IntoElement)]
pub struct DialogTrigger {
    id: ElementId,
    #[allow(clippy::type_complexity)]
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl DialogTrigger {
    /// Create a new dialog trigger.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            on_click: None,
            children: Vec::new(),
        }
    }

    /// Set the click handler (typically opens the dialog).
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl ParentElement for DialogTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogTrigger {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut el = div().id(self.id).cursor_pointer();

        if let Some(on_click) = self.on_click {
            el = el.on_click(
                move |event: &ClickEvent, window: &mut Window, cx: &mut App| {
                    on_click(event, window, cx);
                },
            );
        }

        el.children(self.children)
    }
}

/// Dialog content container.
///
/// The main content area of the dialog, rendered centered over the backdrop.
/// Uses popover theme colors, rounded corners, and shadow.
#[derive(IntoElement)]
pub struct DialogContent {
    children: Vec<AnyElement>,
}

impl DialogContent {
    /// Create a new dialog content container.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for DialogContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        div()
            .id("dialog-content")
            .flex()
            .flex_col()
            .gap(gpui::px(16.0))
            .bg(colors.popover)
            .text_color(colors.popover_foreground)
            .border_1()
            .border_color(colors.border)
            .rounded_lg()
            .shadow_lg()
            .p(gpui::px(24.0))
            .max_w(gpui::px(512.0))
            .w_full()
            // Prevent click-through to backdrop (stops propagation)
            .on_click(|_event: &ClickEvent, _window: &mut Window, _cx: &mut App| {})
            .children(self.children)
    }
}

/// Dialog header section.
///
/// Contains the title and description with vertical spacing.
#[derive(IntoElement)]
pub struct DialogHeader {
    children: Vec<AnyElement>,
}

impl DialogHeader {
    /// Create a new dialog header.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for DialogHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(gpui::px(6.0))
            .children(self.children)
    }
}

/// Dialog title text.
///
/// Renders with semibold weight and larger text.
#[derive(IntoElement)]
pub struct DialogTitle {
    text: SharedString,
}

impl DialogTitle {
    /// Create a new dialog title.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for DialogTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .text_lg()
            .font_weight(FontWeight::SEMIBOLD)
            .line_height(gpui::rems(1.75))
            .child(self.text)
    }
}

/// Dialog description text.
///
/// Renders with muted foreground color and smaller text.
#[derive(IntoElement)]
pub struct DialogDescription {
    text: SharedString,
}

impl DialogDescription {
    /// Create a new dialog description.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for DialogDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .text_sm()
            .text_color(theme.colors.muted_foreground)
            .child(self.text)
    }
}

/// Dialog footer section.
///
/// A flex row at the bottom of the dialog for action buttons.
/// Buttons are right-aligned by default.
#[derive(IntoElement)]
pub struct DialogFooter {
    children: Vec<AnyElement>,
}

impl DialogFooter {
    /// Create a new dialog footer.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for DialogFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .justify_end()
            .gap(gpui::px(8.0))
            .children(self.children)
    }
}

/// Dialog close button.
///
/// A wrapper that closes the dialog when clicked.
#[derive(IntoElement)]
pub struct DialogClose {
    #[allow(clippy::type_complexity)]
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl DialogClose {
    /// Create a new dialog close element.
    pub fn new() -> Self {
        Self {
            on_click: None,
            children: Vec::new(),
        }
    }

    /// Set the click handler (typically closes the dialog).
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl ParentElement for DialogClose {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogClose {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut el = div().id("dialog-close").cursor_pointer();

        if let Some(on_click) = self.on_click {
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
    fn test_dialog_builder() {
        let dialog = Dialog::new("test-dialog").open(true);
        assert!(dialog.open);
    }

    #[test]
    fn test_dialog_defaults() {
        let dialog = Dialog::new("test");
        assert!(!dialog.open);
        assert!(dialog.on_close.is_none());
    }

    #[test]
    fn test_dialog_title() {
        let title = DialogTitle::new("Test Title");
        assert_eq!(title.text, SharedString::from("Test Title"));
    }

    #[test]
    fn test_dialog_description() {
        let desc = DialogDescription::new("Test description");
        assert_eq!(desc.text, SharedString::from("Test description"));
    }
}
