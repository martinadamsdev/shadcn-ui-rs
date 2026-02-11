//! HoverCard component for shadcn-ui-rs
//!
//! A hover-triggered card overlay with rich content.
//!
//! # Example
//!
//! ```rust
//! HoverCard::new("my-hover-card")
//!     .open(is_hovered)
//!     .side(HoverCardSide::Bottom)
//!     .trigger(div().child("Hover over me"))
//!     .content(div().child("Rich content here"))
//! ```

use gpui::{
    deferred, div, px, AnyElement, App, ElementId, IntoElement, ParentElement, RenderOnce,
    SharedString, Styled, Window,
};

use crate::theme::Theme;

/// Side on which the hover card appears relative to the trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HoverCardSide {
    /// Display above the trigger.
    Top,
    /// Display to the right of the trigger.
    Right,
    /// Display below the trigger.
    #[default]
    Bottom,
    /// Display to the left of the trigger.
    Left,
}

/// A hover-triggered card overlay with rich content.
///
/// Similar to Tooltip but accepts arbitrary children instead of text-only.
/// The `open` state is controlled externally (e.g., via hover state in the
/// parent view). Uses separate `.trigger()` and `.content()` methods.
#[derive(IntoElement)]
pub struct HoverCard {
    id: ElementId,
    open: bool,
    side: HoverCardSide,
    trigger: Vec<AnyElement>,
    content: Vec<AnyElement>,
}

impl HoverCard {
    /// Create a new hover card with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            side: HoverCardSide::default(),
            trigger: Vec::new(),
            content: Vec::new(),
        }
    }

    /// Set the open state.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set which side the hover card appears on.
    pub fn side(mut self, side: HoverCardSide) -> Self {
        self.side = side;
        self
    }

    /// Add a trigger element.
    pub fn trigger(mut self, element: impl IntoElement) -> Self {
        self.trigger.push(element.into_any_element());
        self
    }

    /// Add content to display in the hover card overlay.
    pub fn content(mut self, element: impl IntoElement) -> Self {
        self.content.push(element.into_any_element());
        self
    }
}

impl RenderOnce for HoverCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let popover_bg = colors.popover;
        let popover_fg = colors.popover_foreground;
        let border_color = colors.border;

        div()
            .id(self.id)
            .relative()
            .children(self.trigger)
            .when(self.open, |el| {
                let overlay = div()
                    .id(ElementId::from(SharedString::from("hover-card-content")))
                    .absolute()
                    .bg(popover_bg)
                    .text_color(popover_fg)
                    .border_1()
                    .border_color(border_color)
                    .rounded_md()
                    .shadow_md()
                    .p(px(16.0))
                    .w(px(300.0))
                    .children(self.content);

                let positioned = match self.side {
                    HoverCardSide::Top => overlay.bottom_full().mb(px(4.0)),
                    HoverCardSide::Bottom => overlay.top_full().mt(px(4.0)),
                    HoverCardSide::Left => overlay.right_full().mr(px(4.0)),
                    HoverCardSide::Right => overlay.left_full().ml(px(4.0)),
                };

                el.child(deferred(positioned).with_priority(200))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hover_card_defaults() {
        let card = HoverCard::new("test");
        assert!(!card.open);
        assert_eq!(card.side, HoverCardSide::Bottom);
        assert!(card.trigger.is_empty());
        assert!(card.content.is_empty());
    }

    #[test]
    fn test_hover_card_builder() {
        let card = HoverCard::new("test")
            .open(true)
            .side(HoverCardSide::Top);
        assert!(card.open);
        assert_eq!(card.side, HoverCardSide::Top);
    }

    #[test]
    fn test_hover_card_side_top() {
        let card = HoverCard::new("test").side(HoverCardSide::Top);
        assert_eq!(card.side, HoverCardSide::Top);
    }

    #[test]
    fn test_hover_card_side_right() {
        let card = HoverCard::new("test").side(HoverCardSide::Right);
        assert_eq!(card.side, HoverCardSide::Right);
    }

    #[test]
    fn test_hover_card_side_bottom() {
        let card = HoverCard::new("test").side(HoverCardSide::Bottom);
        assert_eq!(card.side, HoverCardSide::Bottom);
    }

    #[test]
    fn test_hover_card_side_left() {
        let card = HoverCard::new("test").side(HoverCardSide::Left);
        assert_eq!(card.side, HoverCardSide::Left);
    }
}
