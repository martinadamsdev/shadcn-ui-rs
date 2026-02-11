//! Button component for shadcn-ui-rs
//!
//! A versatile button component with multiple variants and sizes.
//!
//! # Usage
//!
//! ```rust
//! Button::new("Click me")
//!     .variant(ButtonVariant::Outline)
//!     .size(ButtonSize::Lg)
//!     .on_click(|_event, _window, _cx| {
//!         println!("clicked!");
//!     })
//! ```

use gpui::prelude::*;
use gpui::{
    App, ClickEvent, Div, ElementId, FontWeight, Hsla, IntoElement, SharedString, Stateful, Window,
    div, px, rems,
};

use crate::theme::{Radius, Theme};

/// Button visual variant
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Ghost,
    Link,
    Destructive,
}

/// Button size preset
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Icon,
}

/// A button component with configurable variant, size, and click handler.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    #[allow(clippy::type_complexity)]
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Button {
    pub fn new(label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self {
            id: ElementId::Name(label.clone()),
            label,
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            disabled: false,
            on_click: None,
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;
        let radius = theme.radius;

        // Determine colors based on variant
        let (bg, fg, border, hover_bg) = match self.variant {
            ButtonVariant::Default => (
                colors.primary,
                colors.primary_foreground,
                colors.primary,
                with_opacity(colors.primary, 0.9),
            ),
            ButtonVariant::Secondary => (
                colors.secondary,
                colors.secondary_foreground,
                colors.secondary,
                with_opacity(colors.secondary, 0.8),
            ),
            ButtonVariant::Outline => (
                colors.background,
                colors.foreground,
                colors.border,
                colors.accent,
            ),
            ButtonVariant::Ghost => (
                transparent(),
                colors.foreground,
                transparent(),
                colors.accent,
            ),
            ButtonVariant::Link => (transparent(), colors.primary, transparent(), transparent()),
            ButtonVariant::Destructive => (
                colors.destructive,
                colors.destructive_foreground,
                colors.destructive,
                with_opacity(colors.destructive, 0.9),
            ),
        };

        // Determine padding and text size based on button size
        let (px_val, py_val, text_size) = match self.size {
            ButtonSize::Xs => (px(8.0), px(4.0), rems(0.75)),
            ButtonSize::Sm => (px(12.0), px(6.0), rems(0.8125)),
            ButtonSize::Default => (px(16.0), px(8.0), rems(0.875)),
            ButtonSize::Lg => (px(24.0), px(10.0), rems(1.0)),
            ButtonSize::Icon => (px(8.0), px(8.0), rems(0.875)),
        };

        let hover_fg = if self.variant == ButtonVariant::Link {
            with_underline_color(colors.primary)
        } else if self.variant == ButtonVariant::Ghost {
            colors.accent_foreground
        } else {
            fg
        };

        let disabled = self.disabled;

        let mut el = apply_radius(
            div().id(self.id).flex().items_center().justify_center(),
            radius,
        )
        .bg(bg)
        .text_color(fg)
        .text_size(text_size)
        .px(px_val)
        .py(py_val)
        .whitespace_nowrap()
        .font_weight(FontWeight::MEDIUM);

        // Apply border for Outline variant
        if self.variant == ButtonVariant::Outline {
            el = el.border_1().border_color(border);
        }

        if disabled {
            el = el.opacity(0.5).cursor_default();
        } else {
            el = el
                .cursor_pointer()
                .hover(move |s| s.bg(hover_bg).text_color(hover_fg))
                .active(move |s| s.opacity(0.8));

            if let Some(handler) = self.on_click {
                el = el.on_click(handler);
            }
        }

        el.child(self.label)
    }
}

/// Apply the theme's border radius to a styled element.
fn apply_radius(el: Stateful<Div>, radius: Radius) -> Stateful<Div> {
    match radius {
        Radius::None => el.rounded_none(),
        Radius::Sm => el.rounded_sm(),
        Radius::Md => el.rounded_md(),
        Radius::Lg => el.rounded_lg(),
        Radius::Full => el.rounded_full(),
    }
}

/// Create a transparent Hsla color.
fn transparent() -> Hsla {
    Hsla {
        h: 0.0,
        s: 0.0,
        l: 0.0,
        a: 0.0,
    }
}

/// Apply a different opacity to an existing color.
fn with_opacity(color: Hsla, a: f32) -> Hsla {
    Hsla { a, ..color }
}

/// For Link variant hover, we just return the same color
/// (underline styling is limited in GPUI, so we darken slightly).
fn with_underline_color(color: Hsla) -> Hsla {
    Hsla {
        l: (color.l - 0.1).max(0.0),
        ..color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_builder() {
        let btn = Button::new("Test")
            .variant(ButtonVariant::Outline)
            .size(ButtonSize::Lg)
            .disabled(true);

        assert_eq!(btn.label, SharedString::from("Test"));
        assert_eq!(btn.variant, ButtonVariant::Outline);
        assert_eq!(btn.size, ButtonSize::Lg);
        assert!(btn.disabled);
    }

    #[test]
    fn test_button_defaults() {
        let btn = Button::new("Default");
        assert_eq!(btn.variant, ButtonVariant::Default);
        assert_eq!(btn.size, ButtonSize::Default);
        assert!(!btn.disabled);
        assert!(btn.on_click.is_none());
    }
}
