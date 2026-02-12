//! Item component for shadcn-ui-rs
//!
//! A generic list item with icon, label, description, and trailing action.
//!
//! # Example
//!
//! ```rust
//! Item::new("settings", "Settings")
//!     .icon("⚙")
//!     .description("Manage your preferences")
//!     .on_click(|_, _, _| {})
//! ```

use gpui::prelude::*;
use gpui::{
    div, px, AnyElement, App, ClickEvent, Div, ElementId, IntoElement, SharedString, Stateful,
    Window,
};

use crate::theme::Theme;

/// A generic list item component.
///
/// Used in menus, lists, and sidebars. Supports icon, label, description,
/// trailing action element, and click interaction.
#[derive(IntoElement)]
pub struct Item {
    id: ElementId,
    label: SharedString,
    description: Option<SharedString>,
    icon: Option<SharedString>,
    action: Option<AnyElement>,
    disabled: bool,
    #[allow(clippy::type_complexity)]
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl Item {
    /// Create a new item with the given id and label.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            icon: None,
            action: None,
            disabled: false,
            on_click: None,
        }
    }

    /// Set the item description text.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set a text/emoji icon for the item.
    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set a trailing action element (e.g. badge, button).
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }

    /// Set whether this item is disabled.
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

impl RenderOnce for Item {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let accent = colors.accent;
        let accent_fg = colors.accent_foreground;
        let muted_fg = colors.muted_foreground;
        let is_clickable = self.on_click.is_some() && !self.disabled;

        let mut el = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .px(px(12.0))
            .py(px(8.0));

        if self.disabled {
            el = el.opacity(0.5);
        }

        if is_clickable {
            el = el
                .cursor_pointer()
                .hover(|style: gpui::StyleRefinement| style.bg(accent).text_color(accent_fg));
        }

        if let Some(on_click) = self.on_click {
            if !self.disabled {
                el = el.on_click(on_click);
            }
        }

        // Icon
        if let Some(icon) = self.icon {
            el = el.child(div().w(px(20.0)).text_center().child(icon));
        }

        // Label and description column
        let mut label_col = div().flex_1().flex().flex_col();
        label_col = label_col.child(div().text_sm().child(self.label));
        if let Some(description) = self.description {
            label_col = label_col.child(
                div()
                    .text_xs()
                    .text_color(muted_fg)
                    .child(description),
            );
        }
        el = el.child(label_col);

        // Trailing action
        if let Some(action) = self.action {
            el = el.child(action);
        }

        el
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_defaults() {
        let item = Item::new("test", "Test Item");
        assert_eq!(item.label, SharedString::from("Test Item"));
        assert!(item.icon.is_none());
        assert!(item.description.is_none());
        assert!(item.action.is_none());
        assert!(!item.disabled);
        assert!(item.on_click.is_none());
    }

    #[test]
    fn test_item_builder() {
        let item = Item::new("settings", "Settings")
            .icon("⚙")
            .description("Manage preferences")
            .disabled(false)
            .on_click(|_, _, _| {});
        assert_eq!(item.label, SharedString::from("Settings"));
        assert_eq!(item.icon, Some(SharedString::from("⚙")));
        assert_eq!(
            item.description,
            Some(SharedString::from("Manage preferences"))
        );
        assert!(!item.disabled);
        assert!(item.on_click.is_some());
    }

    #[test]
    fn test_item_disabled() {
        let item = Item::new("disabled", "Disabled Item").disabled(true);
        assert!(item.disabled);
    }
}
