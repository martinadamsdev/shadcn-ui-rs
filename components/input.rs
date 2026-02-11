//! Input component for shadcn-ui-rs
//!
//! A text input field with theme-aware styling, placeholder support, and
//! disabled state.
//!
//! # Usage
//!
//! ```rust
//! Input::new("email-input")
//!     .placeholder("Enter your email")
//!     .value("hello@example.com")
//!     .disabled(false)
//! ```

use gpui::prelude::*;
use gpui::{px, App, ElementId, IntoElement, SharedString, Window};

use crate::theme::{Radius, Theme};

/// A themed text input component.
#[derive(IntoElement)]
pub struct Input {
    id: ElementId,
    placeholder: Option<SharedString>,
    value: Option<SharedString>,
    disabled: bool,
}

impl Input {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            placeholder: None,
            value: None,
            disabled: false,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for Input {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;
        let radius = theme.radius;

        let display_text: SharedString = if let Some(val) = &self.value {
            if val.is_empty() {
                self.placeholder.clone().unwrap_or_default()
            } else {
                val.clone()
            }
        } else {
            self.placeholder.clone().unwrap_or_default()
        };

        let is_placeholder = self.value.as_ref().map_or(true, |v| v.is_empty());
        let text_col = if is_placeholder {
            colors.muted_foreground
        } else {
            colors.foreground
        };

        let focus_border = colors.ring;
        let border_color = colors.input;

        let mut el = apply_radius(div().id(self.id).flex().items_center().w_full(), radius)
            .h(px(36.0))
            .border_1()
            .border_color(border_color)
            .bg(colors.background)
            .px(px(12.0))
            .py(px(6.0))
            .text_sm()
            .text_color(text_col);

        if self.disabled {
            el = el.opacity(0.5).cursor_default();
        } else {
            el = el
                .cursor_text()
                .hover(move |s| s.border_color(focus_border));
        }

        el.child(display_text)
    }
}

/// Apply the theme's border radius to a styled element.
fn apply_radius<E: Styled>(el: E, radius: Radius) -> E {
    match radius {
        Radius::None => el.rounded_none(),
        Radius::Sm => el.rounded_sm(),
        Radius::Md => el.rounded_md(),
        Radius::Lg => el.rounded_lg(),
        Radius::Full => el.rounded_full(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_builder() {
        let input = Input::new("test-input")
            .placeholder("Enter text")
            .value("hello")
            .disabled(true);

        assert_eq!(input.placeholder, Some(SharedString::from("Enter text")));
        assert_eq!(input.value, Some(SharedString::from("hello")));
        assert!(input.disabled);
    }

    #[test]
    fn test_input_defaults() {
        let input = Input::new("my-input");
        assert!(input.placeholder.is_none());
        assert!(input.value.is_none());
        assert!(!input.disabled);
    }
}
