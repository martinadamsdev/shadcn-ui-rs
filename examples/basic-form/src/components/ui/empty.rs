//! Empty state component for shadcn-ui-rs
//!
//! A centered placeholder for empty states with title, optional description,
//! and optional action element.
//!
//! # Example
//!
//! ```rust
//! Empty::new("No results")
//!     .description("Try adjusting your search query")
//!     .action(Button::new("Clear filters"))
//! ```

use gpui::{
    div, prelude::*, AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce,
    SharedString, Styled, Window,
};
use crate::theme::Theme;

/// Empty state placeholder component.
///
/// Displays a centered layout with a title, optional description text, and
/// an optional action element (e.g. a button).
#[derive(IntoElement)]
pub struct Empty {
    title: SharedString,
    description: Option<SharedString>,
    action: Option<AnyElement>,
}

impl Empty {
    /// Create a new empty state with the given title.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            action: None,
        }
    }

    /// Set the description text.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the action element (e.g. a button).
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }
}

impl RenderOnce for Empty {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let mut el = div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(gpui::px(8.0))
            .py(gpui::px(40.0))
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(colors.foreground)
                    .child(self.title),
            );

        if let Some(description) = self.description {
            el = el.child(
                div()
                    .text_sm()
                    .text_color(colors.muted_foreground)
                    .child(description),
            );
        }

        if let Some(action) = self.action {
            el = el.child(div().mt(gpui::px(16.0)).child(action));
        }

        el
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_defaults() {
        let empty = Empty::new("No results");
        assert_eq!(empty.title, SharedString::from("No results"));
        assert!(empty.description.is_none());
        assert!(empty.action.is_none());
    }

    #[test]
    fn test_empty_builder() {
        let empty = Empty::new("No items").description("Try a different query");
        assert_eq!(empty.title, SharedString::from("No items"));
        assert_eq!(
            empty.description,
            Some(SharedString::from("Try a different query"))
        );
    }

    #[test]
    fn test_empty_description() {
        let empty = Empty::new("Empty").description("Nothing here");
        assert_eq!(
            empty.description,
            Some(SharedString::from("Nothing here"))
        );
    }
}
