//! Sheet component for shadcn-ui-rs
//!
//! A slide-in overlay panel from screen edge.
//!
//! # Example
//!
//! ```rust
//! // In a stateful view:
//! struct MyView {
//!     sheet_open: bool,
//! }
//!
//! impl Render for MyView {
//!     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!         let open = self.sheet_open;
//!         div()
//!             .child(
//!                 Button::new()
//!                     .on_click(cx.listener(|this, _event, _window, _cx| {
//!                         this.sheet_open = true;
//!                     }))
//!                     .child("Open Sheet")
//!             )
//!             .when(open, |el| {
//!                 el.child(
//!                     Sheet::new("my-sheet")
//!                         .open(true)
//!                         .side(SheetSide::Right)
//!                         .on_close(cx.listener(|this, _window, _cx| {
//!                             this.sheet_open = false;
//!                         }))
//!                         .child(
//!                             SheetContent::new()
//!                                 .child(
//!                                     SheetHeader::new()
//!                                         .child(SheetTitle::new("Edit Profile"))
//!                                         .child(SheetDescription::new(
//!                                             "Make changes to your profile here."
//!                                         ))
//!                                 )
//!                                 .child("Sheet body content")
//!                                 .child(
//!                                     SheetFooter::new()
//!                                         .child("Save changes")
//!                                 )
//!                         )
//!                 )
//!             })
//!     }
//! }
//! ```

use std::rc::Rc;

use gpui::{
    deferred, div, prelude::*, px, AnyElement, App, ClickEvent, Div, ElementId, FontWeight,
    IntoElement, KeyDownEvent, ParentElement, RenderOnce, SharedString, Stateful, Styled, Window,
};
use crate::theme::Theme;

/// The side of the screen from which the sheet slides in.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SheetSide {
    /// Sheet slides in from the top edge.
    Top,
    /// Sheet slides in from the right edge (default).
    #[default]
    Right,
    /// Sheet slides in from the bottom edge.
    Bottom,
    /// Sheet slides in from the left edge.
    Left,
}

/// Sheet root component.
///
/// A slide-in overlay panel from a screen edge. Uses GPUI's `deferred` element
/// to render on top of all other content with a semi-transparent backdrop.
///
/// Clicking the backdrop or pressing Escape closes the sheet.
#[derive(IntoElement)]
pub struct Sheet {
    id: ElementId,
    open: bool,
    side: SheetSide,
    #[allow(clippy::type_complexity)]
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl Sheet {
    /// Create a new sheet with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            side: SheetSide::default(),
            on_close: None,
            children: Vec::new(),
        }
    }

    /// Set the open state of the sheet.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set the side from which the sheet slides in.
    pub fn side(mut self, side: SheetSide) -> Self {
        self.side = side;
        self
    }

    /// Set the callback for when the sheet should close.
    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }
}

impl ParentElement for Sheet {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Sheet {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let backdrop_color = gpui::hsla(0.0, 0.0, 0.0, 0.8);
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let popover_bg = colors.popover;
        let popover_fg = colors.popover_foreground;
        let border_color = colors.border;
        let side = self.side;

        div().when(self.open, |el: Div| {
            el.child(
                deferred(
                    div()
                        .id(self.id)
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .bg(backdrop_color)
                        .when_some(self.on_close, |el: Stateful<Div>, on_close| {
                            let on_close: Rc<dyn Fn(&mut Window, &mut App)> = Rc::new(on_close);
                            let on_close_key = on_close.clone();
                            el.on_click({
                                move |_event: &ClickEvent, window: &mut Window, cx: &mut App| {
                                    on_close(window, cx);
                                }
                            })
                            .on_key_down(
                                move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
                                    if event.keystroke.key.as_str() == "escape" {
                                        on_close_key(window, cx);
                                    }
                                },
                            )
                        })
                        .child({
                            let mut inner = div()
                                .absolute()
                                .bg(popover_bg)
                                .text_color(popover_fg)
                                .shadow_lg();

                            inner = match side {
                                SheetSide::Right => inner
                                    .right_0()
                                    .top_0()
                                    .h_full()
                                    .w(px(350.0))
                                    .border_l_1()
                                    .border_color(border_color),
                                SheetSide::Left => inner
                                    .left_0()
                                    .top_0()
                                    .h_full()
                                    .w(px(350.0))
                                    .border_r_1()
                                    .border_color(border_color),
                                SheetSide::Top => inner
                                    .top_0()
                                    .left_0()
                                    .w_full()
                                    .h(px(300.0))
                                    .border_b_1()
                                    .border_color(border_color),
                                SheetSide::Bottom => inner
                                    .bottom_0()
                                    .left_0()
                                    .w_full()
                                    .h(px(300.0))
                                    .border_t_1()
                                    .border_color(border_color),
                            };

                            inner.children(self.children)
                        }),
                )
                .with_priority(100),
            )
        })
    }
}

