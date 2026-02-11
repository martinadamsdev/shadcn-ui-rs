# Component Reference

All components use a builder pattern API. They read theme colors from the GPUI global `Theme` at render time.

---

## Button

A button with multiple visual variants and sizes.

**Variants:** `Default`, `Secondary`, `Outline`, `Ghost`, `Link`, `Destructive`

**Sizes:** `Xs`, `Sm`, `Default`, `Lg`, `Icon`

```rust
use components::ui::{Button, ButtonVariant, ButtonSize};

// Default button
Button::new("Save")

// Styled button with click handler
Button::new("Delete")
    .variant(ButtonVariant::Destructive)
    .size(ButtonSize::Sm)
    .on_click(|_event, _window, _cx| {
        println!("Deleted!");
    })

// Disabled button
Button::new("Submit")
    .disabled(true)
```

**Builder methods:**

| Method | Type | Description |
|--------|------|-------------|
| `new(label)` | `impl Into<SharedString>` | Create with label text |
| `.id(id)` | `impl Into<ElementId>` | Override the element ID |
| `.variant(v)` | `ButtonVariant` | Set visual variant |
| `.size(s)` | `ButtonSize` | Set size preset |
| `.disabled(b)` | `bool` | Disable the button |
| `.on_click(f)` | `Fn(&ClickEvent, &mut Window, &mut App)` | Click handler |

---

## Input

A text input field with placeholder support.

```rust
use components::ui::Input;

Input::new("email-input")
    .placeholder("Enter your email")
    .value("hello@example.com")

Input::new("disabled-input")
    .placeholder("Read only")
    .disabled(true)
```

**Builder methods:**

| Method | Type | Description |
|--------|------|-------------|
| `new(id)` | `impl Into<ElementId>` | Create with element ID |
| `.placeholder(text)` | `impl Into<SharedString>` | Placeholder text |
| `.value(text)` | `impl Into<SharedString>` | Current value |
| `.disabled(b)` | `bool` | Disable the input |

---

## Label

A text label for form fields.

```rust
use components::ui::Label;

Label::new("Email address")

Label::new("Password").required(true)
```

**Builder methods:**

| Method | Type | Description |
|--------|------|-------------|
| `new(text)` | `impl Into<SharedString>` | Create with label text |
| `.required(b)` | `bool` | Show asterisk for required fields |

---

## Checkbox

A checkbox with checked/unchecked states.

```rust
use components::ui::Checkbox;

Checkbox::new("accept-terms")
    .checked(true)
    .on_toggle(|checked, _window, _cx| {
        println!("checked: {}", checked);
    })

Checkbox::new("disabled-cb")
    .checked(false)
    .disabled(true)
```

**Builder methods:**

| Method | Type | Description |
|--------|------|-------------|
| `new(id)` | `impl Into<ElementId>` | Create with element ID |
| `.checked(b)` | `bool` | Set checked state |
| `.disabled(b)` | `bool` | Disable the checkbox |
| `.on_toggle(f)` | `Fn(bool, &mut Window, &mut App)` | Toggle handler (receives new state) |

---

## Radio

A radio group for selecting one option from a set.

```rust
use components::ui::{RadioGroup, RadioItem};

RadioGroup::new("size-group")
    .value("medium")
    .on_change(|value, _window, _cx| {
        println!("Selected: {value}");
    })
    .child(RadioItem::new("small", "Small"))
    .child(RadioItem::new("medium", "Medium"))
    .child(RadioItem::new("large", "Large"))
```

**RadioGroup builder methods:**

| Method | Type | Description |
|--------|------|-------------|
| `new(id)` | `impl Into<ElementId>` | Create with element ID |
| `.value(v)` | `impl Into<SharedString>` | Currently selected value |
| `.disabled(b)` | `bool` | Disable the entire group |
| `.child(item)` | `RadioItem` | Add a single radio item |
| `.children(items)` | `impl IntoIterator<Item = RadioItem>` | Add multiple items |
| `.on_change(f)` | `Fn(&str, &mut Window, &mut App)` | Selection change handler |

**RadioItem builder methods:**

| Method | Type | Description |
|--------|------|-------------|
| `new(value, label)` | `impl Into<SharedString>` | Create with value and display label |
| `.disabled(b)` | `bool` | Disable this item |

---

## Switch

A binary on/off toggle switch.

```rust
use components::ui::Switch;

Switch::new("notifications")
    .checked(true)
    .on_change(|checked, _window, _cx| {
        println!("Switch is now: {checked}");
    })
```

**Builder methods:**

| Method | Type | Description |
|--------|------|-------------|
| `new(id)` | `impl Into<ElementId>` | Create with element ID |
| `.checked(b)` | `bool` | Set the on/off state |
| `.disabled(b)` | `bool` | Disable the switch |
| `.on_change(f)` | `Fn(bool, &mut Window, &mut App)` | Toggle handler (receives new state) |

