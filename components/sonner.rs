//! Sonner component for shadcn-ui-rs
//!
//! A stacked toast notification manager that renders multiple toasts at a
//! configurable screen position.
//!
//! # Example
//!
//! ```rust
//! Sonner::new("my-sonner")
//!     .position(SonnerPosition::BottomRight)
//!     .on_dismiss(|toast_id, _window, _cx| {
//!         println!("Dismissed: {}", toast_id);
//!     })
//!     .toast(SonnerToast {
//!         id: "t1".into(),
//!         title: "Event Created".into(),
//!         description: Some("Your event has been scheduled.".into()),
//!         variant: SonnerVariant::Default,
//!     })
//! ```

use std::rc::Rc;

use gpui::{
    deferred, div, px, App, ClickEvent, ElementId, FontWeight, IntoElement, ParentElement,
    RenderOnce, SharedString, Styled, Window,
};
use crate::theme::Theme;

/// Sonner toast visual variant.
///
/// Defined locally (not imported from toast.rs) because each component file
/// is standalone and copied independently to user projects.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SonnerVariant {
    #[default]
    Default,
    Destructive,
}

/// Position for the sonner toast stack on screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SonnerPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

/// A single toast entry managed by Sonner.
#[derive(Clone)]
pub struct SonnerToast {
    pub id: SharedString,
    pub title: SharedString,
    pub description: Option<SharedString>,
    pub variant: SonnerVariant,
}

/// Stacked toast notification manager.
///
/// Renders a list of toasts at a fixed screen position using `deferred`
/// with priority 300.
#[derive(IntoElement)]
pub struct Sonner {
    id: ElementId,
    toasts: Vec<SonnerToast>,
    position: SonnerPosition,
    #[allow(clippy::type_complexity)]
    on_dismiss: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl Sonner {
    /// Create a new sonner container with the given ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            toasts: Vec::new(),
            position: SonnerPosition::BottomRight,
            on_dismiss: None,
        }
    }

    /// Set the position of the toast stack.
    pub fn position(mut self, position: SonnerPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the dismiss handler, called with the toast ID when dismissed.
    pub fn on_dismiss(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self
    }

    /// Add a single toast.
    pub fn toast(mut self, toast: SonnerToast) -> Self {
        self.toasts.push(toast);
        self
    }

    /// Add multiple toasts.
    pub fn toasts(mut self, toasts: impl IntoIterator<Item = SonnerToast>) -> Self {
        self.toasts.extend(toasts);
        self
    }
}

impl RenderOnce for Sonner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = theme.colors.clone();

        if self.toasts.is_empty() {
            return div();
        }

        let mut container = div()
            .id(self.id)
            .absolute()
            .flex()
            .flex_col()
            .gap(px(8.0));

        // Apply position
        container = match self.position {
            SonnerPosition::BottomRight => container.bottom(px(16.0)).right(px(16.0)),
            SonnerPosition::BottomLeft => container.bottom(px(16.0)).left(px(16.0)),
            SonnerPosition::BottomCenter => container
                .bottom(px(16.0))
                .left_0()
                .right_0()
                .items_center(),
            SonnerPosition::TopRight => container.top(px(16.0)).right(px(16.0)),
            SonnerPosition::TopLeft => container.top(px(16.0)).left(px(16.0)),
            SonnerPosition::TopCenter => container
                .top(px(16.0))
                .left_0()
                .right_0()
                .items_center(),
        };

        for toast in self.toasts {
            let (bg, fg, border_color) = match toast.variant {
                SonnerVariant::Default => (colors.background, colors.foreground, colors.border),
                SonnerVariant::Destructive => (
                    colors.destructive,
                    colors.destructive_foreground,
                    colors.destructive,
                ),
            };

            let toast_id = toast.id.clone();
            let on_dismiss = self.on_dismiss.clone();

            let mut toast_el = div()
                .w(px(356.0))
                .rounded_lg()
                .border_1()
                .border_color(border_color)
                .shadow_md()
                .p(px(16.0))
                .bg(bg)
                .text_color(fg)
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
                                .child(toast.title),
                        )
                        .when_some(toast.description, |el, desc| {
                            el.child(div().text_sm().opacity(0.9).child(desc))
                        }),
                );

            if let Some(on_dismiss) = on_dismiss {
                toast_el = toast_el.child(
                    div()
                        .id(SharedString::from(format!("sonner-close-{}", toast_id)))
                        .cursor_pointer()
                        .ml(px(8.0))
                        .on_click(move |_event: &ClickEvent, window: &mut Window, cx: &mut App| {
                            on_dismiss(toast_id.as_ref(), window, cx);
                        })
                        .child("X"),
                );
            }

            container = container.child(toast_el);
        }

        div().child(deferred(container).with_priority(300))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sonner_defaults() {
        let sonner = Sonner::new("test");
        assert!(sonner.toasts.is_empty());
        assert_eq!(sonner.position, SonnerPosition::BottomRight);
        assert!(sonner.on_dismiss.is_none());
    }

    #[test]
    fn test_sonner_toast() {
        let toast = SonnerToast {
            id: "t1".into(),
            title: "Hello".into(),
            description: Some("World".into()),
            variant: SonnerVariant::Default,
        };
        assert_eq!(toast.title, SharedString::from("Hello"));
        assert_eq!(toast.description, Some(SharedString::from("World")));
        assert_eq!(toast.variant, SonnerVariant::Default);
    }

    #[test]
    fn test_sonner_position() {
        let sonner = Sonner::new("test").position(SonnerPosition::TopLeft);
        assert_eq!(sonner.position, SonnerPosition::TopLeft);

        let sonner = Sonner::new("test").position(SonnerPosition::TopCenter);
        assert_eq!(sonner.position, SonnerPosition::TopCenter);

        let sonner = Sonner::new("test").position(SonnerPosition::TopRight);
        assert_eq!(sonner.position, SonnerPosition::TopRight);

        let sonner = Sonner::new("test").position(SonnerPosition::BottomLeft);
        assert_eq!(sonner.position, SonnerPosition::BottomLeft);

        let sonner = Sonner::new("test").position(SonnerPosition::BottomCenter);
        assert_eq!(sonner.position, SonnerPosition::BottomCenter);

        let sonner = Sonner::new("test").position(SonnerPosition::BottomRight);
        assert_eq!(sonner.position, SonnerPosition::BottomRight);
    }

    #[test]
    fn test_sonner_add_toasts() {
        let sonner = Sonner::new("test")
            .toast(SonnerToast {
                id: "t1".into(),
                title: "First".into(),
                description: None,
                variant: SonnerVariant::Default,
            })
            .toasts(vec![
                SonnerToast {
                    id: "t2".into(),
                    title: "Second".into(),
                    description: None,
                    variant: SonnerVariant::Default,
                },
                SonnerToast {
                    id: "t3".into(),
                    title: "Third".into(),
                    description: Some("desc".into()),
                    variant: SonnerVariant::Destructive,
                },
            ]);
        assert_eq!(sonner.toasts.len(), 3);
        assert_eq!(sonner.toasts[0].title, SharedString::from("First"));
        assert_eq!(sonner.toasts[1].title, SharedString::from("Second"));
        assert_eq!(sonner.toasts[2].title, SharedString::from("Third"));
        assert_eq!(sonner.toasts[2].variant, SonnerVariant::Destructive);
    }
}
