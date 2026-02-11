//! Theme system for shadcn-ui-rs
//!
//! Provides runtime-switchable themes with compile-time optimization options.

use gpui::Hsla;
use serde::{Deserialize, Serialize};

mod colors;
mod presets;

pub use colors::*;
pub use presets::*;

/// Theme mode (light or dark)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

/// Border radius presets
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum Radius {
    None,
    Sm,
    #[default]
    Md,
    Lg,
    Full,
}

impl Radius {
    pub fn to_px(self) -> f32 {
        match self {
            Radius::None => 0.0,
            Radius::Sm => 4.0,
            Radius::Md => 6.0,
            Radius::Lg => 8.0,
            Radius::Full => 9999.0,
        }
    }
}

/// Complete theme definition
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub mode: ThemeMode,
    pub colors: ThemeColors,
    pub radius: Radius,
}

/// Theme color palette
#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub card: Hsla,
    pub card_foreground: Hsla,
    pub popover: Hsla,
    pub popover_foreground: Hsla,
    pub primary: Hsla,
    pub primary_foreground: Hsla,
    pub secondary: Hsla,
    pub secondary_foreground: Hsla,
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    pub accent: Hsla,
    pub accent_foreground: Hsla,
    pub destructive: Hsla,
    pub destructive_foreground: Hsla,
    pub border: Hsla,
    pub input: Hsla,
    pub ring: Hsla,
}

impl Default for Theme {
    fn default() -> Self {
        presets::zinc(ThemeMode::Light)
    }
}
