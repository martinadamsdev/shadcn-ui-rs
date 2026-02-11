//! Separator component for shadcn-ui-rs
//!
//! A horizontal or vertical dividing line for visual separation of content.
//!
//! # Example
//!
//! ```rust
//! // Horizontal separator (default)
//! Separator::new()
//!
//! // Vertical separator
//! Separator::new()
//!     .orientation(SeparatorOrientation::Vertical)
//! ```

use gpui::{div, px, App, IntoElement, RenderOnce, Styled, Window};

use crate::theme::Theme;

/// Separator orientation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SeparatorOrientation {
    /// Horizontal line spanning full width.
    #[default]
    Horizontal,
    /// Vertical line spanning full height.
    Vertical,
}

/// A themed dividing line.
///
/// Renders as a thin line using the theme's border color.
#[derive(IntoElement)]
pub struct Separator {
    orientation: SeparatorOrientation,
}

impl Separator {
    /// Create a new horizontal separator.
    pub fn new() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
        }
    }

    /// Set the separator orientation.
    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }
}

impl RenderOnce for Separator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let el = div().flex_shrink_0().bg(colors.border);

        match self.orientation {
            SeparatorOrientation::Horizontal => el.w_full().h(px(1.0)),
            SeparatorOrientation::Vertical => el.h_full().w(px(1.0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separator_defaults() {
        let sep = Separator::new();
        assert_eq!(sep.orientation, SeparatorOrientation::Horizontal);
    }

    #[test]
    fn test_separator_vertical() {
        let sep = Separator::new().orientation(SeparatorOrientation::Vertical);
        assert_eq!(sep.orientation, SeparatorOrientation::Vertical);
    }
}
