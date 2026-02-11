//! Badge component for shadcn-ui-rs
//!
//! An inline status label with multiple visual variants.
//!
//! # Example
//!
//! ```rust
//! Badge::new("New")
//!     .variant(BadgeVariant::Destructive)
//! ```

use gpui::prelude::*;
use gpui::{div, px, rems, App, FontWeight, IntoElement, SharedString, Window};

use crate::theme::Theme;

/// Badge visual variant.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BadgeVariant {
    /// Primary background with primary foreground text.
    #[default]
    Default,
    /// Secondary background with secondary foreground text.
    Secondary,
    /// Transparent background with border and foreground text.
    Outline,
    /// Destructive background with destructive foreground text.
    Destructive,
}

/// An inline badge component for displaying short status labels.
///
/// Renders as a pill-shaped container with themed colors based on the variant.
#[derive(IntoElement)]
pub struct Badge {
    label: SharedString,
    variant: BadgeVariant,
}

impl Badge {
    /// Create a new badge with the given label text.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            variant: BadgeVariant::Default,
        }
    }

    /// Set the badge variant.
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let (bg, fg, has_border) = match self.variant {
            BadgeVariant::Default => (colors.primary, colors.primary_foreground, false),
            BadgeVariant::Secondary => (colors.secondary, colors.secondary_foreground, false),
            BadgeVariant::Outline => (
                gpui::Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.0,
                    a: 0.0,
                },
                colors.foreground,
                true,
            ),
            BadgeVariant::Destructive => {
                (colors.destructive, colors.destructive_foreground, false)
            }
        };

        let mut el = div()
            .flex()
            .items_center()
            .rounded_full()
            .px(px(10.0))
            .py(px(2.0))
            .text_size(rems(0.75))
            .font_weight(FontWeight::SEMIBOLD)
            .bg(bg)
            .text_color(fg);

        if has_border {
            el = el.border_1().border_color(colors.border);
        }

        el.child(self.label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_defaults() {
        let badge = Badge::new("New");
        assert_eq!(badge.variant, BadgeVariant::Default);
        assert_eq!(badge.label, SharedString::from("New"));
    }

    #[test]
    fn test_badge_variants() {
        let badge = Badge::new("Test").variant(BadgeVariant::Secondary);
        assert_eq!(badge.variant, BadgeVariant::Secondary);

        let badge = Badge::new("Test").variant(BadgeVariant::Outline);
        assert_eq!(badge.variant, BadgeVariant::Outline);

        let badge = Badge::new("Test").variant(BadgeVariant::Destructive);
        assert_eq!(badge.variant, BadgeVariant::Destructive);
    }

    #[test]
    fn test_badge_builder() {
        let badge = Badge::new("Status").variant(BadgeVariant::Destructive);
        assert_eq!(badge.label, SharedString::from("Status"));
        assert_eq!(badge.variant, BadgeVariant::Destructive);
    }
}
