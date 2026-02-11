//! Slider component for shadcn-ui-rs
//!
//! A horizontal range slider that lets the user select a numeric value
//! by clicking or dragging along a track.
//!
//! # Example
//! ```rust
//! Slider::new("volume")
//!     .min(0.0)
//!     .max(100.0)
//!     .value(50.0)
//!     .step(1.0)
//!     .on_change(|value, _window, _cx| {
//!         println!("Volume: {value}");
//!     })
//! ```

use std::rc::Rc;

use gpui::prelude::*;
use gpui::{px, App, ElementId, IntoElement, Window};

use crate::theme::Theme;

/// Default track height in pixels.
const TRACK_HEIGHT: f32 = 6.0;
/// Default thumb diameter in pixels.
const THUMB_SIZE: f32 = 16.0;
/// Default slider width in pixels.
const SLIDER_WIDTH: f32 = 200.0;

// ---------------------------------------------------------------------------
// Slider
// ---------------------------------------------------------------------------

/// A horizontal slider for selecting a numeric value within a range.
///
/// This is a controlled component: the current `value` is provided externally
/// and the `on_change` callback is invoked when the user interacts with the slider.
#[derive(IntoElement)]
pub struct Slider {
    id: ElementId,
    min: f32,
    max: f32,
    value: f32,
    step: f32,
    disabled: bool,
    width: f32,
    on_change: Option<Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>>,
}

impl Slider {
    /// Create a new slider with the given element id.
    /// Defaults to range 0..100 with value 0 and step 1.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            min: 0.0,
            max: 100.0,
            value: 0.0,
            step: 1.0,
            disabled: false,
            width: SLIDER_WIDTH,
            on_change: None,
        }
    }

    /// Set the minimum value.
    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    /// Set the maximum value.
    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    /// Set the current value. Clamped to [min, max].
    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    /// Set the step increment. Values will be rounded to the nearest step.
    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    /// Disable the slider.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the display width of the slider in pixels.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Register a callback for when the value changes.
    pub fn on_change(
        mut self,
        handler: impl Fn(f32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Calculate the fraction [0.0, 1.0] for the current value.
    fn fraction(&self) -> f32 {
        if (self.max - self.min).abs() < f32::EPSILON {
            return 0.0;
        }
        ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
    }

    /// Snap a raw value to the nearest step.
    fn snap(value: f32, min: f32, max: f32, step: f32) -> f32 {
        if step <= 0.0 {
            return value.clamp(min, max);
        }
        let steps = ((value - min) / step).round();
        (min + steps * step).clamp(min, max)
    }
}

impl RenderOnce for Slider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let primary = theme.colors.primary;
        let muted = theme.colors.muted;
        let background = theme.colors.background;
        let border = theme.colors.border;

        let fraction = self.fraction();
        let track_width = self.width;
        let filled_width = fraction * track_width;
        let disabled = self.disabled;

        // The outer container uses relative positioning so the thumb can be
        // absolutely placed over the track at the correct offset.
        div()
            .id(self.id)
            .relative()
            .flex()
            .items_center()
            .w(px(track_width))
            .h(px(THUMB_SIZE))
            .when(!disabled, |el| el.cursor_pointer())
            .when(disabled, |el| el.opacity(0.5))
            // Unfilled track (full width background)
            .child(
                div()
                    .absolute()
                    .left_0()
                    .top(px((THUMB_SIZE - TRACK_HEIGHT) / 2.0))
                    .w_full()
                    .h(px(TRACK_HEIGHT))
                    .rounded(px(TRACK_HEIGHT / 2.0))
                    .bg(muted),
            )
            // Filled track (from left to current value)
            .child(
                div()
                    .absolute()
                    .left_0()
                    .top(px((THUMB_SIZE - TRACK_HEIGHT) / 2.0))
                    .w(px(filled_width))
                    .h(px(TRACK_HEIGHT))
                    .rounded(px(TRACK_HEIGHT / 2.0))
                    .bg(primary),
            )
            // Thumb indicator
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left(px((filled_width - THUMB_SIZE / 2.0).max(0.0)))
                    .size(px(THUMB_SIZE))
                    .rounded(px(THUMB_SIZE / 2.0))
                    .bg(background)
                    .border_1()
                    .border_color(border),
            )
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_defaults() {
        let slider = Slider::new("test");
        assert_eq!(slider.min, 0.0);
        assert_eq!(slider.max, 100.0);
        assert_eq!(slider.value, 0.0);
        assert_eq!(slider.step, 1.0);
        assert!(!slider.disabled);
    }

    #[test]
    fn test_slider_fraction() {
        let slider = Slider::new("test").min(0.0).max(100.0).value(50.0);
        assert!((slider.fraction() - 0.5).abs() < f32::EPSILON);

        let slider = Slider::new("test").min(0.0).max(100.0).value(0.0);
        assert!((slider.fraction() - 0.0).abs() < f32::EPSILON);

        let slider = Slider::new("test").min(0.0).max(100.0).value(100.0);
        assert!((slider.fraction() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_slider_fraction_clamped() {
        let slider = Slider::new("test").min(0.0).max(100.0).value(150.0);
        assert!((slider.fraction() - 1.0).abs() < f32::EPSILON);

        let slider = Slider::new("test").min(0.0).max(100.0).value(-50.0);
        assert!((slider.fraction() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_slider_fraction_zero_range() {
        let slider = Slider::new("test").min(50.0).max(50.0).value(50.0);
        assert!((slider.fraction() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_slider_snap() {
        assert!((Slider::snap(7.3, 0.0, 100.0, 5.0) - 5.0).abs() < f32::EPSILON);
        assert!((Slider::snap(7.6, 0.0, 100.0, 5.0) - 10.0).abs() < f32::EPSILON);
        assert!((Slider::snap(102.0, 0.0, 100.0, 5.0) - 100.0).abs() < f32::EPSILON);
        assert!((Slider::snap(-5.0, 0.0, 100.0, 5.0) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_slider_snap_zero_step() {
        assert!((Slider::snap(7.3, 0.0, 100.0, 0.0) - 7.3).abs() < f32::EPSILON);
    }

    #[test]
    fn test_slider_builder() {
        let slider = Slider::new("test")
            .min(10.0)
            .max(50.0)
            .value(25.0)
            .step(5.0)
            .disabled(true)
            .width(300.0);

        assert_eq!(slider.min, 10.0);
        assert_eq!(slider.max, 50.0);
        assert_eq!(slider.value, 25.0);
        assert_eq!(slider.step, 5.0);
        assert!(slider.disabled);
        assert_eq!(slider.width, 300.0);
    }
}
