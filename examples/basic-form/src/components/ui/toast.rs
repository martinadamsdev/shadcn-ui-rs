//! Toast component for shadcn-ui-rs
//!
//! A temporary notification that appears at the edge of the screen.
//!
//! # Example
//!
//! ```rust
//! Toast::new("my-toast", "Event Created")
//!     .description("Your event has been scheduled.")
//!     .variant(ToastVariant::Default)
//!     .open(true)
//!     .on_close(|_window, _cx| {
//!         // handle close
//!     })
//! ```

use gpui::{
    deferred, div, prelude::*, px, AnyElement, App, ClickEvent, Div, ElementId, FontWeight,
    IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window,
};
use crate::theme::Theme;

/// Toast visual variant.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ToastVariant {
    #[default]
    Default,
    Destructive,
}

/// A toast notification component.
///
/// Renders as a card-like overlay at the bottom-right of the screen.
/// Uses `deferred` with priority 300 to render above other overlays.
#[derive(IntoElement)]
pub struct Toast {
    id: ElementId,
    title: SharedString,
    description: Option<SharedString>,
    variant: ToastVariant,
    open: bool,
    #[allow(clippy::type_complexity)]
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    action: Option<AnyElement>,
}

impl Toast {
    /// Create a new toast with the given ID and title.
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            variant: ToastVariant::Default,
            open: false,
            on_close: None,
            action: None,
        }
    }

    /// Set the description text.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the toast variant.
    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the open state.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set the close handler.
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }

    /// Set an optional action element (e.g., a button).
    pub fn action(mut self, element: impl IntoElement) -> Self {
        self.action = Some(element.into_any_element());
        self
    }
}

impl RenderOnce for Toast {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let (bg, fg, border_color) = match self.variant {
            ToastVariant::Default => (colors.background, colors.foreground, colors.border),
            ToastVariant::Destructive => (
                colors.destructive,
                colors.destructive_foreground,
                colors.destructive,
            ),
        };

        div().when(self.open, move |el: Div| {
            el.child(
                deferred(
                    div()
                        .id(self.id)
                        .absolute()
                        .bottom(px(16.0))
                        .right(px(16.0))
                        .w(px(356.0))
                        .rounded_lg()
                        .border_1()
                        .border_color(border_color)
                        .shadow_lg()
                        .p(px(16.0))
                        .bg(bg)
                        .text_color(fg)
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(4.0))
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child(self.title),
                                        )
                                        .when_some(self.description, |el, desc| {
                                            el.child(
                                                div().text_sm().opacity(0.9).child(desc),
                                            )
                                        }),
                                )
                                .when_some(self.on_close, |el, on_close| {
                                    el.child(
                                        div()
                                            .id("toast-close")
                                            .cursor_pointer()
                                            .ml(px(8.0))
                                            .on_click(
                                                move |_event: &ClickEvent,
                                                      window: &mut Window,
                                                      cx: &mut App| {
                                                    on_close(window, cx);
                                                },
                                            )
                                            .child("X"),
                                    )
                                }),
                        )
                        .when_some(self.action, |el, action| el.child(action)),
                )
                .with_priority(300),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toast_defaults() {
        let toast = Toast::new("test", "Title");
        assert_eq!(toast.variant, ToastVariant::Default);
        assert!(!toast.open);
        assert!(toast.description.is_none());
        assert!(toast.on_close.is_none());
        assert!(toast.action.is_none());
    }

    #[test]
    fn test_toast_builder() {
        let toast = Toast::new("test", "Title")
            .description("A description")
            .variant(ToastVariant::Default)
            .open(true);
        assert_eq!(toast.title, SharedString::from("Title"));
        assert_eq!(
            toast.description,
            Some(SharedString::from("A description"))
        );
        assert_eq!(toast.variant, ToastVariant::Default);
        assert!(toast.open);
    }

    #[test]
    fn test_toast_destructive() {
        let toast = Toast::new("test", "Error")
            .variant(ToastVariant::Destructive);
        assert_eq!(toast.variant, ToastVariant::Destructive);
    }
}
