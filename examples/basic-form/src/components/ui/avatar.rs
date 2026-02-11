//! Avatar component for shadcn-ui-rs
//!
//! A circular avatar with fallback initials text.
//!
//! # Example
//!
//! ```rust
//! Avatar::new("MA")
//!     .size(AvatarSize::Lg)
//! ```

use gpui::prelude::*;
use gpui::{div, px, rems, App, FontWeight, IntoElement, SharedString, Window};

use crate::theme::Theme;

/// Avatar size preset.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AvatarSize {
    /// 32px diameter.
    Sm,
    /// 40px diameter.
    #[default]
    Default,
    /// 48px diameter.
    Lg,
}

/// A circular avatar component displaying fallback initials.
///
/// Renders as a circle with muted background and centered text.
#[derive(IntoElement)]
pub struct Avatar {
    fallback: SharedString,
    size: AvatarSize,
}

impl Avatar {
    /// Create a new avatar with the given fallback text (e.g. initials).
    pub fn new(fallback: impl Into<SharedString>) -> Self {
        Self {
            fallback: fallback.into(),
            size: AvatarSize::Default,
        }
    }

    /// Set the avatar size.
    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for Avatar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let (dimension, text_size) = match self.size {
            AvatarSize::Sm => (px(32.0), rems(0.75)),
            AvatarSize::Default => (px(40.0), rems(0.875)),
            AvatarSize::Lg => (px(48.0), rems(1.0)),
        };

        div()
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .overflow_hidden()
            .w(dimension)
            .h(dimension)
            .bg(colors.muted)
            .text_color(colors.muted_foreground)
            .text_size(text_size)
            .font_weight(FontWeight::MEDIUM)
            .flex_shrink_0()
            .child(self.fallback)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_defaults() {
        let avatar = Avatar::new("MA");
        assert_eq!(avatar.size, AvatarSize::Default);
        assert_eq!(avatar.fallback, SharedString::from("MA"));
    }

    #[test]
    fn test_avatar_sizes() {
        let avatar = Avatar::new("A").size(AvatarSize::Sm);
        assert_eq!(avatar.size, AvatarSize::Sm);

        let avatar = Avatar::new("A").size(AvatarSize::Default);
        assert_eq!(avatar.size, AvatarSize::Default);

        let avatar = Avatar::new("A").size(AvatarSize::Lg);
        assert_eq!(avatar.size, AvatarSize::Lg);
    }

    #[test]
    fn test_avatar_builder() {
        let avatar = Avatar::new("JD").size(AvatarSize::Lg);
        assert_eq!(avatar.fallback, SharedString::from("JD"));
        assert_eq!(avatar.size, AvatarSize::Lg);
    }
}
