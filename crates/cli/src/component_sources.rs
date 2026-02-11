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
        // Phase 2
        "alert" => Some(include_str!("../../../components/alert.rs")),
        "alert_dialog" => Some(include_str!("../../../components/alert_dialog.rs")),
        "tooltip" => Some(include_str!("../../../components/tooltip.rs")),
        "popover" => Some(include_str!("../../../components/popover.rs")),
        "hover_card" => Some(include_str!("../../../components/hover_card.rs")),
        "dropdown_menu" => Some(include_str!("../../../components/dropdown_menu.rs")),
        "sheet" => Some(include_str!("../../../components/sheet.rs")),
        "drawer" => Some(include_str!("../../../components/drawer.rs")),
        "toast" => Some(include_str!("../../../components/toast.rs")),
        "sonner" => Some(include_str!("../../../components/sonner.rs")),
        _ => None,
    }
}
