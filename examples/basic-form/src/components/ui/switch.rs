//! Switch (toggle) component for shadcn-ui-rs
//!
//! A binary on/off control styled as a sliding toggle.
//!
//! # Example
//! ```rust
//! Switch::new("notifications")
//!     .checked(true)
//!     .on_change(|checked, _window, _cx| {
//!         println!("Switch is now: {checked}");
//!     })
//! ```

use std::rc::Rc;

use gpui::prelude::*;
use gpui::{App, ClickEvent, Div, ElementId, IntoElement, Stateful, Window, div, px};

use crate::theme::Theme;

/// Track width and height constants (in px).
const TRACK_WIDTH: f32 = 44.0;
const TRACK_HEIGHT: f32 = 24.0;
const THUMB_SIZE: f32 = 20.0;
const THUMB_PADDING: f32 = 2.0;

// ---------------------------------------------------------------------------
// Switch
// ---------------------------------------------------------------------------

/// A toggle switch that can be either on or off.
///
/// This is a controlled component: the `checked` state is passed in, and
/// changes are communicated through the `on_change` callback.
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    checked: bool,
    disabled: bool,
    #[allow(clippy::type_complexity)]
    on_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl Switch {
    /// Create a new switch with the given element id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            disabled: false,
            on_change: None,
        }
    }

    /// Set the checked (on) state.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Disable the switch so it cannot be toggled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Register a callback for when the switch is toggled.
    /// The callback receives the new checked state.
    pub fn on_change(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Switch {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let primary = theme.colors.primary;
        let muted = theme.colors.muted;
        let background = theme.colors.background;
        let checked = self.checked;
        let disabled = self.disabled;
        let on_change = self.on_change;

        // Track background: primary when on, muted when off
        let track_bg = if checked { primary } else { muted };

        // Thumb offset: left when off, right when on
        let thumb_offset = if checked {
            TRACK_WIDTH - THUMB_SIZE - THUMB_PADDING
        } else {
            THUMB_PADDING
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .w(px(TRACK_WIDTH))
            .h(px(TRACK_HEIGHT))
            .rounded(px(TRACK_HEIGHT / 2.0))
            .bg(track_bg)
            .when(!disabled, |el: Stateful<Div>| el.cursor_pointer())
            .when(disabled, |el: Stateful<Div>| el.opacity(0.5))
            .when(!disabled, |el: Stateful<Div>| {
                el.on_click(move |_event: &ClickEvent, window, cx| {
                    if let Some(ref handler) = on_change {
                        handler(!checked, window, cx);
                    }
                })
            })
            .child(
                // Thumb
                div()
                    .size(px(THUMB_SIZE))
                    .rounded(px(THUMB_SIZE / 2.0))
                    .bg(background)
                    .ml(px(thumb_offset)),
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
    fn test_switch_defaults() {
        let switch = Switch::new("test");
        assert!(!switch.checked);
        assert!(!switch.disabled);
        assert!(switch.on_change.is_none());
    }

    #[test]
    fn test_switch_checked() {
        let switch = Switch::new("test").checked(true);
        assert!(switch.checked);
    }

    #[test]
    fn test_switch_disabled() {
        let switch = Switch::new("test").disabled(true);
        assert!(switch.disabled);
    }

    #[test]
    fn test_switch_with_callback() {
        let switch = Switch::new("test").on_change(|_checked, _window, _cx| {});
        assert!(switch.on_change.is_some());
    }

    #[test]
    fn test_thumb_position() {
        // When off, thumb is at the left
        let off_offset = THUMB_PADDING;
        assert!(off_offset < TRACK_WIDTH / 2.0);

        // When on, thumb is at the right
        let on_offset = TRACK_WIDTH - THUMB_SIZE - THUMB_PADDING;
        assert!(on_offset >= TRACK_WIDTH / 2.0);
    }
}
