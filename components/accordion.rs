//! Accordion component for shadcn-ui-rs
//!
//! Multiple expandable/collapsible sections. Supports single (one at a time)
//! and multiple (any number) open modes.
//!
//! # Example
//!
//! ```rust
//! Accordion::new("faq")
//!     .type_(AccordionType::Single)
//!     .value(vec!["item-1".into()])
//!     .on_value_change(|values, _window, _cx| {
//!         // update state
//!     })
//!     .child(
//!         AccordionItem::new("item-1")
//!             .open(true)
//!             .on_toggle(|_window, _cx| { /* toggle */ })
//!             .child(AccordionTrigger::new().child("Section 1"))
//!             .child(AccordionContent::new().child("Content 1"))
//!     )
//!     .child(
//!         AccordionItem::new("item-2")
//!             .child(AccordionTrigger::new().child("Section 2"))
//!             .child(AccordionContent::new().child("Content 2"))
//!     )
//! ```

use std::rc::Rc;

use gpui::prelude::*;
use gpui::{div, px, AnyElement, App, ClickEvent, Div, ElementId, IntoElement, SharedString, Stateful, Window};

use crate::theme::Theme;

/// Selection mode for the accordion.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AccordionType {
    /// Only one item can be open at a time.
    #[default]
    Single,
    /// Multiple items can be open simultaneously.
    Multiple,
}

/// Accordion container.
///
/// Wraps multiple [`AccordionItem`] children. The parent View manages which
/// items are open; the Accordion itself is a layout container.
#[derive(IntoElement)]
pub struct Accordion {
    id: ElementId,
    type_: AccordionType,
    value: Vec<SharedString>,
    #[allow(clippy::type_complexity)]
    on_value_change: Option<Rc<dyn Fn(Vec<SharedString>, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl Accordion {
    /// Create a new accordion with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            type_: AccordionType::Single,
            value: Vec::new(),
            on_value_change: None,
            children: Vec::new(),
        }
    }

    /// Set the accordion type (single or multiple).
    pub fn type_(mut self, type_: AccordionType) -> Self {
        self.type_ = type_;
        self
    }

    /// Set the currently open item values.
    pub fn value(mut self, value: Vec<SharedString>) -> Self {
        self.value = value;
        self
    }

    /// Set the callback for when the open items change.
    pub fn on_value_change(
        mut self,
        handler: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for Accordion {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Accordion {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .children(self.children)
    }
}

/// A single section within an [`Accordion`].
///
/// Contains a trigger and content. The `open` prop controls whether the
/// content is visible. The parent View manages the open state.
#[derive(IntoElement)]
pub struct AccordionItem {
    value: SharedString,
    open: bool,
    #[allow(clippy::type_complexity)]
    on_toggle: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl AccordionItem {
    /// Create a new accordion item with the given value.
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            open: false,
            on_toggle: None,
            children: Vec::new(),
        }
    }

    /// Set the open state of this item.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set the callback for when this item is toggled.
    pub fn on_toggle(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for AccordionItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AccordionItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let border_color = theme.colors.border;

        div()
            .flex()
            .flex_col()
            .border_b_1()
            .border_color(border_color)
            .children(self.children)
    }
}

/// Clickable header row within an [`AccordionItem`].
///
/// Renders a row with children and a chevron indicator. Clicking toggles
/// the parent item's open state via the `on_toggle` callback.
#[derive(IntoElement)]
pub struct AccordionTrigger {
    #[allow(clippy::type_complexity)]
    on_click: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    open: bool,
    children: Vec<AnyElement>,
}

impl AccordionTrigger {
    /// Create a new accordion trigger.
    pub fn new() -> Self {
        Self {
            on_click: None,
            open: false,
            children: Vec::new(),
        }
    }

    /// Set the click handler (typically toggles the item).
    pub fn on_click(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    /// Set whether the associated item is open (controls chevron direction).
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl ParentElement for AccordionTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AccordionTrigger {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let muted_fg = theme.colors.muted_foreground;

        let mut el = div()
            .id("accordion-trigger")
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .py(px(16.0))
            .cursor_pointer()
            .hover(move |style| style.text_color(muted_fg))
            .children(self.children)
            .child(
                div()
                    .text_sm()
                    .child(if self.open { "\u{25B2}" } else { "\u{25BC}" }),
            );

        if let Some(on_click) = self.on_click {
            el = el.on_click(
                move |_event: &ClickEvent, window: &mut Window, cx: &mut App| {
                    on_click(window, cx);
                },
            );
        }

        el
    }
}

/// Content area within an [`AccordionItem`].
///
/// Only visible when the parent item's `open` prop is true.
/// Pass `active(true)` to show the content.
#[derive(IntoElement)]
pub struct AccordionContent {
    active: bool,
    children: Vec<AnyElement>,
}

impl AccordionContent {
    /// Create a new accordion content area.
    pub fn new() -> Self {
        Self {
            active: false,
            children: Vec::new(),
        }
    }

    /// Set whether the content is visible.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

impl ParentElement for AccordionContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AccordionContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().when(self.active, |el: Div| {
            el.pb(px(16.0)).children(self.children)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accordion_defaults() {
        let accordion = Accordion::new("test");
        assert_eq!(accordion.type_, AccordionType::Single);
        assert!(accordion.value.is_empty());
        assert!(accordion.on_value_change.is_none());
        assert!(accordion.children.is_empty());
    }

    #[test]
    fn test_accordion_item() {
        let item = AccordionItem::new("item-1").open(true);
        assert_eq!(item.value, SharedString::from("item-1"));
        assert!(item.open);
    }

    #[test]
    fn test_accordion_trigger() {
        let trigger = AccordionTrigger::new().open(true);
        assert!(trigger.open);
        assert!(trigger.children.is_empty());
    }

    #[test]
    fn test_accordion_content() {
        let content = AccordionContent::new().active(true);
        assert!(content.active);
        assert!(content.children.is_empty());
    }

    #[test]
    fn test_accordion_type() {
        let accordion = Accordion::new("test").type_(AccordionType::Multiple);
        assert_eq!(accordion.type_, AccordionType::Multiple);

        let accordion = Accordion::new("test").type_(AccordionType::Single);
        assert_eq!(accordion.type_, AccordionType::Single);
    }
}
