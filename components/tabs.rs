//! Tabs component for shadcn-ui-rs
//!
//! Tabbed content panels with a tab list and switchable content areas.
//!
//! # Example
//!
//! ```rust
//! Tabs::new("my-tabs")
//!     .value("account")
//!     .on_value_change(|value, _window, _cx| {
//!         // set active tab
//!     })
//!     .child(
//!         TabsList::new()
//!             .child(TabsTrigger::new("t-account", "Account").active(true))
//!             .child(TabsTrigger::new("t-password", "Password"))
//!     )
//!     .child(TabsContent::new("account").active(true).child("Account settings"))
//!     .child(TabsContent::new("password").child("Password settings"))
//! ```

use std::rc::Rc;

use gpui::prelude::*;
use gpui::{
    div, px, AnyElement, App, ClickEvent, Div, ElementId, FontWeight, IntoElement, SharedString,
    Stateful, Window,
};

use crate::theme::Theme;

/// Tabs container.
///
/// Wraps a [`TabsList`] and multiple [`TabsContent`] panels.
/// The parent View manages the active tab value.
#[derive(IntoElement)]
pub struct Tabs {
    id: ElementId,
    value: SharedString,
    #[allow(clippy::type_complexity)]
    on_value_change: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl Tabs {
    /// Create a new tabs container with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: SharedString::default(),
            on_value_change: None,
            children: Vec::new(),
        }
    }

    /// Set the active tab value.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    /// Set the callback for when the active tab changes.
    pub fn on_value_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for Tabs {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Tabs {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .children(self.children)
    }
}

/// Horizontal list of tab triggers.
///
/// Renders as a row with muted background and rounded corners.
#[derive(IntoElement)]
pub struct TabsList {
    children: Vec<AnyElement>,
}

impl TabsList {
    /// Create a new tabs list.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for TabsList {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TabsList {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let radius = theme.radius.to_px();

        div()
            .flex()
            .flex_row()
            .items_center()
            .h(px(40.0))
            .rounded(px(radius))
            .bg(theme.colors.muted)
            .p(px(4.0))
            .children(self.children)
    }
}

/// A single tab trigger button within a [`TabsList`].
///
/// Displays the tab label. Active state shows a distinct background and shadow.
#[derive(IntoElement)]
pub struct TabsTrigger {
    id: ElementId,
    value: SharedString,
    label: SharedString,
    active: bool,
    disabled: bool,
    #[allow(clippy::type_complexity)]
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl TabsTrigger {
    /// Create a new tab trigger with a value and display label.
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        let value = value.into();
        Self {
            id: ElementId::Name(value.clone()),
            value,
            label: label.into(),
            active: false,
            disabled: false,
            on_click: None,
        }
    }

    /// Set the active (selected) state.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the click handler.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for TabsTrigger {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;
        let radius = theme.radius.to_px();

        let bg_color = colors.background;
        let fg_color = colors.foreground;
        let muted_fg = colors.muted_foreground;
        let hover_bg = colors.accent;

        let mut el = div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .px(px(12.0))
            .py(px(6.0))
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .cursor_pointer();

        if self.active {
            el = el
                .bg(bg_color)
                .text_color(fg_color)
                .shadow_sm()
                .rounded(px(radius - 2.0));
        } else {
            el = el.text_color(muted_fg);
            if !self.disabled {
                el = el.hover(move |style| style.bg(hover_bg));
            }
        }

        if self.disabled {
            el = el.opacity(0.5).cursor_default();
        }

        if !self.disabled {
            if let Some(on_click) = self.on_click {
                el = el.on_click(
                    move |event: &ClickEvent, window: &mut Window, cx: &mut App| {
                        on_click(event, window, cx);
                    },
                );
            }
        }

        el.child(self.label)
    }
}

/// Content panel associated with a tab value.
///
/// Only renders its children when `active` is true.
#[derive(IntoElement)]
pub struct TabsContent {
    value: SharedString,
    active: bool,
    children: Vec<AnyElement>,
}

impl TabsContent {
    /// Create a new tab content panel with the given value.
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            active: false,
            children: Vec::new(),
        }
    }

    /// Set whether this content is currently active (visible).
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

impl ParentElement for TabsContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TabsContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().when(self.active, |el: Div| {
            el.mt(px(8.0)).children(self.children)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tabs_defaults() {
        let tabs = Tabs::new("test").value("tab1");
        assert_eq!(tabs.value, SharedString::from("tab1"));
        assert!(tabs.on_value_change.is_none());
        assert!(tabs.children.is_empty());
    }

    #[test]
    fn test_tabs_list() {
        let list = TabsList::new();
        assert!(list.children.is_empty());
    }

    #[test]
    fn test_tabs_trigger() {
        let trigger = TabsTrigger::new("tab1", "Account")
            .active(false)
            .disabled(true);

        assert_eq!(trigger.value, SharedString::from("tab1"));
        assert_eq!(trigger.label, SharedString::from("Account"));
        assert!(!trigger.active);
        assert!(trigger.disabled);
    }

    #[test]
    fn test_tabs_content() {
        let content = TabsContent::new("tab1").active(true);
        assert_eq!(content.value, SharedString::from("tab1"));
        assert!(content.active);
        assert!(content.children.is_empty());
    }

    #[test]
    fn test_tabs_trigger_active() {
        let trigger = TabsTrigger::new("tab1", "Account").active(true);
        assert!(trigger.active);

        let trigger = TabsTrigger::new("tab2", "Password").active(false);
        assert!(!trigger.active);
    }
}
