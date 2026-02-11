//! Preset themes matching shadcn/ui
//!
//! Color values sourced from the official shadcn/ui base color definitions.

use crate::{Radius, Theme, ThemeColors, ThemeMode, hsl};

/// Zinc theme (default) - cool gray with subtle blue tint
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

/// Slate theme - strong blue-gray tint
pub fn slate(mode: ThemeMode) -> Theme {
    let colors = match mode {
        ThemeMode::Light => ThemeColors {
            background: hsl(0.0, 0.0, 100.0),
            foreground: hsl(222.2, 84.0, 4.9),
            card: hsl(0.0, 0.0, 100.0),
            card_foreground: hsl(222.2, 84.0, 4.9),
            popover: hsl(0.0, 0.0, 100.0),
            popover_foreground: hsl(222.2, 84.0, 4.9),
            primary: hsl(222.2, 47.4, 11.2),
            primary_foreground: hsl(210.0, 40.0, 98.0),
            secondary: hsl(210.0, 40.0, 96.1),
            secondary_foreground: hsl(222.2, 47.4, 11.2),
            muted: hsl(210.0, 40.0, 96.1),
            muted_foreground: hsl(215.4, 16.3, 46.9),
            accent: hsl(210.0, 40.0, 96.1),
            accent_foreground: hsl(222.2, 47.4, 11.2),
            destructive: hsl(0.0, 84.2, 60.2),
            destructive_foreground: hsl(210.0, 40.0, 98.0),
            border: hsl(214.3, 31.8, 91.4),
            input: hsl(214.3, 31.8, 91.4),
            ring: hsl(222.2, 84.0, 4.9),
        },
        ThemeMode::Dark => ThemeColors {
            background: hsl(222.2, 84.0, 4.9),
            foreground: hsl(210.0, 40.0, 98.0),
            card: hsl(222.2, 84.0, 4.9),
            card_foreground: hsl(210.0, 40.0, 98.0),
            popover: hsl(222.2, 84.0, 4.9),
            popover_foreground: hsl(210.0, 40.0, 98.0),
            primary: hsl(210.0, 40.0, 98.0),
            primary_foreground: hsl(222.2, 47.4, 11.2),
            secondary: hsl(217.2, 32.6, 17.5),
            secondary_foreground: hsl(210.0, 40.0, 98.0),
            muted: hsl(217.2, 32.6, 17.5),
            muted_foreground: hsl(215.0, 20.2, 65.1),
            accent: hsl(217.2, 32.6, 17.5),
            accent_foreground: hsl(210.0, 40.0, 98.0),
            destructive: hsl(0.0, 62.8, 30.6),
            destructive_foreground: hsl(210.0, 40.0, 98.0),
            border: hsl(217.2, 32.6, 17.5),
            input: hsl(217.2, 32.6, 17.5),
            ring: hsl(212.7, 26.8, 83.9),
        },
    };

    Theme {
        name: "slate".to_string(),
        mode,
        colors,
        radius: Radius::Md,
    }
}

/// Stone theme - warm gray with brown tint
pub fn stone(mode: ThemeMode) -> Theme {
    let colors = match mode {
        ThemeMode::Light => ThemeColors {
            background: hsl(0.0, 0.0, 100.0),
            foreground: hsl(20.0, 14.3, 4.1),
            card: hsl(0.0, 0.0, 100.0),
            card_foreground: hsl(20.0, 14.3, 4.1),
            popover: hsl(0.0, 0.0, 100.0),
            popover_foreground: hsl(20.0, 14.3, 4.1),
            primary: hsl(24.0, 9.8, 10.0),
            primary_foreground: hsl(60.0, 9.1, 97.8),
            secondary: hsl(60.0, 4.8, 95.9),
            secondary_foreground: hsl(24.0, 9.8, 10.0),
            muted: hsl(60.0, 4.8, 95.9),
            muted_foreground: hsl(25.0, 5.3, 44.7),
            accent: hsl(60.0, 4.8, 95.9),
            accent_foreground: hsl(24.0, 9.8, 10.0),
            destructive: hsl(0.0, 84.2, 60.2),
            destructive_foreground: hsl(60.0, 9.1, 97.8),
            border: hsl(20.0, 5.9, 90.0),
            input: hsl(20.0, 5.9, 90.0),
            ring: hsl(20.0, 14.3, 4.1),
        },
        ThemeMode::Dark => ThemeColors {
            background: hsl(20.0, 14.3, 4.1),
            foreground: hsl(60.0, 9.1, 97.8),
            card: hsl(20.0, 14.3, 4.1),
            card_foreground: hsl(60.0, 9.1, 97.8),
            popover: hsl(20.0, 14.3, 4.1),
            popover_foreground: hsl(60.0, 9.1, 97.8),
            primary: hsl(60.0, 9.1, 97.8),
            primary_foreground: hsl(24.0, 9.8, 10.0),
            secondary: hsl(12.0, 6.5, 15.1),
            secondary_foreground: hsl(60.0, 9.1, 97.8),
            muted: hsl(12.0, 6.5, 15.1),
            muted_foreground: hsl(24.0, 5.4, 63.9),
            accent: hsl(12.0, 6.5, 15.1),
            accent_foreground: hsl(60.0, 9.1, 97.8),
            destructive: hsl(0.0, 62.8, 30.6),
            destructive_foreground: hsl(60.0, 9.1, 97.8),
            border: hsl(12.0, 6.5, 15.1),
            input: hsl(12.0, 6.5, 15.1),
            ring: hsl(24.0, 5.7, 82.9),
        },
    };

    Theme {
        name: "stone".to_string(),
        mode,
        colors,
        radius: Radius::Lg,
    }
}

