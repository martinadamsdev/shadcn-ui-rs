//! Popover component for shadcn-ui-rs
//!
//! A click-triggered overlay with arbitrary content.
//!
//! # Example
//!
//! ```rust
//! Popover::new("my-popover")
//!     .open(is_open)
//!     .side(PopoverSide::Bottom)
//!     .on_open_change(|open, _window, _cx| {
//!         // Toggle state
//!     })
//!     .trigger(Button::new("Click me"))
//!     .content(div().child("Popover content"))
//! ```

use std::rc::Rc;

use gpui::{
    deferred, div, px, AnyElement, App, ClickEvent, ElementId, IntoElement, KeyDownEvent,
    MouseDownEvent, ParentElement, RenderOnce, SharedString, Styled, Window,
};

use crate::theme::Theme;

/// Side on which the popover appears relative to the trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverSide {
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

/// Alignment of the popover relative to the trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverAlign {
    /// Align to the start edge.
    Start,
    /// Align to the center.
    #[default]
    Center,
    /// Align to the end edge.
    End,
}

/// A click-triggered overlay with arbitrary content.
///
/// Uses separate `.trigger()` and `.content()` methods to distinguish
/// the trigger element from the overlay content. The `open` state is
/// controlled externally and communicated via `on_open_change`.
#[derive(IntoElement)]
pub struct Popover {
    id: ElementId,
    open: bool,
    side: PopoverSide,
    align: PopoverAlign,
    #[allow(clippy::type_complexity)]
    on_open_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
    trigger: Vec<AnyElement>,
    content: Vec<AnyElement>,
}

impl Popover {
    /// Create a new popover with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            side: PopoverSide::default(),
            align: PopoverAlign::default(),
            on_open_change: None,
            trigger: Vec::new(),
            content: Vec::new(),
        }
    }

    /// Set the open state.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set which side the popover appears on.
    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    /// Set the alignment of the popover.
    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }

    /// Set the callback for when the open state should change.
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }

    /// Add a trigger element.
    pub fn trigger(mut self, element: impl IntoElement) -> Self {
        self.trigger.push(element.into_any_element());
        self
    }

    /// Add content to display in the popover overlay.
    pub fn content(mut self, element: impl IntoElement) -> Self {
        self.content.push(element.into_any_element());
        self
    }
}

impl RenderOnce for Popover {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let popover_bg = colors.popover;
        let popover_fg = colors.popover_foreground;
        let border_color = colors.border;

        let open = self.open;
        let on_open_change = self.on_open_change;

        div()
            .relative()
            .child(
                div()
                    .id(self.id.clone())
                    .cursor_pointer()
                    .when_some(on_open_change.clone(), move |el, on_open_change| {
                        el.on_click(move |_event: &ClickEvent, window: &mut Window, cx: &mut App| {
                            on_open_change(!open, window, cx);
                        })
                    })
                    .children(self.trigger),
            )
            .when(open, |el| {
                let overlay = div()
                    .id(ElementId::from(SharedString::from("popover-content")))
                    .absolute()
                    .bg(popover_bg)
                    .text_color(popover_fg)
                    .border_1()
                    .border_color(border_color)
                    .rounded_md()
                    .shadow_lg()
                    .p(px(16.0))
                    .min_w(px(220.0))
                    .when_some(on_open_change.clone(), |el, on_open_change| {
                        let on_open_change_key = on_open_change.clone();
                        el.on_mouse_down_out(
                            move |_event: &MouseDownEvent, window: &mut Window, cx: &mut App| {
                                on_open_change(false, window, cx);
                            },
                        )
                        .on_key_down(
                            move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
                                if event.keystroke.key.as_str() == "escape" {
                                    on_open_change_key(false, window, cx);
                                }
                            },
                        )
                    })
                    .children(self.content);

                let positioned = match self.side {
                    PopoverSide::Top => overlay.bottom_full().mb(px(4.0)),
                    PopoverSide::Bottom => overlay.top_full().mt(px(4.0)),
                    PopoverSide::Left => overlay.right_full().mr(px(4.0)),
                    PopoverSide::Right => overlay.left_full().ml(px(4.0)),
                };

                el.child(deferred(positioned).with_priority(200))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popover_defaults() {
        let popover = Popover::new("test");
        assert_eq!(popover.side, PopoverSide::Bottom);
        assert_eq!(popover.align, PopoverAlign::Center);
        assert!(!popover.open);
        assert!(popover.on_open_change.is_none());
        assert!(popover.trigger.is_empty());
        assert!(popover.content.is_empty());
    }

    #[test]
    fn test_popover_builder() {
        let popover = Popover::new("test")
            .open(true)
            .side(PopoverSide::Top)
            .align(PopoverAlign::Start)
            .on_open_change(|_open, _window, _cx| {});
        assert!(popover.open);
        assert_eq!(popover.side, PopoverSide::Top);
        assert_eq!(popover.align, PopoverAlign::Start);
        assert!(popover.on_open_change.is_some());
    }

    #[test]
    fn test_popover_side_top() {
        let popover = Popover::new("test").side(PopoverSide::Top);
        assert_eq!(popover.side, PopoverSide::Top);
    }

    #[test]
    fn test_popover_side_right() {
        let popover = Popover::new("test").side(PopoverSide::Right);
        assert_eq!(popover.side, PopoverSide::Right);
    }

    #[test]
    fn test_popover_side_bottom() {
        let popover = Popover::new("test").side(PopoverSide::Bottom);
        assert_eq!(popover.side, PopoverSide::Bottom);
    }

    #[test]
    fn test_popover_side_left() {
        let popover = Popover::new("test").side(PopoverSide::Left);
        assert_eq!(popover.side, PopoverSide::Left);
    }

    #[test]
    fn test_popover_align_variants() {
        assert_eq!(
            Popover::new("test").align(PopoverAlign::Start).align,
            PopoverAlign::Start
        );
        assert_eq!(
            Popover::new("test").align(PopoverAlign::Center).align,
            PopoverAlign::Center
        );
        assert_eq!(
            Popover::new("test").align(PopoverAlign::End).align,
            PopoverAlign::End
        );
    }
}