/// Sheet content container.
///
/// The main content area of the sheet with scrollable overflow.
#[derive(IntoElement)]
pub struct SheetContent {
    children: Vec<AnyElement>,
}

impl SheetContent {
    /// Create a new sheet content container.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for SheetContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SheetContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        div()
            .id("sheet-content")
            .flex()
            .flex_col()
            .gap(px(16.0))
            .p(px(24.0))
            .h_full()
            .overflow_y_scroll()
            .bg(colors.popover)
            .text_color(colors.popover_foreground)
            // Prevent click-through to backdrop
            .on_click(|_event: &ClickEvent, _window: &mut Window, _cx: &mut App| {})
            .children(self.children)
    }
}

/// Sheet header section.
///
/// Contains the title and description with vertical spacing.
#[derive(IntoElement)]
pub struct SheetHeader {
    children: Vec<AnyElement>,
}

impl SheetHeader {
    /// Create a new sheet header.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for SheetHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SheetHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .children(self.children)
    }
}

/// Sheet title text.
///
/// Renders with semibold weight and larger text.
#[derive(IntoElement)]
pub struct SheetTitle {
    text: SharedString,
}

impl SheetTitle {
    /// Create a new sheet title.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for SheetTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .text_lg()
            .font_weight(FontWeight::SEMIBOLD)
            .line_height(gpui::rems(1.75))
            .child(self.text)
    }
}

/// Sheet description text.
///
/// Renders with muted foreground color and smaller text.
#[derive(IntoElement)]
pub struct SheetDescription {
    text: SharedString,
}

impl SheetDescription {
    /// Create a new sheet description.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for SheetDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .text_sm()
            .text_color(theme.colors.muted_foreground)
            .child(self.text)
    }
}

/// Sheet footer section.
///
/// A flex row at the bottom of the sheet for action buttons.
#[derive(IntoElement)]
pub struct SheetFooter {
    children: Vec<AnyElement>,
}

impl SheetFooter {
    /// Create a new sheet footer.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for SheetFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SheetFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .justify_end()
            .gap(px(8.0))
            .children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheet_defaults() {
        let sheet = Sheet::new("test-sheet");
        assert!(!sheet.open);
        assert_eq!(sheet.side, SheetSide::Right);
        assert!(sheet.on_close.is_none());
    }

    #[test]
    fn test_sheet_builder() {
        let sheet = Sheet::new("test-sheet")
            .open(true)
            .side(SheetSide::Left)
            .on_close(|_window, _cx| {});
        assert!(sheet.open);
        assert_eq!(sheet.side, SheetSide::Left);
        assert!(sheet.on_close.is_some());
    }

    #[test]
    fn test_sheet_side_variants() {
        let sheet_top = Sheet::new("top").side(SheetSide::Top);
        assert_eq!(sheet_top.side, SheetSide::Top);

        let sheet_right = Sheet::new("right").side(SheetSide::Right);
        assert_eq!(sheet_right.side, SheetSide::Right);

        let sheet_bottom = Sheet::new("bottom").side(SheetSide::Bottom);
        assert_eq!(sheet_bottom.side, SheetSide::Bottom);

        let sheet_left = Sheet::new("left").side(SheetSide::Left);
        assert_eq!(sheet_left.side, SheetSide::Left);
    }

    #[test]
    fn test_sheet_title() {
        let title = SheetTitle::new("Test Title");
        assert_eq!(title.text, SharedString::from("Test Title"));
    }

    #[test]
    fn test_sheet_description() {
        let desc = SheetDescription::new("Test description");
        assert_eq!(desc.text, SharedString::from("Test description"));
    }
}
