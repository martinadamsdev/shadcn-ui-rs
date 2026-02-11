//! Kbd component for shadcn-ui-rs
//!
//! A keyboard shortcut display label with styled container.
//!
//! # Example
//!
//! ```rust
//! Kbd::new("⌘K")
//! ```

use gpui::{div, App, IntoElement, RenderOnce, SharedString, Styled, Window};
use crate::theme::Theme;

/// Keyboard shortcut display label.
///
/// Renders keys in a styled inline container with muted background,
/// border, and monospace-style text.
#[derive(IntoElement)]
pub struct Kbd {
    keys: SharedString,
}

impl Kbd {
    /// Create a new keyboard shortcut label.
    pub fn new(keys: impl Into<SharedString>) -> Self {
        Self { keys: keys.into() }
    }
}

impl RenderOnce for Kbd {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        div()
            .bg(colors.muted)
            .border_1()
            .border_color(colors.border)
            .rounded_md()
            .px(gpui::px(6.0))
            .py(gpui::px(2.0))
            .text_xs()
            .text_color(colors.muted_foreground)
            .child(self.keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kbd_new() {
        let kbd = Kbd::new("⌘K");
        assert_eq!(kbd.keys, SharedString::from("⌘K"));
    }

    #[test]
    fn test_kbd_builder() {
        let kbd = Kbd::new("Ctrl+C");
        assert_eq!(kbd.keys, SharedString::from("Ctrl+C"));
    }
}
