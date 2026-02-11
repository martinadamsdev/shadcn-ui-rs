//! Preset themes matching shadcn/ui

use crate::{hsl, Theme, ThemeColors, ThemeMode, Radius};

/// Zinc theme (default)
pub fn zinc(mode: ThemeMode) -> Theme {
    let colors = match mode {
        ThemeMode::Light => ThemeColors {
            background: hsl(0.0, 0.0, 100.0),
            foreground: hsl(240.0, 10.0, 3.9),
            card: hsl(0.0, 0.0, 100.0),
            card_foreground: hsl(240.0, 10.0, 3.9),
            popover: hsl(0.0, 0.0, 100.0),
            popover_foreground: hsl(240.0, 10.0, 3.9),
            primary: hsl(240.0, 5.9, 10.0),
            primary_foreground: hsl(0.0, 0.0, 98.0),
            secondary: hsl(240.0, 4.8, 95.9),
            secondary_foreground: hsl(240.0, 5.9, 10.0),
            muted: hsl(240.0, 4.8, 95.9),
            muted_foreground: hsl(240.0, 3.8, 46.1),
            accent: hsl(240.0, 4.8, 95.9),
            accent_foreground: hsl(240.0, 5.9, 10.0),
            destructive: hsl(0.0, 84.2, 60.2),
            destructive_foreground: hsl(0.0, 0.0, 98.0),
            border: hsl(240.0, 5.9, 90.0),
            input: hsl(240.0, 5.9, 90.0),
            ring: hsl(240.0, 5.9, 10.0),
        },
        ThemeMode::Dark => ThemeColors {
            background: hsl(240.0, 10.0, 3.9),
            foreground: hsl(0.0, 0.0, 98.0),
            card: hsl(240.0, 10.0, 3.9),
            card_foreground: hsl(0.0, 0.0, 98.0),
            popover: hsl(240.0, 10.0, 3.9),
            popover_foreground: hsl(0.0, 0.0, 98.0),
            primary: hsl(0.0, 0.0, 98.0),
            primary_foreground: hsl(240.0, 5.9, 10.0),
            secondary: hsl(240.0, 3.7, 15.9),
            secondary_foreground: hsl(0.0, 0.0, 98.0),
            muted: hsl(240.0, 3.7, 15.9),
            muted_foreground: hsl(240.0, 5.0, 64.9),
            accent: hsl(240.0, 3.7, 15.9),
            accent_foreground: hsl(0.0, 0.0, 98.0),
            destructive: hsl(0.0, 62.8, 30.6),
            destructive_foreground: hsl(0.0, 0.0, 98.0),
            border: hsl(240.0, 3.7, 15.9),
            input: hsl(240.0, 3.7, 15.9),
            ring: hsl(240.0, 4.9, 83.9),
        },
    };

    Theme {
        name: "zinc".to_string(),
        mode,
        colors,
        radius: Radius::Md,
    }
}

/// Slate theme
pub fn slate(mode: ThemeMode) -> Theme {
    // TODO: Implement slate colors
    let mut theme = zinc(mode);
    theme.name = "slate".to_string();
    theme
}

/// Stone theme
pub fn stone(mode: ThemeMode) -> Theme {
    // TODO: Implement stone colors
    let mut theme = zinc(mode);
    theme.name = "stone".to_string();
    theme
}

/// Gray theme
pub fn gray(mode: ThemeMode) -> Theme {
    // TODO: Implement gray colors
    let mut theme = zinc(mode);
    theme.name = "gray".to_string();
    theme
}

/// Neutral theme
pub fn neutral(mode: ThemeMode) -> Theme {
    // TODO: Implement neutral colors
    let mut theme = zinc(mode);
    theme.name = "neutral".to_string();
    theme
}
