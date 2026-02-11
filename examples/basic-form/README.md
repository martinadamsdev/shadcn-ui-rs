# Basic Form Example

A complete GPUI application showcasing all 12 Phase 1 shadcn-ui-rs components
in a form layout.

## Components Demonstrated

1. **Button** - Multiple variants (Default, Outline, Secondary, Destructive)
2. **Input** - Text input with placeholder
3. **Label** - Form labels with required indicator
4. **Checkbox** - Terms acceptance toggle
5. **RadioGroup** - Plan selection (Free / Pro / Enterprise)
6. **Switch** - Notifications on/off toggle
7. **Slider** - Volume control
8. **Select** - Dropdown fruit picker
9. **Toggle** - Bold/Italic text formatting toggles
10. **ToggleGroup** - Alignment selection (Left / Center / Right)
11. **Card** - Form container with header, content, and footer
12. **Dialog** - Modal edit profile dialog

## Running

```bash
cd examples/basic-form
cargo run
```

## Structure

```
src/
├── main.rs              # App entry, window setup, root form view
├── theme.rs             # Self-contained theme (zinc preset)
└── components/
    └── ui/
        ├── mod.rs       # Re-exports all components
        ├── button.rs
        ├── card.rs
        ├── checkbox.rs
        ├── dialog.rs
        ├── input.rs
        ├── label.rs
        ├── radio.rs
        ├── select.rs
        ├── slider.rs
        ├── switch.rs
        ├── toggle.rs
        └── toggle_group.rs
```

This example is a standalone project and is not part of the workspace.
