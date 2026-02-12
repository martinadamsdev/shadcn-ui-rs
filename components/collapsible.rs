//! Collapsible component for shadcn-ui-rs
//!
//! A single expandable/collapsible section with a trigger and content area.
//!
//! # Example
//!
//! ```rust
//! Collapsible::new("my-section")
//!     .open(is_open)
//!     .on_open_change(|open, _window, _cx| {
//!         // toggle state
//!     })
//!     .trigger(div().child("Click to expand"))
//!     .child("Hidden content revealed when open")
//! ```

use std::rc::Rc;

use gpui::prelude::*;
use gpui::{div, AnyElement, App, ClickEvent, Div, ElementId, IntoElement, Stateful, Window};

use crate::theme::Theme;

/// A controlled collapsible section.
///
/// Click the trigger to toggle the content visibility.
/// The parent manages the open/close state via `on_open_change`.
#[derive(IntoElement)]
pub struct Collapsible {
    id: ElementId,
    open: bool,
    #[allow(clippy::type_complexity)]
    on_open_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
    trigger: Vec<AnyElement>,
    children: Vec<AnyElement>,
}

impl Collapsible {
    /// Create a new collapsible section with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            on_open_change: None,
            trigger: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Set the open state of the collapsible.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
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

    /// Set the trigger element that toggles the collapsible.
    pub fn trigger(mut self, element: impl IntoElement) -> Self {
        self.trigger = vec![element.into_any_element()];
        self
    }
}

impl ParentElement for Collapsible {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Collapsible {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let open = self.open;

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .child(
                div()
                    .id("collapsible-trigger")
                    .cursor_pointer()
                    .when_some(self.on_open_change, |el: Stateful<Div>, on_open_change| {
                        el.on_click(move |_event: &ClickEvent, window: &mut Window, cx: &mut App| {
                            on_open_change(!open, window, cx);
                        })
                    })
                    .children(self.trigger),
            )
            .when(open, |el: Stateful<Div>| el.children(self.children))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapsible_defaults() {
        let collapsible = Collapsible::new("test");
        assert!(!collapsible.open);
        assert!(collapsible.on_open_change.is_none());
        assert!(collapsible.trigger.is_empty());
        assert!(collapsible.children.is_empty());
    }

    #[test]
    fn test_collapsible_builder() {
        let collapsible = Collapsible::new("test")
            .open(true)
            .on_open_change(|_open, _window, _cx| {})
            .trigger(div().child("Toggle"));

        assert!(collapsible.open);
        assert!(collapsible.on_open_change.is_some());
        assert_eq!(collapsible.trigger.len(), 1);
    }

    #[test]
    fn test_collapsible_open() {
        let collapsible = Collapsible::new("test").open(true);
        assert!(collapsible.open);

        let collapsible = Collapsible::new("test").open(false);
        assert!(!collapsible.open);
    }
}