/// Gray theme - medium blue-gray
pub fn gray(mode: ThemeMode) -> Theme {
    let colors = match mode {
        ThemeMode::Light => ThemeColors {
            background: hsl(0.0, 0.0, 100.0),
            foreground: hsl(224.0, 71.4, 4.1),
            card: hsl(0.0, 0.0, 100.0),
            card_foreground: hsl(224.0, 71.4, 4.1),
            popover: hsl(0.0, 0.0, 100.0),
            popover_foreground: hsl(224.0, 71.4, 4.1),
            primary: hsl(220.9, 39.3, 11.0),
            primary_foreground: hsl(210.0, 20.0, 98.0),
            secondary: hsl(220.0, 14.3, 95.9),
            secondary_foreground: hsl(220.9, 39.3, 11.0),
            muted: hsl(220.0, 14.3, 95.9),
            muted_foreground: hsl(220.0, 8.9, 46.1),
            accent: hsl(220.0, 14.3, 95.9),
            accent_foreground: hsl(220.9, 39.3, 11.0),
            destructive: hsl(0.0, 84.2, 60.2),
            destructive_foreground: hsl(210.0, 20.0, 98.0),
            border: hsl(220.0, 13.0, 91.0),
            input: hsl(220.0, 13.0, 91.0),
            ring: hsl(224.0, 71.4, 4.1),
        },
        ThemeMode::Dark => ThemeColors {
            background: hsl(224.0, 71.4, 4.1),
            foreground: hsl(210.0, 20.0, 98.0),
            card: hsl(224.0, 71.4, 4.1),
            card_foreground: hsl(210.0, 20.0, 98.0),
            popover: hsl(224.0, 71.4, 4.1),
            popover_foreground: hsl(210.0, 20.0, 98.0),
            primary: hsl(210.0, 20.0, 98.0),
            primary_foreground: hsl(220.9, 39.3, 11.0),
            secondary: hsl(215.0, 27.9, 16.9),
            secondary_foreground: hsl(210.0, 20.0, 98.0),
            muted: hsl(215.0, 27.9, 16.9),
            muted_foreground: hsl(217.9, 10.6, 64.9),
            accent: hsl(215.0, 27.9, 16.9),
            accent_foreground: hsl(210.0, 20.0, 98.0),
            destructive: hsl(0.0, 62.8, 30.6),
            destructive_foreground: hsl(210.0, 20.0, 98.0),
            border: hsl(215.0, 27.9, 16.9),
            input: hsl(215.0, 27.9, 16.9),
            ring: hsl(216.0, 12.2, 83.9),
        },
    };

    Theme {
        name: "gray".to_string(),
        mode,
        colors,
        radius: Radius::Sm,
    }
}

/// Neutral theme - true grayscale with no color tint
pub fn neutral(mode: ThemeMode) -> Theme {
    let colors = match mode {
        ThemeMode::Light => ThemeColors {
            background: hsl(0.0, 0.0, 100.0),
            foreground: hsl(0.0, 0.0, 3.9),
            card: hsl(0.0, 0.0, 100.0),
            card_foreground: hsl(0.0, 0.0, 3.9),
            popover: hsl(0.0, 0.0, 100.0),
            popover_foreground: hsl(0.0, 0.0, 3.9),
            primary: hsl(0.0, 0.0, 9.0),
            primary_foreground: hsl(0.0, 0.0, 98.0),
            secondary: hsl(0.0, 0.0, 96.1),
            secondary_foreground: hsl(0.0, 0.0, 9.0),
            muted: hsl(0.0, 0.0, 96.1),
            muted_foreground: hsl(0.0, 0.0, 45.1),
            accent: hsl(0.0, 0.0, 96.1),
            accent_foreground: hsl(0.0, 0.0, 9.0),
            destructive: hsl(0.0, 84.2, 60.2),
            destructive_foreground: hsl(0.0, 0.0, 98.0),
            border: hsl(0.0, 0.0, 89.8),
            input: hsl(0.0, 0.0, 89.8),
            ring: hsl(0.0, 0.0, 3.9),
        },
        ThemeMode::Dark => ThemeColors {
            background: hsl(0.0, 0.0, 3.9),
            foreground: hsl(0.0, 0.0, 98.0),
            card: hsl(0.0, 0.0, 3.9),
            card_foreground: hsl(0.0, 0.0, 98.0),
            popover: hsl(0.0, 0.0, 3.9),
            popover_foreground: hsl(0.0, 0.0, 98.0),
            primary: hsl(0.0, 0.0, 98.0),
            primary_foreground: hsl(0.0, 0.0, 9.0),
            secondary: hsl(0.0, 0.0, 14.9),
            secondary_foreground: hsl(0.0, 0.0, 98.0),
            muted: hsl(0.0, 0.0, 14.9),
            muted_foreground: hsl(0.0, 0.0, 63.9),
            accent: hsl(0.0, 0.0, 14.9),
            accent_foreground: hsl(0.0, 0.0, 98.0),
            destructive: hsl(0.0, 62.8, 30.6),
            destructive_foreground: hsl(0.0, 0.0, 98.0),
            border: hsl(0.0, 0.0, 14.9),
            input: hsl(0.0, 0.0, 14.9),
            ring: hsl(0.0, 0.0, 83.1),
        },
    };

    Theme {
        name: "neutral".to_string(),
        mode,
        colors,
        radius: Radius::Md,
    }
}

