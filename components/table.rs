//! Table component for shadcn-ui-rs
//!
//! A pure layout data table built with flex containers.
//!
//! # Example
//!
//! ```rust
//! Table::new()
//!     .child(TableHeader::new()
//!         .child(TableRow::new()
//!             .child(TableHead::new("Name"))
//!             .child(TableHead::new("Email"))))
//!     .child(TableBody::new()
//!         .child(TableRow::new()
//!             .child(TableCell::new().child("Alice"))
//!             .child(TableCell::new().child("alice@example.com"))))
//!     .child(TableCaption::new("A list of users"))
//! ```

use gpui::{
    div, px, AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    Styled, Window,
};

use crate::theme::Theme;

/// Table container component.
///
/// A flex column that contains TableHeader, TableBody, and optionally TableCaption.
#[derive(IntoElement)]
pub struct Table {
    children: Vec<AnyElement>,
}

impl Table {
    /// Create a new table.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for Table {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Table {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .flex_col()
            .text_sm()
            .children(self.children)
    }
}

/// Table header section.
///
/// A flex column container for header rows.
#[derive(IntoElement)]
pub struct TableHeader {
    children: Vec<AnyElement>,
}

impl TableHeader {
    /// Create a new table header.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for TableHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().flex_col().children(self.children)
    }
}

/// Table body section.
///
/// A flex column container for data rows.
#[derive(IntoElement)]
pub struct TableBody {
    children: Vec<AnyElement>,
}

impl TableBody {
    /// Create a new table body.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for TableBody {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableBody {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().flex_col().children(self.children)
    }
}

/// Table row.
///
/// A flex row with bottom border and hover background.
#[derive(IntoElement)]
pub struct TableRow {
    children: Vec<AnyElement>,
}

impl TableRow {
    /// Create a new table row.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for TableRow {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let hover_bg = colors.muted;

        div()
            .flex()
            .flex_row()
            .items_center()
            .border_b_1()
            .border_color(colors.border)
            .hover(move |s| s.bg(hover_bg))
            .children(self.children)
    }
}

/// Table head cell.
///
/// A header cell with muted text and medium font weight.
#[derive(IntoElement)]
pub struct TableHead {
    text: SharedString,
}

impl TableHead {
    /// Create a new table head cell with the given text.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for TableHead {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        div()
            .flex_1()
            .flex()
            .items_center()
            .h(px(48.0))
            .px(px(16.0))
            .text_color(colors.muted_foreground)
            .font_weight(FontWeight::MEDIUM)
            .child(self.text)
    }
}

/// Table data cell.
///
/// A standard data cell with vertical padding.
#[derive(IntoElement)]
pub struct TableCell {
    children: Vec<AnyElement>,
}

impl TableCell {
    /// Create a new table cell.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for TableCell {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableCell {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex_1()
            .flex()
            .items_center()
            .py(px(16.0))
            .px(px(16.0))
            .children(self.children)
    }
}

/// Table caption.
///
/// A descriptive text shown below the table.
#[derive(IntoElement)]
pub struct TableCaption {
    text: SharedString,
}

impl TableCaption {
    /// Create a new table caption with the given text.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for TableCaption {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        div()
            .mt(px(16.0))
            .text_sm()
            .text_color(colors.muted_foreground)
            .child(self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_new() {
        let table = Table::new();
        assert!(table.children.is_empty());
    }

    #[test]
    fn test_table_head() {
        let head = TableHead::new("Name");
        assert_eq!(head.text, SharedString::from("Name"));
    }

    #[test]
    fn test_table_caption() {
        let caption = TableCaption::new("A list of users");
        assert_eq!(caption.text, SharedString::from("A list of users"));
    }

    #[test]
    fn test_table_cell_new() {
        let cell = TableCell::new();
        assert!(cell.children.is_empty());
    }
}
