//! Label component for shadcn-ui-rs
//!
//! A simple themed text label, typically used alongside form inputs.
//!
//! # Usage
//!
//! ```rust
//! Label::new("Email address")
//! ```

use gpui::prelude::*;
use gpui::{App, FontWeight, IntoElement, SharedString, Window, div};

use crate::theme::Theme;

/// A themed text label component.
#[derive(IntoElement)]
pub struct Label {
    text: SharedString,
    required: bool,
}

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            required: false,
        }
    }

    /// Mark the label as required (shows an asterisk).
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let mut el = div()
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .text_color(theme.colors.foreground);

        if self.required {
            el = el.child(format!("{} *", self.text));
        } else {
            el = el.child(self.text);
        }

        el
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_builder() {
        let label = Label::new("Username").required(true);
        assert_eq!(label.text, SharedString::from("Username"));
        assert!(label.required);
    }

    #[test]
    fn test_label_defaults() {
        let label = Label::new("Name");
        assert!(!label.required);
    }
}