/// All available preset names
pub fn preset_names() -> &'static [&'static str] {
    &["zinc", "slate", "stone", "gray", "neutral"]
}

/// Get a preset theme by name and mode
pub fn get_preset(name: &str, mode: ThemeMode) -> Option<Theme> {
    match name {
        "zinc" => Some(zinc(mode)),
        "slate" => Some(slate(mode)),
        "stone" => Some(stone(mode)),
        "gray" => Some(gray(mode)),
        "neutral" => Some(neutral(mode)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_presets_light() {
        for name in preset_names() {
            let theme = get_preset(name, ThemeMode::Light).unwrap();
            assert_eq!(theme.name, *name);
            assert_eq!(theme.mode, ThemeMode::Light);
        }
    }

    #[test]
    fn test_all_presets_dark() {
        for name in preset_names() {
            let theme = get_preset(name, ThemeMode::Dark).unwrap();
            assert_eq!(theme.name, *name);
            assert_eq!(theme.mode, ThemeMode::Dark);
        }
    }

    #[test]
    fn test_unknown_preset_returns_none() {
        assert!(get_preset("unknown", ThemeMode::Light).is_none());
    }

    #[test]
    fn test_preset_names_complete() {
        let names = preset_names();
        assert_eq!(names.len(), 5);
        assert!(names.contains(&"zinc"));
        assert!(names.contains(&"slate"));
        assert!(names.contains(&"stone"));
        assert!(names.contains(&"gray"));
        assert!(names.contains(&"neutral"));
    }

    #[test]
    fn test_zinc_is_default() {
        let default_theme = Theme::default();
        assert_eq!(default_theme.name, "zinc");
        assert_eq!(default_theme.mode, ThemeMode::Light);
    }

    #[test]
    fn test_light_backgrounds_are_white() {
        for name in preset_names() {
            let theme = get_preset(name, ThemeMode::Light).unwrap();
            // All light themes have white background: hsl(0, 0%, 100%)
            assert_eq!(
                theme.colors.background.l, 1.0,
                "{name} light background should be white"
            );
        }
    }

    #[test]
    fn test_dark_backgrounds_are_dark() {
        for name in preset_names() {
            let theme = get_preset(name, ThemeMode::Dark).unwrap();
            // All dark themes have very low lightness backgrounds
            assert!(
                theme.colors.background.l < 0.1,
                "{name} dark background lightness should be < 10%"
            );
        }
    }

    #[test]
    fn test_neutral_has_zero_saturation() {
        let theme = neutral(ThemeMode::Light);
        // Neutral theme uses 0% saturation for primary colors
        assert_eq!(
            theme.colors.primary.s, 0.0,
            "neutral primary should have 0 saturation"
        );
        assert_eq!(
            theme.colors.foreground.s, 0.0,
            "neutral foreground should have 0 saturation"
        );
    }

    #[test]
    fn test_slate_has_blue_tint() {
        let theme = slate(ThemeMode::Light);
        // Slate foreground has hue ~222 (blue range)
        let hue_degrees = theme.colors.foreground.h * 360.0;
        assert!(
            (200.0..240.0).contains(&hue_degrees),
            "slate foreground hue should be in blue range, got {hue_degrees}"
        );
    }

    #[test]
    fn test_stone_has_warm_tint() {
        let theme = stone(ThemeMode::Light);
        // Stone foreground has hue ~20 (warm/orange range)
        let hue_degrees = theme.colors.foreground.h * 360.0;
        assert!(
            (10.0..40.0).contains(&hue_degrees),
            "stone foreground hue should be in warm range, got {hue_degrees}"
        );
    }
}