---

## Slider

A horizontal range slider for numeric value selection.

```rust
use components::ui::Slider;

Slider::new("volume")
    .min(0.0)
    .max(100.0)
    .value(50.0)
    .step(1.0)
    .on_change(|value, _window, _cx| {
        println!("Volume: {value}");
    })

Slider::new("custom-width")
    .min(0.0)
    .max(1.0)
    .step(0.1)
    .width(300.0)
```

**Builder methods:**

| Method | Type | Description |
|--------|------|-------------|
| `new(id)` | `impl Into<ElementId>` | Create with element ID (default range: 0-100) |
| `.min(v)` | `f32` | Minimum value |
| `.max(v)` | `f32` | Maximum value |
| `.value(v)` | `f32` | Current value |
| `.step(v)` | `f32` | Step increment |
| `.width(v)` | `f32` | Display width in pixels (default: 200) |
| `.disabled(b)` | `bool` | Disable the slider |
| `.on_change(f)` | `Fn(f32, &mut Window, &mut App)` | Value change handler |

---

## Select

A dropdown for selecting a single value from a list.

```rust
use components::ui::{Select, SelectItem};

Select::new("fruit-select")
    .placeholder("Pick a fruit...")
    .value("apple")
    .open(is_open)
    .on_open_change(|open, _window, _cx| {
        // Toggle dropdown visibility
    })
    .on_change(|value, _window, _cx| {
        println!("Selected: {value}");
    })
    .child(SelectItem::new("apple", "Apple"))
    .child(SelectItem::new("banana", "Banana"))
    .child(SelectItem::new("cherry", "Cherry"))
```

This is a **controlled** component: both `value` and `open` state are passed in as props.

**Select builder methods:**

| Method | Type | Description |
|--------|------|-------------|
| `new(id)` | `impl Into<ElementId>` | Create with element ID |
| `.value(v)` | `impl Into<SharedString>` | Currently selected value |
| `.placeholder(text)` | `impl Into<SharedString>` | Placeholder when nothing is selected |
| `.open(b)` | `bool` | Control whether the dropdown is open |
| `.disabled(b)` | `bool` | Disable the select |
| `.child(item)` | `SelectItem` | Add a single option |
| `.children(items)` | `impl IntoIterator<Item = SelectItem>` | Add multiple options |
| `.on_change(f)` | `Fn(&str, &mut Window, &mut App)` | Selection change handler |
| `.on_open_change(f)` | `Fn(bool, &mut Window, &mut App)` | Open/close state change handler |

**SelectItem builder methods:**

| Method | Type | Description |
|--------|------|-------------|
| `new(value, label)` | `impl Into<SharedString>` | Create with value and display label |
| `.disabled(b)` | `bool` | Disable this item |

---

## Toggle

A two-state button that can be pressed or unpressed.

**Variants:** `Default`, `Outline`

**Sizes:** `Sm`, `Default`, `Lg`

```rust
use components::ui::{Toggle, ToggleVariant, ToggleSize};

Toggle::new("bold-toggle")
    .pressed(true)
    .variant(ToggleVariant::Outline)
    .size(ToggleSize::Default)
    .on_press_change(|pressed, _window, _cx| {
        println!("Toggle is now: {pressed}");
    })
    .child("B")
```

**Builder methods:**

| Method | Type | Description |
|--------|------|-------------|
| `new(id)` | `impl Into<ElementId>` | Create with element ID |
| `.pressed(b)` | `bool` | Set pressed state |
| `.variant(v)` | `ToggleVariant` | Set visual variant |
| `.size(s)` | `ToggleSize` | Set size preset |
| `.disabled(b)` | `bool` | Disable the toggle |
| `.on_press_change(f)` | `Fn(bool, &mut Window, &mut App)` | Press state change handler |
| `.child(el)` | GPUI element | Add child content |

---

## ToggleGroup

A group of toggle buttons with single or multiple selection.

**Types:** `Single`, `Multiple`

**Variants:** `Default`, `Outline`

**Sizes:** `Sm`, `Default`, `Lg`

```rust
use components::ui::{ToggleGroup, ToggleGroupItem, ToggleGroupType};

ToggleGroup::new("alignment")
    .type_(ToggleGroupType::Single)
    .value(vec!["left".into()])
    .on_change(|values, _window, _cx| {
        println!("Selected: {:?}", values);
    })
    .child(ToggleGroupItem::new("left", "left").child("Left"))
    .child(ToggleGroupItem::new("center", "center").child("Center"))
    .child(ToggleGroupItem::new("right", "right").child("Right"))
```

**ToggleGroup builder methods:**

