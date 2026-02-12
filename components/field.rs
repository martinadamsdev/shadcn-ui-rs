//! Field component for shadcn-ui-rs
//!
//! A form field wrapper with label, input slot, description, and error message.
//!
//! # Example
//!
//! ```rust
//! Field::new()
//!     .label("Email")
//!     .description("Enter your email address")
//!     .child(Input::new("email"))
//! ```

use gpui::prelude::*;
use gpui::{div, px, AnyElement, App, FontWeight, IntoElement, ParentElement, SharedString, Window};

use crate::theme::Theme;

/// A form field wrapper component.
///
/// Renders a vertical stack with optional label, child input slot,
/// optional description text, and optional error message.
#[derive(IntoElement)]
pub struct Field {
    children: Vec<AnyElement>,
    label: Option<SharedString>,
    description: Option<SharedString>,
    error: Option<SharedString>,
}

impl Field {
    /// Create a new field.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            label: None,
            description: None,
            error: None,
        }
    }

    /// Set the field label text.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the field description text.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the field error message.
    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self
    }
}

impl ParentElement for Field {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Field {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let mut el = div().flex().flex_col().gap(px(8.0));

        if let Some(label) = self.label {
            el = el.child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .child(label),
            );
        }

        el = el.children(self.children);

        if let Some(description) = self.description {
            el = el.child(
                div()
                    .text_sm()
                    .text_color(colors.muted_foreground)
                    .child(description),
            );
        }

        if let Some(error) = self.error {
            el = el.child(
                div()
                    .text_sm()
                    .text_color(colors.destructive)
                    .child(error),
            );
        }

        el
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_defaults() {
        let field = Field::new();
        assert!(field.label.is_none());
        assert!(field.description.is_none());
        assert!(field.error.is_none());
        assert!(field.children.is_empty());
    }

    #[test]
    fn test_field_builder() {
        let field = Field::new()
            .label("Username")
            .description("Choose a unique username")
            .error("Username is taken")
            .child(div().child("input placeholder"));
        assert_eq!(field.label, Some(SharedString::from("Username")));
        assert_eq!(
            field.description,
            Some(SharedString::from("Choose a unique username"))
        );
        assert_eq!(field.error, Some(SharedString::from("Username is taken")));
        assert_eq!(field.children.len(), 1);
    }

    #[test]
    fn test_field_error() {
        let field = Field::new().error("This field is required");
        assert_eq!(
            field.error,
            Some(SharedString::from("This field is required"))
        );
    }
}
