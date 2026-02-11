//! Checkbox component for shadcn-ui-rs
//!
//! A checkbox input with checked/unchecked states and theme-aware styling.
//!
//! # Usage
//!
//! ```rust
//! Checkbox::new("accept-terms")
//!     .checked(true)
//!     .on_toggle(|checked, _window, _cx| {
//!         println!("checked: {}", checked);
//!     })
//! ```

use gpui::prelude::*;
use gpui::{px, rems, App, ElementId, IntoElement, Window};

use crate::theme::Theme;

/// A themed checkbox component.
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    checked: bool,
    disabled: bool,
    on_toggle: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            disabled: false,
            on_toggle: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let checked = self.checked;
        let disabled = self.disabled;

        let (bg, border_col) = if checked {
            (colors.primary, colors.primary)
        } else {
            (colors.background, colors.border)
        };

        let check_color = colors.primary_foreground;
        let hover_border = colors.ring;

        let mut el = div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .size(px(16.0))
            .rounded_sm()
            .border_1()
            .border_color(border_col)
            .bg(bg);

        if checked {
            // Render a checkmark using a Unicode character
            el = el
                .child(
                    div()
                        .text_color(check_color)
                        .text_size(rems(0.625))
                        .child("✓"),
                );
        }

        if disabled {
            el = el.opacity(0.5).cursor_default();
        } else {
            el = el
                .cursor_pointer()
                .hover(move |s| s.border_color(hover_border));

            if let Some(handler) = self.on_toggle {
                let next_checked = !checked;
                el = el.on_click(move |_event, window, cx| {
                    handler(next_checked, window, cx);
                });
            }
        }

        el
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_builder() {
        let cb = Checkbox::new("test-cb")
            .checked(true)
            .disabled(true);

        assert!(cb.checked);
        assert!(cb.disabled);
    }

    #[test]
    fn test_checkbox_defaults() {
        let cb = Checkbox::new("my-cb");
        assert!(!cb.checked);
        assert!(!cb.disabled);
        assert!(cb.on_toggle.is_none());
    }
}