| Method | Type | Description |
|--------|------|-------------|
| `new(id)` | `impl Into<ElementId>` | Create with element ID |
| `.type_(t)` | `ToggleGroupType` | Single or Multiple selection |
| `.variant(v)` | `ToggleGroupVariant` | Visual variant for all items |
| `.size(s)` | `ToggleGroupSize` | Size preset for all items |
| `.value(v)` | `Vec<SharedString>` | Currently selected values |
| `.disabled(b)` | `bool` | Disable the entire group |
| `.on_change(f)` | `Fn(Vec<SharedString>, &mut Window, &mut App)` | Selection change handler |
| `.child(item)` | GPUI element | Add a child (typically ToggleGroupItem) |

**ToggleGroupItem builder methods:**

| Method | Type | Description |
|--------|------|-------------|
| `new(id, value)` | `impl Into<ElementId>`, `impl Into<SharedString>` | Create with ID and value |
| `.pressed(b)` | `bool` | Set pressed state |
| `.variant(v)` | `ToggleGroupVariant` | Override variant |
| `.size(s)` | `ToggleGroupSize` | Override size |
| `.disabled(b)` | `bool` | Disable this item |
| `.on_click(f)` | `Fn(&ClickEvent, &mut Window, &mut App)` | Click handler |
| `.child(el)` | GPUI element | Add child content |

---

## Card

A container with header, content, and footer sections.

**Sub-components:** `Card`, `CardHeader`, `CardTitle`, `CardDescription`, `CardContent`, `CardFooter`

```rust
use components::ui::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};

Card::new()
    .child(
        CardHeader::new()
            .child(CardTitle::new("Account Settings"))
            .child(CardDescription::new("Manage your account preferences."))
    )
    .child(
        CardContent::new()
            .child("Main content goes here")
    )
    .child(
        CardFooter::new()
            .child(Button::new("Save").variant(ButtonVariant::Default))
            .child(Button::new("Cancel").variant(ButtonVariant::Outline))
    )
```

| Component | Constructor | Description |
|-----------|------------|-------------|
| `Card` | `Card::new()` | Outer container with border, rounded corners, and shadow |
| `CardHeader` | `CardHeader::new()` | Top section with vertical spacing (padding: 24px) |
| `CardTitle` | `CardTitle::new(text)` | Semibold title text |
| `CardDescription` | `CardDescription::new(text)` | Muted description text |
| `CardContent` | `CardContent::new()` | Main content area (padding: 24px horizontal) |
| `CardFooter` | `CardFooter::new()` | Bottom row for action buttons |

All Card sub-components accept children via `.child()`.

---

## Dialog

A modal overlay with backdrop that displays content on top of the interface.

**Sub-components:** `Dialog`, `DialogTrigger`, `DialogContent`, `DialogHeader`, `DialogTitle`, `DialogDescription`, `DialogFooter`, `DialogClose`

```rust
use components::ui::{
    Dialog, DialogTrigger, DialogContent, DialogHeader,
    DialogTitle, DialogDescription, DialogFooter, DialogClose,
    Button, ButtonVariant,
};

// In a stateful view's render method:
let open = self.dialog_open;

div()
    .child(
        DialogTrigger::new("open-btn")
            .on_click(cx.listener(|this, _event, _window, _cx| {
                this.dialog_open = true;
            }))
            .child("Open Dialog")
    )
    .when(open, |el| {
        el.child(
            Dialog::new("my-dialog")
                .open(true)
                .on_close(cx.listener(|this, _window, _cx| {
                    this.dialog_open = false;
                }))
                .child(
                    DialogContent::new()
                        .child(
                            DialogHeader::new()
                                .child(DialogTitle::new("Edit Profile"))
                                .child(DialogDescription::new(
                                    "Make changes to your profile here."
                                ))
                        )
                        .child("Dialog body content")
                        .child(
                            DialogFooter::new()
                                .child(
                                    DialogClose::new()
                                        .on_click(cx.listener(|this, _event, _window, _cx| {
                                            this.dialog_open = false;
                                        }))
                                        .child(Button::new("Save"))
                                )
                        )
                )
        )
    })
```

The Dialog closes when clicking the backdrop or pressing Escape.

| Component | Constructor | Description |
|-----------|------------|-------------|
| `Dialog` | `Dialog::new(id)` | Root overlay with backdrop. Set `.open(bool)` and `.on_close(f)`. |
| `DialogTrigger` | `DialogTrigger::new(id)` | Wrapper that opens the dialog on click |
| `DialogContent` | `DialogContent::new()` | Centered content panel (max-width: 512px) |
| `DialogHeader` | `DialogHeader::new()` | Header section with vertical spacing |
| `DialogTitle` | `DialogTitle::new(text)` | Semibold title text |
| `DialogDescription` | `DialogDescription::new(text)` | Muted description text |
| `DialogFooter` | `DialogFooter::new()` | Right-aligned row for action buttons |
| `DialogClose` | `DialogClose::new()` | Wrapper that closes the dialog on click |
