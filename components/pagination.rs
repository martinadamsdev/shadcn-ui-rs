//! Pagination component for shadcn-ui-rs
//!
//! Page navigation controls with previous/next buttons and page number buttons.
//!
//! # Example
//!
//! ```rust
//! Pagination::new("pages")
//!     .current_page(3)
//!     .total_pages(10)
//!     .on_page_change(|page, _window, _cx| {
//!         println!("Navigate to page {}", page);
//!     })
//! ```

use std::rc::Rc;

use gpui::prelude::*;
use gpui::{
    div, px, App, ClickEvent, ElementId, IntoElement, RenderOnce, Styled, Window,
};
use crate::theme::Theme;

/// Page navigation controls.
///
/// Renders a horizontal row with Previous button, page number buttons, and
/// Next button. The current page is highlighted. Ellipsis ("...") is shown
/// for gaps in the page range.
#[derive(IntoElement)]
pub struct Pagination {
    id: ElementId,
    current_page: usize,
    total_pages: usize,
    #[allow(clippy::type_complexity)]
    on_page_change: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl Pagination {
    /// Create a new pagination with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            current_page: 1,
            total_pages: 1,
            on_page_change: None,
        }
    }

    /// Set the current active page (1-indexed).
    pub fn current_page(mut self, page: usize) -> Self {
        self.current_page = page;
        self
    }

    /// Set the total number of pages.
    pub fn total_pages(mut self, total: usize) -> Self {
        self.total_pages = total;
        self
    }

    /// Set the callback for when a page is selected.
    pub fn on_page_change(
        mut self,
        handler: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_page_change = Some(Rc::new(handler));
        self
    }
}

/// Compute the page window: which page numbers and ellipses to display.
///
/// Returns a list of `Some(page_number)` for page buttons, or `None` for
/// ellipsis gaps. Always includes first and last page, plus up to 2 pages
/// around the current page.
fn compute_page_window(current: usize, total: usize) -> Vec<Option<usize>> {
    if total == 0 {
        return vec![];
    }
    if total <= 7 {
        return (1..=total).map(Some).collect();
    }

    let mut pages = Vec::new();

    // Always include page 1
    pages.push(Some(1));

    // Window around current page: current-2 .. current+2
    let window_start = if current > 3 { current - 2 } else { 2 };
    let window_end = if current + 2 < total {
        current + 2
    } else {
        total - 1
    };

    // Add ellipsis before window if gap exists
    if window_start > 2 {
        pages.push(None);
    }

    // Add pages in window
    for p in window_start..=window_end {
        pages.push(Some(p));
    }

    // Add ellipsis after window if gap exists
    if window_end < total - 1 {
        pages.push(None);
    }

    // Always include last page
    pages.push(Some(total));

    pages
}

impl RenderOnce for Pagination {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let primary = colors.primary;
        let primary_fg = colors.primary_foreground;
        let fg = colors.foreground;
        let accent = colors.accent;
        let muted_fg = colors.muted_foreground;

        let current = self.current_page;
        let total = self.total_pages;
        let on_page_change = self.on_page_change;

        let page_window = compute_page_window(current, total);

