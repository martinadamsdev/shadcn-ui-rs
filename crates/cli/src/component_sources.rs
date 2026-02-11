//! Embedded component source code.
//!
//! Uses `include_str!()` to embed the real component `.rs` files into the CLI
//! binary at compile time so that `add` can copy them into user projects.

/// Return the embedded source code for a component, or `None` if unknown.
pub fn get_component_source(name: &str) -> Option<&'static str> {
    match name {
        "button" => Some(include_str!("../../../components/button.rs")),
        "input" => Some(include_str!("../../../components/input.rs")),
        "label" => Some(include_str!("../../../components/label.rs")),
        "checkbox" => Some(include_str!("../../../components/checkbox.rs")),
        "radio" => Some(include_str!("../../../components/radio.rs")),
        "switch" => Some(include_str!("../../../components/switch.rs")),
        "slider" => Some(include_str!("../../../components/slider.rs")),
        "select" => Some(include_str!("../../../components/select.rs")),
        "toggle" => Some(include_str!("../../../components/toggle.rs")),
        "toggle_group" => Some(include_str!("../../../components/toggle_group.rs")),
        "card" => Some(include_str!("../../../components/card.rs")),
        "dialog" => Some(include_str!("../../../components/dialog.rs")),
        _ => None,
    }
}
