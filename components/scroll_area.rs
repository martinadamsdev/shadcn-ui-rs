//! ScrollArea component for shadcn-ui-rs
//!
//! A scrollable container with configurable orientation.
//!
//! # Example
//!
//! ```rust
//! ScrollArea::new()
//!     .max_height(px(300.0))
//!     .child(long_content)
//!
//! ScrollArea::new()
//!     .orientation(ScrollOrientation::Horizontal)
//!     .child(wide_content)
//! ```

use gpui::{div, AnyElement, App, IntoElement, ParentElement, Pixels, RenderOnce, Styled, Window};

/// Scroll orientation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ScrollOrientation {
    /// Vertical scrolling only.
    #[default]
    Vertical,
    /// Horizontal scrolling only.
    Horizontal,
    /// Both vertical and horizontal scrolling.
    Both,
}

/// A themed scrollable container.
///
/// Wraps content with overflow scrolling based on the selected orientation.
#[derive(IntoElement)]
pub struct ScrollArea {
    orientation: ScrollOrientation,
    max_height: Option<Pixels>,
    children: Vec<AnyElement>,
}

impl ScrollArea {
    /// Create a new vertical scroll area.
    pub fn new() -> Self {
        Self {
            orientation: ScrollOrientation::Vertical,
            max_height: None,
            children: Vec::new(),
        }
    }

    /// Set the scroll orientation.
    pub fn orientation(mut self, orientation: ScrollOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the maximum height of the scroll area.
    pub fn max_height(mut self, max_height: Pixels) -> Self {
        self.max_height = Some(max_height);
        self
    }
}

impl ParentElement for ScrollArea {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ScrollArea {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut el = div().relative();

        el = match self.orientation {
            ScrollOrientation::Vertical => el.overflow_y_scroll(),
            ScrollOrientation::Horizontal => el.overflow_x_scroll(),
            ScrollOrientation::Both => el.overflow_scroll(),
        };

        if let Some(max_h) = self.max_height {
            el = el.max_h(max_h);
        }

        el.children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::px;

    #[test]
    fn test_scroll_area_defaults() {
        let area = ScrollArea::new();
        assert_eq!(area.orientation, ScrollOrientation::Vertical);
        assert!(area.max_height.is_none());
        assert!(area.children.is_empty());
    }

    #[test]
    fn test_scroll_area_builder() {
        let area = ScrollArea::new()
            .orientation(ScrollOrientation::Both)
            .max_height(px(300.0));
        assert_eq!(area.orientation, ScrollOrientation::Both);
        assert_eq!(area.max_height, Some(px(300.0)));
    }

    #[test]
    fn test_scroll_area_horizontal() {
        let area = ScrollArea::new().orientation(ScrollOrientation::Horizontal);
        assert_eq!(area.orientation, ScrollOrientation::Horizontal);
    }
}
