//! Toggle component for shadcn-ui-rs
//!
//! A two-state button that can be either on or off.
//!
//! # Example
//!
//! ```rust
//! Toggle::new("bold-toggle")
//!     .pressed(true)
//!     .variant(ToggleVariant::Outline)
//!     .size(ToggleSize::Default)
//!     .on_press_change(|pressed, window, cx| {
//!         println!("Toggle is now: {}", pressed);
//!     })
//!     .child("B")
//! ```

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, Hsla, IntoElement, ParentElement,
    RenderOnce, Styled, Window, div, prelude::*,
};
use crate::theme::Theme;

/// Toggle visual variant
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum ToggleVariant {
    /// Default variant with transparent background
    #[default]
    Default,
    /// Outline variant with border
    Outline,
}

/// Toggle size presets
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum ToggleSize {
    /// Small: h-9 px-2.5
    Sm,
    /// Default: h-10 px-3
    #[default]
    Default,
    /// Large: h-11 px-5
    Lg,
}

/// A two-state toggle button component.
///
/// Toggles can be pressed or unpressed, and support variants and sizes
/// matching the shadcn/ui Toggle component.
#[derive(IntoElement)]
pub struct Toggle {
    id: ElementId,
    pressed: bool,
    variant: ToggleVariant,
    size: ToggleSize,
    disabled: bool,
    on_press_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl Toggle {
    /// Create a new toggle with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            pressed: false,
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            disabled: false,
            on_press_change: None,
            children: Vec::new(),
        }
    }

    /// Set the pressed state.
    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    /// Set the visual variant.
    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the size.
    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    /// Set the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the callback for when the pressed state changes.
    pub fn on_press_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_press_change = Some(Box::new(handler));
        self
    }
}

impl ParentElement for Toggle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Toggle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        // Determine background and text colors based on pressed state
        let (bg_color, text_color): (Hsla, Hsla) = if self.pressed {
            (colors.accent, colors.accent_foreground)
        } else {
            (gpui::transparent_black(), colors.foreground)
        };

        // Build the base element
        let mut el = div()
            .id(self.id.clone())
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .cursor_pointer()
            .bg(bg_color)
            .text_color(text_color);

        // Apply size
        match self.size {
            ToggleSize::Sm => {
                el = el
                    .h(gpui::px(36.0))
                    .px(gpui::px(10.0))
                    .min_w(gpui::px(36.0));
            }
            ToggleSize::Default => {
                el = el
                    .h(gpui::px(40.0))
                    .px(gpui::px(12.0))
                    .min_w(gpui::px(40.0));
            }
            ToggleSize::Lg => {
                el = el
                    .h(gpui::px(44.0))
                    .px(gpui::px(20.0))
                    .min_w(gpui::px(44.0));
            }
        }

        // Apply variant-specific styles
        match self.variant {
            ToggleVariant::Default => {}
            ToggleVariant::Outline => {
                el = el.border_1().border_color(colors.input);
                if self.pressed {
                    el = el.bg(colors.accent);
                }
            }
        }

        // Apply disabled styling
        if self.disabled {
            el = el.opacity(0.5).cursor_default();
        }

        // Hover effect (only when not disabled)
        let hover_bg = colors.muted;
        let accent_bg = colors.accent;
        let pressed = self.pressed;
        if !self.disabled {
            el = el.hover(move |style| {
                if pressed {
                    style.bg(accent_bg)
                } else {
                    style.bg(hover_bg)
                }
            });
        }

        // Click handler
        if !self.disabled {
            if let Some(on_press_change) = self.on_press_change {
                let new_pressed = !self.pressed;
                el = el.on_click(
                    move |_event: &ClickEvent, window: &mut Window, cx: &mut App| {
                        on_press_change(new_pressed, window, cx);
                    },
                );
            }
        }

        // Add children
        el.children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_builder() {
        let toggle = Toggle::new("test")
            .pressed(true)
            .variant(ToggleVariant::Outline)
            .size(ToggleSize::Lg)
            .disabled(false);

        assert!(toggle.pressed);
        assert_eq!(toggle.variant, ToggleVariant::Outline);
        assert_eq!(toggle.size, ToggleSize::Lg);
        assert!(!toggle.disabled);
    }

    #[test]
    fn test_toggle_defaults() {
        let toggle = Toggle::new("test");

        assert!(!toggle.pressed);
        assert_eq!(toggle.variant, ToggleVariant::Default);
        assert_eq!(toggle.size, ToggleSize::Default);
        assert!(!toggle.disabled);
    }
}
