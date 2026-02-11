//! Typography components for shadcn-ui-rs
//!
//! A collection of styled text elements: headings, paragraphs, blockquotes,
//! inline code, and lead text.
//!
//! # Example
//!
//! ```rust
//! H1::new("Page Title")
//! H2::new("Section Title")
//! Paragraph::new("Body text here")
//! Blockquote::new().child(Paragraph::new("Quoted text"))
//! InlineCode::new("let x = 1;")
//! Lead::new("A lead paragraph with larger, muted text")
//! ```

use gpui::{
    div, AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window,
};
use crate::theme::Theme;

// ---------------------------------------------------------------------------
// H1
// ---------------------------------------------------------------------------

/// Heading level 1.
///
/// Renders with extra-bold weight and 3xl text size.
#[derive(IntoElement)]
pub struct H1 {
    text: SharedString,
}

impl H1 {
    /// Create a new H1 heading.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for H1 {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .text_3xl()
            .font_weight(FontWeight::EXTRA_BOLD)
            .line_height(gpui::rems(2.25))
            .text_color(theme.colors.foreground)
            .child(self.text)
    }
}

// ---------------------------------------------------------------------------
// H2
// ---------------------------------------------------------------------------

/// Heading level 2.
///
/// Renders with semibold weight and 2xl text size.
#[derive(IntoElement)]
pub struct H2 {
    text: SharedString,
}

impl H2 {
    /// Create a new H2 heading.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for H2 {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .text_2xl()
            .font_weight(FontWeight::SEMIBOLD)
            .line_height(gpui::rems(2.0))
            .text_color(theme.colors.foreground)
            .child(self.text)
    }
}

// ---------------------------------------------------------------------------
// H3
// ---------------------------------------------------------------------------

/// Heading level 3.
///
/// Renders with semibold weight and xl text size.
#[derive(IntoElement)]
pub struct H3 {
    text: SharedString,
}

impl H3 {
    /// Create a new H3 heading.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for H3 {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .text_xl()
            .font_weight(FontWeight::SEMIBOLD)
            .line_height(gpui::rems(1.75))
            .text_color(theme.colors.foreground)
            .child(self.text)
    }
}

// ---------------------------------------------------------------------------
// H4
// ---------------------------------------------------------------------------

/// Heading level 4.
///
/// Renders with semibold weight and lg text size.
#[derive(IntoElement)]
pub struct H4 {
    text: SharedString,
}

impl H4 {
    /// Create a new H4 heading.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for H4 {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .text_lg()
            .font_weight(FontWeight::SEMIBOLD)
            .line_height(gpui::rems(1.75))
            .text_color(theme.colors.foreground)
            .child(self.text)
    }
}

// ---------------------------------------------------------------------------
// Paragraph
// ---------------------------------------------------------------------------

/// Paragraph text element.
///
/// Renders with base text size and normal line height. Implements `ParentElement`
/// for mixed content (e.g. inline code within a paragraph).
#[derive(IntoElement)]
pub struct Paragraph {
    text: SharedString,
    children: Vec<AnyElement>,
}

impl Paragraph {
    /// Create a new paragraph with the given text.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            children: Vec::new(),
        }
    }
}

impl ParentElement for Paragraph {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Paragraph {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .text_base()
            .line_height(gpui::rems(1.75))
            .text_color(theme.colors.foreground)
            .child(self.text)
            .children(self.children)
    }
}

// ---------------------------------------------------------------------------
// Blockquote
// ---------------------------------------------------------------------------

/// Blockquote container.
///
/// Renders with a left border and left padding. Implements `ParentElement`
/// for containing paragraphs or other elements.
#[derive(IntoElement)]
pub struct Blockquote {
    children: Vec<AnyElement>,
}

impl Blockquote {
    /// Create a new blockquote.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for Blockquote {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Blockquote {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .border_l_2()
            .border_color(theme.colors.border)
            .pl(gpui::px(16.0))
            .italic()
            .text_color(theme.colors.muted_foreground)
            .children(self.children)
    }
}

// ---------------------------------------------------------------------------
// InlineCode
// ---------------------------------------------------------------------------

/// Inline code element.
///
/// Renders with muted background, small rounded corners, and monospace-style
/// smaller text.
#[derive(IntoElement)]
pub struct InlineCode {
    text: SharedString,
}

impl InlineCode {
    /// Create a new inline code element.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for InlineCode {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .bg(theme.colors.muted)
            .rounded_sm()
            .px(gpui::px(4.0))
            .py(gpui::px(1.0))
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .text_color(theme.colors.foreground)
            .child(self.text)
    }
}

// ---------------------------------------------------------------------------
// Lead
// ---------------------------------------------------------------------------

/// Lead paragraph.
///
/// Renders with xl text size and muted foreground color, used for introductory
/// or emphasized text.
#[derive(IntoElement)]
pub struct Lead {
    text: SharedString,
}

impl Lead {
    /// Create a new lead paragraph.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for Lead {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .text_xl()
            .text_color(theme.colors.muted_foreground)
            .child(self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h1_new() {
        let h1 = H1::new("Page Title");
        assert_eq!(h1.text, SharedString::from("Page Title"));
    }

    #[test]
    fn test_h2_new() {
        let h2 = H2::new("Section Title");
        assert_eq!(h2.text, SharedString::from("Section Title"));
    }

    #[test]
    fn test_h3_new() {
        let h3 = H3::new("Subsection Title");
        assert_eq!(h3.text, SharedString::from("Subsection Title"));
    }

    #[test]
    fn test_h4_new() {
        let h4 = H4::new("Minor Title");
        assert_eq!(h4.text, SharedString::from("Minor Title"));
    }

    #[test]
    fn test_paragraph_new() {
        let p = Paragraph::new("Body text");
        assert_eq!(p.text, SharedString::from("Body text"));
        assert!(p.children.is_empty());
    }

    #[test]
    fn test_blockquote_new() {
        let bq = Blockquote::new();
        assert!(bq.children.is_empty());
    }

    #[test]
    fn test_inline_code_new() {
        let code = InlineCode::new("let x = 1;");
        assert_eq!(code.text, SharedString::from("let x = 1;"));
    }

    #[test]
    fn test_lead_new() {
        let lead = Lead::new("Lead text");
        assert_eq!(lead.text, SharedString::from("Lead text"));
    }
}
