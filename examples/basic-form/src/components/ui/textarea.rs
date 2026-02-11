//! Textarea component for shadcn-ui-rs
//!
//! A multi-line text display with theme-aware styling, placeholder support,
//! and disabled state. Styled identically to Input but with configurable
//! minimum height based on row count.
//!
//! **Note:** This component renders a visual representation of a textarea.
//! GPUI does not provide a built-in editable multi-line text widget, so actual
//! text editing requires platform-specific integration. Use this component for
//! static form displays or as a styling reference when building your own
//! editable textarea.
//!
//! # Example
//!
//! ```rust
//! Textarea::new("description")
//!     .placeholder("Enter description...")
//!     .value("Some long text\nwith multiple lines")
//!     .min_rows(5)
//! ```

use gpui::prelude::*;
use gpui::{div, px, App, Div, ElementId, IntoElement, SharedString, Stateful, Window};

use crate::theme::{Radius, Theme};

/// A themed multi-line text display component.
#[derive(IntoElement)]
pub struct Textarea {
    id: ElementId,
    placeholder: Option<SharedString>,
    value: Option<SharedString>,
    disabled: bool,
    min_rows: u32,
}

impl Textarea {
    /// Create a new textarea with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            placeholder: None,
            value: None,
            disabled: false,
            min_rows: 3,
        }
    }

    /// Set the placeholder text.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Set the display value.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the minimum number of visible rows.
    pub fn min_rows(mut self, min_rows: u32) -> Self {
        self.min_rows = min_rows;
        self
    }
}

impl RenderOnce for Textarea {
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

        let is_placeholder = self.value.as_ref().is_none_or(|v| v.is_empty());
        let text_col = if is_placeholder {
            colors.muted_foreground
        } else {
            colors.foreground
        };

        let focus_border = colors.ring;
        let border_color = colors.input;

        // Line height ~20px, plus vertical padding (6px top + 6px bottom = 12px)
        let min_height = px(self.min_rows as f32 * 20.0 + 12.0);

        let mut el = apply_radius(div().id(self.id).flex().w_full(), radius)
            .min_h(min_height)
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
fn apply_radius(el: Stateful<Div>, radius: Radius) -> Stateful<Div> {
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
    fn test_textarea_defaults() {
        let textarea = Textarea::new("test");
        assert!(textarea.placeholder.is_none());
        assert!(textarea.value.is_none());
        assert!(!textarea.disabled);
        assert_eq!(textarea.min_rows, 3);
    }

    #[test]
    fn test_textarea_builder() {
        let textarea = Textarea::new("test")
            .placeholder("Enter text...")
            .value("Hello\nWorld")
            .min_rows(5)
            .disabled(true);

        assert_eq!(
            textarea.placeholder,
            Some(SharedString::from("Enter text..."))
        );
        assert_eq!(textarea.value, Some(SharedString::from("Hello\nWorld")));
        assert_eq!(textarea.min_rows, 5);
        assert!(textarea.disabled);
    }

    #[test]
    fn test_textarea_disabled() {
        let textarea = Textarea::new("test").disabled(true);
        assert!(textarea.disabled);
    }
}
