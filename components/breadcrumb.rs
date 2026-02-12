//! Breadcrumb component for shadcn-ui-rs
//!
//! Navigation path indicator with clickable items and separators.
//!
//! # Example
//!
//! ```rust
//! Breadcrumb::new()
//!     .child(BreadcrumbItem::new("Home").on_click(|_, _, _| {}))
//!     .child(BreadcrumbSeparator)
//!     .child(BreadcrumbItem::new("Products"))
//! ```

use gpui::prelude::*;
use gpui::{
    div, px, AnyElement, App, ClickEvent, IntoElement, ParentElement, SharedString, Styled, Window,
};

use crate::theme::Theme;

/// A single item in a breadcrumb trail.
///
/// Clickable items render in muted foreground; the last item (no on_click)
/// renders in foreground color to indicate the current page.
#[derive(IntoElement)]
pub struct BreadcrumbItem {
    label: SharedString,
    #[allow(clippy::type_complexity)]
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl BreadcrumbItem {
    /// Create a new breadcrumb item with the given label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            on_click: None,
        }
    }

    /// Set a click handler, making this item a navigable link.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for BreadcrumbItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        if let Some(on_click) = self.on_click {
            div()
                .id("breadcrumb-item")
                .text_sm()
                .text_color(colors.muted_foreground)
                .cursor_pointer()
                .hover(|style: gpui::StyleRefinement| style.underline())
                .on_click(on_click)
                .child(self.label)
                .into_any_element()
        } else {
            div()
                .text_sm()
                .text_color(colors.foreground)
                .child(self.label)
                .into_any_element()
        }
    }
}

/// A separator between breadcrumb items, renders "/" character.
#[derive(IntoElement)]
pub struct BreadcrumbSeparator;

impl RenderOnce for BreadcrumbSeparator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .text_sm()
            .text_color(theme.colors.muted_foreground)
            .child("/")
    }
}

/// Breadcrumb container component.
///
/// A horizontal row of breadcrumb items and separators showing a navigation path.
#[derive(IntoElement)]
pub struct Breadcrumb {
    children: Vec<AnyElement>,
}

impl Breadcrumb {
    /// Create a new breadcrumb container.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for Breadcrumb {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Breadcrumb {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumb_item() {
        let item = BreadcrumbItem::new("Home");
        assert_eq!(item.label, SharedString::from("Home"));
        assert!(item.on_click.is_none());
    }

    #[test]
    fn test_breadcrumb_separator() {
        let _sep = BreadcrumbSeparator;
    }

    #[test]
    fn test_breadcrumb_builder() {
        let breadcrumb = Breadcrumb::new()
            .child(BreadcrumbItem::new("Home").on_click(|_, _, _| {}))
            .child(BreadcrumbSeparator)
            .child(BreadcrumbItem::new("Current"));
        assert_eq!(breadcrumb.children.len(), 3);
    }
}
