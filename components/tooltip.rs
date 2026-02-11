//! Tooltip component for shadcn-ui-rs
//!
//! A hover-triggered text overlay that displays additional information.
//!
//! # Example
//!
//! ```rust
//! Tooltip::new("my-tooltip")
//!     .text("This is a tooltip")
//!     .side(TooltipSide::Top)
//!     .open(is_hovered)
//!     .child(Button::new("Hover me"))
//! ```

use gpui::{
    deferred, div, px, AnyElement, App, ElementId, IntoElement, ParentElement, RenderOnce,
    SharedString, Styled, Window,
};

use crate::theme::Theme;

/// Side on which the tooltip appears relative to the trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipSide {
    /// Display above the trigger.
    #[default]
    Top,
    /// Display to the right of the trigger.
    Right,
    /// Display below the trigger.
    Bottom,
    /// Display to the left of the trigger.
    Left,
}

/// A hover-triggered text overlay.
///
/// Wraps trigger children in a relative container and renders a positioned
/// overlay with text content when open. The `open` state is controlled
/// externally (e.g., via hover state in the parent view).
#[derive(IntoElement)]
pub struct Tooltip {
    id: ElementId,
    text: SharedString,
    side: TooltipSide,
    open: bool,
    children: Vec<AnyElement>,
}

impl Tooltip {
    /// Create a new tooltip with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            text: SharedString::default(),
            side: TooltipSide::default(),
            open: false,
            children: Vec::new(),
        }
    }

    /// Set the tooltip text content.
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = text.into();
        self
    }

    /// Set which side the tooltip appears on.
    pub fn side(mut self, side: TooltipSide) -> Self {
        self.side = side;
        self
    }

    /// Set the open state of the tooltip.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl ParentElement for Tooltip {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Tooltip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let popover_bg = colors.popover;
        let popover_fg = colors.popover_foreground;
        let border_color = colors.border;

        div()
            .id(self.id)
            .relative()
            .children(self.children)
            .when(self.open, |el| {
                let overlay = div()
                    .absolute()
                    .bg(popover_bg)
                    .text_color(popover_fg)
                    .border_1()
                    .border_color(border_color)
                    .rounded_md()
                    .shadow_md()
                    .px(px(12.0))
                    .py(px(6.0))
                    .text_sm()
                    .whitespace_nowrap()
                    .child(self.text);

                let positioned = match self.side {
                    TooltipSide::Top => overlay.bottom_full().mb(px(4.0)),
                    TooltipSide::Bottom => overlay.top_full().mt(px(4.0)),
                    TooltipSide::Left => overlay.right_full().mr(px(4.0)),
                    TooltipSide::Right => overlay.left_full().ml(px(4.0)),
                };

                el.child(deferred(positioned).with_priority(200))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_defaults() {
        let tooltip = Tooltip::new("test");
        assert_eq!(tooltip.side, TooltipSide::Top);
        assert!(!tooltip.open);
        assert_eq!(tooltip.text, SharedString::default());
    }

    #[test]
    fn test_tooltip_builder() {
        let tooltip = Tooltip::new("test")
            .text("Hello")
            .side(TooltipSide::Bottom)
            .open(true);
        assert_eq!(tooltip.text, SharedString::from("Hello"));
        assert_eq!(tooltip.side, TooltipSide::Bottom);
        assert!(tooltip.open);
    }

    #[test]
    fn test_tooltip_side_top() {
        let tooltip = Tooltip::new("test").side(TooltipSide::Top);
        assert_eq!(tooltip.side, TooltipSide::Top);
    }

    #[test]
    fn test_tooltip_side_right() {
        let tooltip = Tooltip::new("test").side(TooltipSide::Right);
        assert_eq!(tooltip.side, TooltipSide::Right);
    }

    #[test]
    fn test_tooltip_side_bottom() {
        let tooltip = Tooltip::new("test").side(TooltipSide::Bottom);
        assert_eq!(tooltip.side, TooltipSide::Bottom);
    }

    #[test]
    fn test_tooltip_side_left() {
        let tooltip = Tooltip::new("test").side(TooltipSide::Left);
        assert_eq!(tooltip.side, TooltipSide::Left);
    }
}
