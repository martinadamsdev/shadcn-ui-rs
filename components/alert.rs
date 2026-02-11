//! Alert component for shadcn-ui-rs
//!
//! A pure layout component for displaying callout messages with optional
//! destructive styling.
//!
//! # Example
//!
//! ```rust
//! Alert::new()
//!     .variant(AlertVariant::Destructive)
//!     .child(AlertTitle::new("Error"))
//!     .child(AlertDescription::new("Your session has expired."))
//! ```

use crate::theme::Theme;
use gpui::{
    div, AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window,
};

/// Alert visual variant.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AlertVariant {
    /// Default alert with standard border and text colors.
    #[default]
    Default,
    /// Destructive alert with destructive border and text colors.
    Destructive,
}

/// Alert container component.
///
/// Displays a themed container with border, rounded corners, and padding.
/// Supports Default and Destructive variants.
#[derive(IntoElement)]
pub struct Alert {
    variant: AlertVariant,
    children: Vec<AnyElement>,
}

impl Alert {
    /// Create a new alert with default variant.
    pub fn new() -> Self {
        Self {
            variant: AlertVariant::Default,
            children: Vec::new(),
        }
    }

    /// Set the alert variant.
    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl ParentElement for Alert {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Alert {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let colors = &theme.colors;

        let (bg, text, border) = match self.variant {
            AlertVariant::Default => (colors.background, colors.foreground, colors.border),
            AlertVariant::Destructive => {
                (colors.background, colors.destructive, colors.destructive)
            }
        };

        div()
            .flex()
            .flex_col()
            .gap(gpui::px(4.0))
            .rounded_lg()
            .border_1()
            .border_color(border)
            .bg(bg)
            .text_color(text)
            .p(gpui::px(16.0))
            .children(self.children)
    }
}

/// Alert title text.
///
/// Renders with semibold font weight and small text size.
#[derive(IntoElement)]
pub struct AlertTitle {
    text: SharedString,
}

impl AlertTitle {
    /// Create a new alert title.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for AlertTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .text_sm()
            .font_weight(FontWeight::SEMIBOLD)
            .line_height(gpui::rems(1.25))
            .child(self.text)
    }
}

/// Alert description text.
///
/// Renders with small text size. Inherits text color from the parent Alert.
#[derive(IntoElement)]
pub struct AlertDescription {
    text: SharedString,
}

impl AlertDescription {
    /// Create a new alert description.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl RenderOnce for AlertDescription {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().text_sm().child(self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_defaults() {
        let alert = Alert::new();
        assert_eq!(alert.variant, AlertVariant::Default);
        assert!(alert.children.is_empty());
    }

    #[test]
    fn test_alert_destructive() {
        let alert = Alert::new().variant(AlertVariant::Destructive);
        assert_eq!(alert.variant, AlertVariant::Destructive);
    }

    #[test]
    fn test_alert_builder() {
        let alert = Alert::new().variant(AlertVariant::Destructive);
        assert_eq!(alert.variant, AlertVariant::Destructive);
    }

    #[test]
    fn test_alert_title() {
        let title = AlertTitle::new("Test Title");
        assert_eq!(title.text, SharedString::from("Test Title"));
    }

    #[test]
    fn test_alert_description() {
        let desc = AlertDescription::new("Test description");
        assert_eq!(desc.text, SharedString::from("Test description"));
    }
}
