//! Sidebar component for shadcn-ui-rs
//!
//! A collapsible side navigation panel with header, content, and footer sections.
//!
//! # Example
//!
//! ```rust
//! div().flex().flex_row().size_full()
//!     .child(
//!         Sidebar::new("sidebar")
//!             .open(true)
//!             .side(SidebarSide::Left)
//!             .width(px(280.0))
//!             .child(SidebarHeader::new().child("Logo"))
//!             .child(
//!                 SidebarContent::new()
//!                     .child("Nav items")
//!             )
//!             .child(SidebarFooter::new().child("User"))
//!     )
//!     .child(div().flex_1().child("Main content"))
//! ```

use gpui::prelude::*;
use gpui::{
    div, px, AnyElement, App, ClickEvent, Div, ElementId, IntoElement, ParentElement, Pixels,
    RenderOnce, Stateful, Styled, Window,
};
use crate::theme::Theme;

/// The side of the layout the sidebar appears on.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SidebarSide {
    /// Sidebar appears on the left (default).
    #[default]
    Left,
    /// Sidebar appears on the right.
    Right,
}

/// A collapsible side navigation panel.
///
/// Renders as a fixed-width column with header, scrollable content, and footer
/// sections. When closed, collapses to zero width with hidden overflow.
#[derive(IntoElement)]
pub struct Sidebar {
    id: ElementId,
    open: bool,
    side: SidebarSide,
    width: Pixels,
    children: Vec<AnyElement>,
}

impl Sidebar {
    /// Create a new sidebar with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: true,
            side: SidebarSide::default(),
            width: px(280.0),
            children: Vec::new(),
        }
    }

    /// Set whether the sidebar is open (visible) or closed (collapsed).
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set which side the sidebar appears on.
    pub fn side(mut self, side: SidebarSide) -> Self {
        self.side = side;
        self
    }

    /// Set the width of the sidebar when open.
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }
}

impl ParentElement for Sidebar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let border_color = colors.border;
        let bg = colors.background;

        let mut el = div()
            .id(self.id)
            .flex()
            .flex_col()
            .h_full()
            .bg(bg)
            .border_color(border_color);

        el = match self.side {
            SidebarSide::Left => el.border_r_1(),
            SidebarSide::Right => el.border_l_1(),
        };

        if self.open {
            el = el.w(self.width);
        } else {
            el = el.w(px(0.0)).overflow_hidden();
        }

        el.children(self.children)
    }
}

/// Sidebar header section.
///
/// Top section of the sidebar with fixed height and bottom border.
#[derive(IntoElement)]
pub struct SidebarHeader {
    children: Vec<AnyElement>,
}

impl SidebarHeader {
    /// Create a new sidebar header.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for SidebarHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        div()
            .flex()
            .items_center()
            .h(px(56.0))
            .px(px(16.0))
            .border_b_1()
            .border_color(colors.border)
            .children(self.children)
    }
}

/// Sidebar content section.
///
/// Scrollable middle section that fills available space.
#[derive(IntoElement)]
pub struct SidebarContent {
    children: Vec<AnyElement>,
}

impl SidebarContent {
    /// Create a new sidebar content section.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for SidebarContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id("sidebar-content")
            .flex_1()
            .flex()
            .flex_col()
            .py(px(8.0))
            .overflow_y_scroll()
            .children(self.children)
    }
}

/// Sidebar footer section.
///
/// Bottom section of the sidebar with fixed height and top border.
#[derive(IntoElement)]
pub struct SidebarFooter {
    children: Vec<AnyElement>,
}

impl SidebarFooter {
    /// Create a new sidebar footer.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for SidebarFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        div()
            .flex()
            .items_center()
            .h(px(56.0))
            .px(px(16.0))
            .border_t_1()
            .border_color(colors.border)
            .children(self.children)
    }
}

/// Sidebar toggle trigger button.
///
/// A wrapper for a clickable element that toggles the sidebar open/closed state.
#[derive(IntoElement)]
pub struct SidebarTrigger {
    id: ElementId,
    #[allow(clippy::type_complexity)]
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl SidebarTrigger {
    /// Create a new sidebar trigger with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            on_click: None,
            children: Vec::new(),
        }
    }

    /// Set the click handler for toggling the sidebar.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl ParentElement for SidebarTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarTrigger {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut el = div().id(self.id).cursor_pointer();

        if let Some(handler) = self.on_click {
            el = el.on_click(handler);
        }

        el.children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sidebar_defaults() {
        let sidebar = Sidebar::new("test-sidebar");
        assert!(sidebar.open);
        assert_eq!(sidebar.side, SidebarSide::Left);
        assert_eq!(sidebar.width, px(280.0));
        assert!(sidebar.children.is_empty());
    }

    #[test]
    fn test_sidebar_builder() {
        let sidebar = Sidebar::new("test-sidebar")
            .open(false)
            .side(SidebarSide::Right)
            .width(px(320.0));
        assert!(!sidebar.open);
        assert_eq!(sidebar.side, SidebarSide::Right);
        assert_eq!(sidebar.width, px(320.0));
    }

    #[test]
    fn test_sidebar_side() {
        let left = Sidebar::new("left").side(SidebarSide::Left);
        assert_eq!(left.side, SidebarSide::Left);

        let right = Sidebar::new("right").side(SidebarSide::Right);
        assert_eq!(right.side, SidebarSide::Right);
    }

    #[test]
    fn test_sidebar_trigger() {
        let trigger = SidebarTrigger::new("toggle");
        assert!(trigger.on_click.is_none());
        assert!(trigger.children.is_empty());

        let trigger_with_click =
            SidebarTrigger::new("toggle").on_click(|_event, _window, _cx| {});
        assert!(trigger_with_click.on_click.is_some());
    }
}
