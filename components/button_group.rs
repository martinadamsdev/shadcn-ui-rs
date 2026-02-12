//! ButtonGroup component for shadcn-ui-rs
//!
//! A container that visually groups buttons with connected borders.
//!
//! # Example
//!
//! ```rust
//! ButtonGroup::new()
//!     .child(Button::new("Left"))
//!     .child(Button::new("Center"))
//!     .child(Button::new("Right"))
//! ```

use gpui::prelude::*;
use gpui::{div, px, AnyElement, App, IntoElement, ParentElement, RenderOnce, Window};

/// Orientation of the button group.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonGroupOrientation {
    /// Buttons arranged in a horizontal row (default).
    #[default]
    Horizontal,
    /// Buttons arranged in a vertical column.
    Vertical,
}

/// A container that visually groups child elements (typically buttons).
///
/// Uses negative gap to overlap borders for a connected appearance.
#[derive(IntoElement)]
pub struct ButtonGroup {
    orientation: ButtonGroupOrientation,
    children: Vec<AnyElement>,
}

impl ButtonGroup {
    /// Create a new button group with horizontal orientation.
    pub fn new() -> Self {
        Self {
            orientation: ButtonGroupOrientation::Horizontal,
            children: Vec::new(),
        }
    }

    /// Set the button group orientation.
    pub fn orientation(mut self, orientation: ButtonGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }
}

impl ParentElement for ButtonGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut el = div().flex().gap(px(-1.0));

        el = match self.orientation {
            ButtonGroupOrientation::Horizontal => el.flex_row(),
            ButtonGroupOrientation::Vertical => el.flex_col(),
        };

        el.children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_group_defaults() {
        let group = ButtonGroup::new();
        assert_eq!(group.orientation, ButtonGroupOrientation::Horizontal);
        assert!(group.children.is_empty());
    }

    #[test]
    fn test_button_group_vertical() {
        let group = ButtonGroup::new().orientation(ButtonGroupOrientation::Vertical);
        assert_eq!(group.orientation, ButtonGroupOrientation::Vertical);
    }

    #[test]
    fn test_button_group_builder() {
        let group = ButtonGroup::new()
            .orientation(ButtonGroupOrientation::Horizontal)
            .child(div().child("A"))
            .child(div().child("B"));
        assert_eq!(group.children.len(), 2);
    }
}
