//! AlertDialog component for shadcn-ui-rs
//!
//! A modal confirmation dialog that requires an explicit user action or cancel.
//! Unlike Dialog, clicking the backdrop does NOT dismiss an AlertDialog.
//!
//! # Example
//!
//! ```rust
//! // In a stateful view:
//! struct MyView {
//!     alert_open: bool,
//! }
//!
//! impl Render for MyView {
//!     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!         let open = self.alert_open;
//!         div()
//!             .when(open, |el| {
//!                 el.child(
//!                     AlertDialog::new("confirm-delete")
//!                         .open(true)
//!                         .on_close(cx.listener(|this, _window, _cx| {
//!                             this.alert_open = false;
//!                         }))
//!                         .child(
//!                             AlertDialogContent::new()
//!                                 .child(
//!                                     AlertDialogHeader::new()
//!                                         .child(AlertDialogTitle::new("Are you sure?"))
//!                                         .child(AlertDialogDescription::new(
//!                                             "This action cannot be undone."
//!                                         ))
//!                                 )
//!                                 .child(
//!                                     AlertDialogFooter::new()
//!                                         .child(
//!                                             AlertDialogCancel::new()
//!                                                 .on_click(cx.listener(|this, _event, _window, _cx| {
//!                                                     this.alert_open = false;
//!                                                 }))
//!                                                 .child("Cancel")
//!                                         )
//!                                         .child(
//!                                             AlertDialogAction::new()
//!                                                 .on_click(|_event, _window, _cx| {
//!                                                     // perform action
//!                                                 })
//!                                                 .child("Continue")
//!                                         )
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
    deferred, div, prelude::*, AnyElement, App, ClickEvent, ElementId, FontWeight, IntoElement,
    KeyDownEvent, ParentElement, RenderOnce, SharedString, Styled, Window,
};

/// AlertDialog root component.
///
/// A modal confirmation dialog. Uses `deferred` with priority 100 to render
/// on top of all other content. Unlike Dialog, the backdrop does NOT close
/// the dialog -- the user must explicitly choose an action or cancel.
/// Pressing Escape still closes via `on_close`.
#[derive(IntoElement)]
pub struct AlertDialog {
    id: ElementId,
    open: bool,
    #[allow(clippy::type_complexity)]
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl AlertDialog {
    /// Create a new alert dialog with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            on_close: None,
            children: Vec::new(),
        }
    }

    /// Set the open state of the alert dialog.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set the callback for when the alert dialog should close.
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }
}

impl ParentElement for AlertDialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AlertDialog {
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
                        // No backdrop click dismiss -- user must choose action or cancel
                        .when_some(self.on_close, |el, on_close| {
                            let on_close: Rc<dyn Fn(&mut Window, &mut App)> = Rc::new(on_close);
                            el.on_key_down(
                                move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
                                    if event.keystroke.key.as_str() == "escape" {
                                        on_close(window, cx);
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

/// AlertDialog content container.
///
/// The main content area, rendered centered over the backdrop.
/// Uses popover theme colors, rounded corners, and shadow.
#[derive(IntoElement)]
pub struct AlertDialogContent {
    children: Vec<AnyElement>,
}

impl AlertDialogContent {
    /// Create a new alert dialog content container.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for AlertDialogContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AlertDialogContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        div()
            .id("alert-dialog-content")
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
            // Prevent click-through to backdrop
            .on_click(|_event: &ClickEvent, _window: &mut Window, _cx: &mut App| {})
            .children(self.children)
    }
}

/// AlertDialog header section.
///
/// Contains the title and description with vertical spacing.
#[derive(IntoElement)]
pub struct AlertDialogHeader {
    children: Vec<AnyElement>,
}

impl AlertDialogHeader {
    /// Create a new alert dialog header.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for AlertDialogHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AlertDialogHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(gpui::px(6.0))
            .children(self.children)
    }
}

/// AlertDialog title text.
///
/// Renders with semibold weight and larger text.
#[derive(IntoElement)]
pub struct AlertDialogTitle {
    text: SharedString,
}

impl AlertDialogTitle {
    /// Create a new alert dialog title.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for AlertDialogTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .text_lg()
            .font_weight(FontWeight::SEMIBOLD)
            .line_height(gpui::rems(1.75))
            .child(self.text)
    }
}

/// AlertDialog description text.
///
/// Renders with muted foreground color and smaller text.
#[derive(IntoElement)]
pub struct AlertDialogDescription {
    text: SharedString,
}

impl AlertDialogDescription {
    /// Create a new alert dialog description.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for AlertDialogDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .text_sm()
            .text_color(theme.colors.muted_foreground)
            .child(self.text)
    }
}

/// AlertDialog footer section.
///
/// A flex row at the bottom for action and cancel buttons, right-aligned.
#[derive(IntoElement)]
pub struct AlertDialogFooter {
    children: Vec<AnyElement>,
}

impl AlertDialogFooter {
    /// Create a new alert dialog footer.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for AlertDialogFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AlertDialogFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .justify_end()
            .gap(gpui::px(8.0))
            .children(self.children)
    }
}

/// AlertDialog action button.
///
/// A clickable container for the primary confirmation action.
#[derive(IntoElement)]
pub struct AlertDialogAction {
    #[allow(clippy::type_complexity)]
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl AlertDialogAction {
    /// Create a new alert dialog action element.
    pub fn new() -> Self {
        Self {
            on_click: None,
            children: Vec::new(),
        }
    }

    /// Set the click handler for the action.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl ParentElement for AlertDialogAction {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AlertDialogAction {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut el = div().id("alert-dialog-action").cursor_pointer();

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

/// AlertDialog cancel button.
///
/// A clickable container for the cancel/dismiss action.
#[derive(IntoElement)]
pub struct AlertDialogCancel {
    #[allow(clippy::type_complexity)]
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl AlertDialogCancel {
    /// Create a new alert dialog cancel element.
    pub fn new() -> Self {
        Self {
            on_click: None,
            children: Vec::new(),
        }
    }

    /// Set the click handler for the cancel action.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl ParentElement for AlertDialogCancel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AlertDialogCancel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut el = div().id("alert-dialog-cancel").cursor_pointer();

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
    fn test_alert_dialog_defaults() {
        let dialog = AlertDialog::new("test");
        assert!(!dialog.open);
        assert!(dialog.on_close.is_none());
        assert!(dialog.children.is_empty());
    }

    #[test]
    fn test_alert_dialog_open() {
        let dialog = AlertDialog::new("test").open(true);
        assert!(dialog.open);
    }

    #[test]
    fn test_alert_dialog_builder() {
        let dialog = AlertDialog::new("test-dialog").open(true);
        assert!(dialog.open);
    }

    #[test]
    fn test_alert_dialog_title() {
        let title = AlertDialogTitle::new("Are you sure?");
        assert_eq!(title.text, SharedString::from("Are you sure?"));
    }

    #[test]
    fn test_alert_dialog_description() {
        let desc = AlertDialogDescription::new("This action cannot be undone.");
        assert_eq!(
            desc.text,
            SharedString::from("This action cannot be undone.")
        );
    }

    #[test]
    fn test_alert_dialog_action_defaults() {
        let action = AlertDialogAction::new();
        assert!(action.on_click.is_none());
        assert!(action.children.is_empty());
    }

    #[test]
    fn test_alert_dialog_cancel_defaults() {
        let cancel = AlertDialogCancel::new();
        assert!(cancel.on_click.is_none());
        assert!(cancel.children.is_empty());
    }
}