        let mut container = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0));

        // Previous button
        let prev_disabled = current <= 1;
        {
            let mut prev_btn = div()
                .id("pagination-prev")
                .w(px(36.0))
                .h(px(36.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(6.0))
                .text_sm()
                .child("\u{2190}");

            if prev_disabled {
                prev_btn = prev_btn.opacity(0.5).cursor_default();
            } else {
                let handler = on_page_change.clone();
                prev_btn = prev_btn
                    .cursor_pointer()
                    .text_color(fg)
                    .hover(|s: gpui::StyleRefinement| s.bg(accent))
                    .on_click(move |_event: &ClickEvent, window: &mut Window, cx: &mut App| {
                        if let Some(ref cb) = handler {
                            cb(current - 1, window, cx);
                        }
                    });
            }

            container = container.child(prev_btn);
        }

        // Page buttons
        for (idx, entry) in page_window.iter().enumerate() {
            match entry {
                Some(page) => {
                    let page = *page;
                    let is_current = page == current;

                    let mut btn = div()
                        .id(ElementId::NamedInteger("page".into(), idx))
                        .w(px(36.0))
                        .h(px(36.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(px(6.0))
                        .text_sm()
                        .cursor_pointer()
                        .child(format!("{}", page));

                    if is_current {
                        btn = btn.bg(primary).text_color(primary_fg);
                    } else {
                        let handler = on_page_change.clone();
                        btn = btn
                            .text_color(fg)
                            .hover(|s: gpui::StyleRefinement| s.bg(accent))
                            .on_click(
                                move |_event: &ClickEvent, window: &mut Window, cx: &mut App| {
                                    if let Some(ref cb) = handler {
                                        cb(page, window, cx);
                                    }
                                },
                            );
                    }

                    container = container.child(btn);
                }
                None => {
                    let ellipsis = div()
                        .w(px(36.0))
                        .h(px(36.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(muted_fg)
                        .child("\u{2026}");
                    container = container.child(ellipsis);
                }
            }
        }

        // Next button
        let next_disabled = current >= total;
        {
            let mut next_btn = div()
                .id("pagination-next")
                .w(px(36.0))
                .h(px(36.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(6.0))
                .text_sm()
                .child("\u{2192}");

            if next_disabled {
                next_btn = next_btn.opacity(0.5).cursor_default();
            } else {
                let handler = on_page_change;
                next_btn = next_btn
                    .cursor_pointer()
                    .text_color(fg)
                    .hover(|s: gpui::StyleRefinement| s.bg(accent))
                    .on_click(move |_event: &ClickEvent, window: &mut Window, cx: &mut App| {
                        if let Some(ref cb) = handler {
                            cb(current + 1, window, cx);
                        }
                    });
            }

            container = container.child(next_btn);
        }

        container
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_defaults() {
        let pagination = Pagination::new("test-pages");
        assert_eq!(pagination.current_page, 1);
        assert_eq!(pagination.total_pages, 1);
        assert!(pagination.on_page_change.is_none());
    }

    #[test]
    fn test_pagination_builder() {
        let pagination = Pagination::new("test-pages")
            .current_page(5)
            .total_pages(20)
            .on_page_change(|_page, _window, _cx| {});
        assert_eq!(pagination.current_page, 5);
        assert_eq!(pagination.total_pages, 20);
        assert!(pagination.on_page_change.is_some());
    }

    #[test]
    fn test_pagination_page_window() {
        // Small total: show all pages
        assert_eq!(
            compute_page_window(1, 5),
            vec![Some(1), Some(2), Some(3), Some(4), Some(5)]
        );

        // Large total, current in middle: 1 ... 3 4 5 6 7 ... 20
        let window = compute_page_window(5, 20);
        assert_eq!(window[0], Some(1));
        assert_eq!(window[1], None); // ellipsis
        assert!(window.contains(&Some(5))); // current page included
        assert_eq!(*window.last().unwrap(), Some(20));

        // Large total, current near start: 1 2 3 4 5 ... 20
        let window = compute_page_window(2, 20);
        assert_eq!(window[0], Some(1));
        assert!(window.contains(&Some(2)));
        assert_eq!(*window.last().unwrap(), Some(20));

        // Large total, current near end: 1 ... 18 19 20
        let window = compute_page_window(19, 20);
        assert_eq!(window[0], Some(1));
        assert_eq!(*window.last().unwrap(), Some(20));
        assert!(window.contains(&Some(19)));
    }

    #[test]
    fn test_pagination_edge_cases() {
        // Single page
        let window = compute_page_window(1, 1);
        assert_eq!(window, vec![Some(1)]);

        // Zero pages
        let window = compute_page_window(1, 0);
        assert!(window.is_empty());

        // First page of many
        let pagination = Pagination::new("p").current_page(1).total_pages(10);
        assert_eq!(pagination.current_page, 1);

        // Last page of many
        let pagination = Pagination::new("p").current_page(10).total_pages(10);
        assert_eq!(pagination.current_page, 10);
    }
}
