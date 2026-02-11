//! Card component for shadcn-ui-rs
//!
//! A container with header, content, and footer sections.
//!
//! # Example
//!
//! ```rust
//! Card::new()
//!     .child(
//!         CardHeader::new()
//!             .child(CardTitle::new("Card Title"))
//!             .child(CardDescription::new("Card description text"))
//!     )
//!     .child(
//!         CardContent::new()
//!             .child("Main content goes here")
//!     )
//!     .child(
//!         CardFooter::new()
//!             .child("Footer content")
//!     )
//! ```

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, prelude::*,
};
use crate::theme::Theme;

/// Card container component.
///
/// Displays a card with rounded corners, border, and card background color.
#[derive(IntoElement)]
pub struct Card {
    children: Vec<AnyElement>,
}

impl Card {
    /// Create a new card.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for Card {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Card {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        div()
            .flex()
            .flex_col()
            .rounded_lg()
            .border_1()
            .border_color(colors.border)
            .bg(colors.card)
            .text_color(colors.card_foreground)
            .shadow_sm()
            .children(self.children)
    }
}

/// Card header section.
///
/// Contains the title and description, with vertical spacing.
#[derive(IntoElement)]
pub struct CardHeader {
    children: Vec<AnyElement>,
}

impl CardHeader {
    /// Create a new card header.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for CardHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(gpui::px(6.0))
            .p(gpui::px(24.0))
            .children(self.children)
    }
}

/// Card title text.
///
/// Renders with semibold weight and larger text size, with tight line height.
#[derive(IntoElement)]
pub struct CardTitle {
    text: SharedString,
}

impl CardTitle {
    /// Create a new card title with the given text.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for CardTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .text_lg()
            .font_weight(FontWeight::SEMIBOLD)
            .line_height(gpui::rems(1.75))
            .child(self.text)
    }
}

/// Card description text.
///
/// Renders with muted foreground color and smaller text size.
#[derive(IntoElement)]
pub struct CardDescription {
    text: SharedString,
}

impl CardDescription {
    /// Create a new card description with the given text.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for CardDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .text_sm()
            .text_color(theme.colors.muted_foreground)
            .child(self.text)
    }
}

/// Card content section.
///
/// The main content area of the card with horizontal padding and bottom padding.
#[derive(IntoElement)]
pub struct CardContent {
    children: Vec<AnyElement>,
}

impl CardContent {
    /// Create a new card content section.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for CardContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .px(gpui::px(24.0))
            .pb(gpui::px(24.0))
            .children(self.children)
    }
}

/// Card footer section.
///
/// A flex row at the bottom of the card for actions.
#[derive(IntoElement)]
pub struct CardFooter {
    children: Vec<AnyElement>,
}

impl CardFooter {
    /// Create a new card footer.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for CardFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .px(gpui::px(24.0))
            .pb(gpui::px(24.0))
            .children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_builder() {
        let _card = Card::new();
    }

    #[test]
    fn test_card_title() {
        let title = CardTitle::new("Test Title");
        assert_eq!(title.text, SharedString::from("Test Title"));
    }

    #[test]
    fn test_card_description() {
        let desc = CardDescription::new("Test description");
        assert_eq!(desc.text, SharedString::from("Test description"));
    }
}
