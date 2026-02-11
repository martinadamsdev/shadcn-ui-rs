//! Skeleton component for shadcn-ui-rs
//!
//! A static loading placeholder block with optional dimensions and shape.
//!
//! # Example
//!
//! ```rust
//! Skeleton::new()
//!     .width(px(200.0))
//!     .height(px(20.0))
//!
//! // Circular skeleton for avatar placeholder
//! Skeleton::new()
//!     .width(px(40.0))
//!     .height(px(40.0))
//!     .rounded(true)
//! ```

use gpui::prelude::*;
use gpui::{div, App, IntoElement, Pixels, Window};

use crate::theme::{Radius, Theme};

/// A static skeleton placeholder component.
///
/// Displays a muted-colored block as a loading placeholder. Width and height
/// can be specified explicitly, or the skeleton fills its parent container.
/// Use `rounded(true)` for circular placeholders (e.g. avatar loading state).
#[derive(IntoElement)]
pub struct Skeleton {
    width: Option<Pixels>,
    height: Option<Pixels>,
    rounded: bool,
}

impl Skeleton {
    /// Create a new skeleton placeholder.
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            rounded: false,
        }
    }

    /// Set the width of the skeleton.
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height of the skeleton.
    pub fn height(mut self, height: Pixels) -> Self {
        self.height = Some(height);
        self
    }

    /// Set whether the skeleton uses full rounding (circular).
    pub fn rounded(mut self, rounded: bool) -> Self {
        self.rounded = rounded;
        self
    }
}

impl RenderOnce for Skeleton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;
        let radius = theme.radius;

        let mut el = div().bg(colors.muted);

        if let Some(w) = self.width {
            el = el.w(w);
        } else {
            el = el.w_full();
        }

        if let Some(h) = self.height {
            el = el.h(h);
        }

        if self.rounded {
            el = el.rounded_full();
        } else {
            el = match radius {
                Radius::None => el.rounded_none(),
                Radius::Sm => el.rounded_sm(),
                Radius::Md => el.rounded_md(),
                Radius::Lg => el.rounded_lg(),
                Radius::Full => el.rounded_full(),
            };
        }

        el
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::px;

    #[test]
    fn test_skeleton_defaults() {
        let skeleton = Skeleton::new();
        assert!(skeleton.width.is_none());
        assert!(skeleton.height.is_none());
        assert!(!skeleton.rounded);
    }

    #[test]
    fn test_skeleton_builder() {
        let skeleton = Skeleton::new()
            .width(px(200.0))
            .height(px(20.0))
            .rounded(true);
        assert_eq!(skeleton.width, Some(px(200.0)));
        assert_eq!(skeleton.height, Some(px(20.0)));
        assert!(skeleton.rounded);
    }

    #[test]
    fn test_skeleton_rounded() {
        let skeleton = Skeleton::new().rounded(true);
        assert!(skeleton.rounded);

        let skeleton = Skeleton::new().rounded(false);
        assert!(!skeleton.rounded);
    }
}
