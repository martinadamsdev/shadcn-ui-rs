//! Color utilities and conversions

use gpui::Hsla;

/// Convert HSL values to GPUI's Hsla
pub fn hsl(h: f32, s: f32, l: f32) -> Hsla {
    Hsla {
        h: h / 360.0,
        s: s / 100.0,
        l: l / 100.0,
        a: 1.0,
    }
}

/// Convert HSL with alpha to GPUI's Hsla
pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Hsla {
    Hsla {
        h: h / 360.0,
        s: s / 100.0,
        l: l / 100.0,
        a,
    }
}
