//! Progress component for shadcn-ui-rs
//!
//! A horizontal progress bar with a track and fill indicator.
//!
//! # Example
//!
//! ```rust
//! Progress::new(65.0)
//! ```

use gpui::prelude::*;
use gpui::{div, px, relative, App, IntoElement, Window};

use crate::theme::Theme;

/// A horizontal progress bar component.
///
/// Displays a track with a filled portion representing the current progress.
/// The value is clamped to the range 0.0..=100.0.
#[derive(IntoElement)]
pub struct Progress {
    value: f32,
}

impl Progress {
    /// Create a new progress bar with the given value (0.0 to 100.0).
    ///
    /// Values outside this range are clamped.
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 100.0),
        }
    }

    /// Set the progress value. Clamped to 0.0..=100.0.
    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(0.0, 100.0);
        self
    }
}

impl RenderOnce for Progress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let fraction = self.value / 100.0;

        // Outer track
        div()
            .w_full()
            .h(px(8.0))
            .rounded_full()
            .bg(colors.secondary)
            .overflow_hidden()
            // Inner fill
            .child(
                div()
                    .h_full()
                    .rounded_full()
                    .bg(colors.primary)
                    .w(relative(fraction)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_defaults() {
        let progress = Progress::new(0.0);
        assert_eq!(progress.value, 0.0);
    }

    #[test]
    fn test_progress_value() {
        let progress = Progress::new(65.0);
        assert_eq!(progress.value, 65.0);

        let progress = progress.value(80.0);
        assert_eq!(progress.value, 80.0);
    }

    #[test]
    fn test_progress_clamp() {
        let progress = Progress::new(150.0);
        assert_eq!(progress.value, 100.0);

        let progress = Progress::new(-10.0);
        assert_eq!(progress.value, 0.0);

        let progress = Progress::new(50.0).value(200.0);
        assert_eq!(progress.value, 100.0);

        let progress = Progress::new(50.0).value(-50.0);
        assert_eq!(progress.value, 0.0);
    }
}
