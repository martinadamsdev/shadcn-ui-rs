# Phase 2 Implementation Plan (v0.2.0)

Overlay and feedback components. 10 components + shared infrastructure.

## Dependency Graph

```
[Infrastructure]
  overlay.rs ─────┬──> Tooltip
                  ├──> Popover ──> HoverCard
                  ├──> DropdownMenu
                  ├──> Sheet ──> Drawer
                  └──> Toast ──> Sonner

[Independent]
  Alert (pure layout, no dependencies)
  AlertDialog (reuses Dialog pattern, depends on dialog.rs)
```

## Implementation Order

Infrastructure must be built first. Then components can be built in parallel.

### Step 1: Shared Infrastructure

Three utility files in `components/`:

#### 1.1 `components/overlay.rs` -- Positioned overlay rendering

Shared utility for rendering deferred content positioned relative to a trigger element.

```rust
pub enum OverlaySide { Top, Right, Bottom, Left }
pub enum OverlayAlign { Start, Center, End }
```

Core pattern:
- Use `deferred()` with `.with_priority(200)` for overlay layer
- Use `absolute()` positioning with calculated offsets
- Accept `side` and `align` to determine placement

Used by: Tooltip, Popover, HoverCard, DropdownMenu, Toast

#### 1.2 `components/dismissable.rs` -- Click-outside and Escape dismiss

Shared behavior pattern (not a standalone component, but a documented pattern):
- `.on_mouse_down_out()` to close on click outside
- `.on_key_down()` with `event.keystroke.key == "escape"` to close on Escape

Used by: Popover, HoverCard, DropdownMenu, Sheet, Drawer

#### 1.3 Animation strategy

GPUI's `with_animation()` API is available but complex for layout animations.
Phase 2 approach: use instant show/hide with `.when()` conditional rendering.
Animation support will be revisited when GPUI's animation API stabilizes.

### Step 2: Independent Components (parallel)

#### 2.1 `components/alert.rs` -- Alert

Pure layout component, no interactivity. Simplest Phase 2 component.

```rust
pub enum AlertVariant { Default, Destructive }

pub struct Alert {
    variant: AlertVariant,
    children: Vec<AnyElement>,
}

pub struct AlertTitle { text: SharedString }
pub struct AlertDescription { text: SharedString }
```

Pattern: Card-like container (Pattern C). No event handlers needed.

Styling:
- Default: `border` + `background` colors
- Destructive: `destructive` color for border and text
- Icon slot (left side), title + description (right side)
- Rounded corners using theme radius

Tests:
- `test_alert_defaults` -- variant is Default
- `test_alert_destructive` -- variant is Destructive
- `test_alert_title` -- text stored correctly
- `test_alert_description` -- text stored correctly

#### 2.2 `components/alert_dialog.rs` -- AlertDialog

Modal confirmation dialog. Reuses Dialog's `deferred()` + backdrop pattern.

```rust
pub struct AlertDialog {
    id: ElementId,
    open: bool,
    on_close: Option<Box<dyn Fn(&mut Window, &mut App)>>,
    children: Vec<AnyElement>,
}

pub struct AlertDialogContent { children: Vec<AnyElement> }
pub struct AlertDialogHeader { children: Vec<AnyElement> }
pub struct AlertDialogTitle { text: SharedString }
pub struct AlertDialogDescription { text: SharedString }
pub struct AlertDialogFooter { children: Vec<AnyElement> }
pub struct AlertDialogAction {
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    children: Vec<AnyElement>,
}
pub struct AlertDialogCancel {
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    children: Vec<AnyElement>,
}
```

Pattern: Same as Dialog (Pattern D). Key difference:
- AlertDialogAction styled as primary button
- AlertDialogCancel styled as outline button
- No backdrop click dismiss (intentional -- user must choose action or cancel)

Tests:
- `test_alert_dialog_defaults` -- open false, no callbacks
- `test_alert_dialog_open` -- open state
- `test_alert_dialog_title` -- text
- `test_alert_dialog_description` -- text

### Step 3: Overlay Components (parallel, after Step 1)

#### 3.1 `components/tooltip.rs` -- Tooltip

Hover-triggered text overlay.

```rust
pub struct Tooltip {
    id: ElementId,
    text: SharedString,
    side: OverlaySide,       // default: Top
    open: bool,              // controlled state
    children: Vec<AnyElement>, // trigger element
}
```

Pattern:
- Wrap trigger children in a container with `.on_hover()`
- When open, render `deferred()` overlay with text content
- Position above/below/left/right of trigger using absolute positioning
- Simple text content only (unlike Popover/HoverCard)

Rendering approach:
- Outer div is relative positioned, contains the trigger
- When open, add a deferred child with absolute positioning
- Side determines offset: Top = `bottom(px(full_height + gap))`, Bottom = `top(px(trigger_height + gap))`

Tests:
- `test_tooltip_defaults` -- side is Top, not open
- `test_tooltip_builder` -- text, side, open state
- `test_tooltip_side` -- each side variant

#### 3.2 `components/popover.rs` -- Popover

Click-triggered overlay with arbitrary content.

```rust
pub enum PopoverSide { Top, Right, Bottom, Left }
pub enum PopoverAlign { Start, Center, End }

pub struct Popover {
    id: ElementId,
    open: bool,
    side: PopoverSide,       // default: Bottom
    align: PopoverAlign,     // default: Center
    on_open_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
    trigger: Vec<AnyElement>,
    content: Vec<AnyElement>,
}
```

Pattern:
- Click trigger to toggle open state via `on_open_change`
- Deferred overlay with content children
- Dismiss with `on_mouse_down_out()` and Escape key
- Position relative to trigger based on `side` and `align`

API design:
```rust
Popover::new("my-popover")
    .open(is_open)
    .side(PopoverSide::Bottom)
    .on_open_change(|open, window, cx| { /* toggle state */ })
    .trigger(Button::new("Click me"))
    .content(div().child("Popover content"))
```

Use separate `.trigger()` and `.content()` methods instead of `.child()` to distinguish the two slots.

Tests:
- `test_popover_defaults` -- side Bottom, align Center, not open
- `test_popover_builder` -- all props
- `test_popover_side_variants` -- all 4 sides

#### 3.3 `components/hover_card.rs` -- HoverCard

Hover-triggered card overlay with rich content. Same as Tooltip but accepts arbitrary children instead of text-only.

```rust
pub struct HoverCard {
    id: ElementId,
    open: bool,
    side: PopoverSide,       // reuse Popover's side enum or define own
    trigger: Vec<AnyElement>,
    content: Vec<AnyElement>,
}
```

Pattern: Combines Tooltip's hover behavior with Popover's content support.
- Hover trigger to show
- Rich content in overlay (not just text)
- Card-like styling (border, shadow, popover colors)

Tests:
- `test_hover_card_defaults` -- not open, side Bottom
- `test_hover_card_builder` -- open state, side

#### 3.4 `components/dropdown_menu.rs` -- DropdownMenu

Click-triggered menu with items, groups, separators, and keyboard navigation.

```rust
pub struct DropdownMenuItem {
    label: SharedString,
    value: SharedString,
    disabled: bool,
    destructive: bool,
}

pub struct DropdownMenuSeparator;

pub enum DropdownMenuEntry {
    Item(DropdownMenuItem),
    Separator,
    Label(SharedString),
}

pub struct DropdownMenu {
    id: ElementId,
    open: bool,
    on_open_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
    on_select: Option<Rc<dyn Fn(&str, &mut Window, &mut App)>>,
    entries: Vec<DropdownMenuEntry>,
    trigger: Vec<AnyElement>,
}
```

Pattern: Extends Select's dropdown pattern (Pattern E).
- Click trigger to open
- Deferred dropdown with items list
- `.on_mouse_down_out()` to dismiss
- Item hover highlight with accent colors
- Separator: thin horizontal line with border color
- Label: non-interactive text header for groups
- Destructive items: red text using `destructive` color

Tests:
- `test_dropdown_menu_defaults` -- not open, empty entries
- `test_dropdown_menu_item` -- value, label, disabled, destructive
- `test_dropdown_menu_entries` -- add items, separators, labels
- `test_dropdown_menu_builder` -- full builder chain

### Step 4: Panel Components (parallel, after Step 1)

#### 4.1 `components/sheet.rs` -- Sheet

Slide-in overlay panel from screen edge.

```rust
pub enum SheetSide { Top, Right, Bottom, Left }

pub struct Sheet {
    id: ElementId,
    open: bool,
    side: SheetSide,         // default: Right
    on_close: Option<Box<dyn Fn(&mut Window, &mut App)>>,
    children: Vec<AnyElement>,
}

pub struct SheetContent { children: Vec<AnyElement> }
pub struct SheetHeader { children: Vec<AnyElement> }
pub struct SheetTitle { text: SharedString }
pub struct SheetDescription { text: SharedString }
pub struct SheetFooter { children: Vec<AnyElement> }
```

Pattern: Similar to Dialog (Pattern D) with edge positioning.
- `deferred()` with backdrop
- Content positioned at edge based on `side`:
  - Right: `right_0().top_0().h_full().w(px(350.0))`
  - Left: `left_0().top_0().h_full().w(px(350.0))`
  - Top: `top_0().left_0().w_full().h(px(300.0))`
  - Bottom: `bottom_0().left_0().w_full().h(px(300.0))`
- Dismiss with backdrop click and Escape
- Sub-components mirror Dialog's structure (Header, Title, Description, Footer)

Tests:
- `test_sheet_defaults` -- side Right, not open
- `test_sheet_side` -- all 4 sides
- `test_sheet_title` -- text
- `test_sheet_description` -- text

#### 4.2 `components/drawer.rs` -- Drawer

Bottom sheet variant. Thin wrapper around Sheet pattern with `side: Bottom` default.

```rust
pub struct Drawer {
    id: ElementId,
    open: bool,
    on_close: Option<Box<dyn Fn(&mut Window, &mut App)>>,
    children: Vec<AnyElement>,
}

pub struct DrawerContent { children: Vec<AnyElement> }
pub struct DrawerHeader { children: Vec<AnyElement> }
pub struct DrawerTitle { text: SharedString }
pub struct DrawerDescription { text: SharedString }
pub struct DrawerFooter { children: Vec<AnyElement> }
```

Pattern: Same as Sheet but always bottom. Adds a drag handle indicator bar at top.
- Fixed bottom position: `bottom_0().left_0().w_full()`
- Drag handle: small rounded bar centered at top of content
- Max height constraint: `.max_h(px(500.0))`

Tests:
- `test_drawer_defaults` -- not open
- `test_drawer_title` -- text
- `test_drawer_description` -- text

### Step 5: Notification Components (parallel, after Step 1)

#### 5.1 `components/toast.rs` -- Toast

Temporary notification with auto-dismiss.

```rust
pub enum ToastVariant { Default, Destructive }

pub struct Toast {
    id: ElementId,
    title: SharedString,
    description: Option<SharedString>,
    variant: ToastVariant,
    open: bool,
    on_close: Option<Box<dyn Fn(&mut Window, &mut App)>>,
    action: Option<AnyElement>,
}
```

Pattern: Deferred overlay at screen corner.
- `deferred()` with fixed position at bottom-right
- Card-like styling with border, shadow
- Title + optional description
- Close button (X) in top-right corner
- Optional action button
- Auto-dismiss is caller's responsibility (use `cx.spawn()` externally)
- Variant controls colors: Default uses card colors, Destructive uses destructive colors

Tests:
- `test_toast_defaults` -- variant Default, not open, no description
- `test_toast_builder` -- title, description, variant, open
- `test_toast_destructive` -- variant is Destructive

#### 5.2 `components/sonner.rs` -- Sonner

Stacked toast notification manager.

```rust
pub struct SonnerToast {
    id: SharedString,
    title: SharedString,
    description: Option<SharedString>,
    variant: ToastVariant,   // reuse from toast.rs
}

pub struct Sonner {
    id: ElementId,
    toasts: Vec<SonnerToast>,
    position: SonnerPosition,
    on_dismiss: Option<Rc<dyn Fn(&str, &mut Window, &mut App)>>,
}

pub enum SonnerPosition {
    TopLeft, TopCenter, TopRight,
    BottomLeft, BottomCenter, BottomRight,
}
```

Pattern: Container that renders multiple toasts stacked.
- Deferred overlay at chosen screen corner
- Stack toasts vertically with gap
- Each toast has a close button calling `on_dismiss` with toast id
- Position controls fixed placement in viewport

Tests:
- `test_sonner_defaults` -- empty toasts, position BottomRight
- `test_sonner_toast` -- title, description
- `test_sonner_position` -- all 6 positions
- `test_sonner_add_toasts` -- multiple toasts stored

### Step 6: Integration

After all component files are created:

#### 6.1 Update `components/mod.rs`

Add module declarations and re-exports for all 10 new components.

#### 6.2 Update `crates/cli/src/component_sources.rs`

Add `include_str!()` entries for all 10 components.

#### 6.3 Update `crates/registry/src/lib.rs`

Add `ComponentMeta` entries for all 10 components with:
- Correct categories (Feedback for Alert/AlertDialog/Toast/Sonner, Display for Tooltip/Popover/HoverCard/DropdownMenu, Layout for Sheet/Drawer)
- Dependencies (alert_dialog depends on dialog, drawer depends on sheet, sonner depends on toast)
- Version "0.2.0"

Update registry version to "0.2.0".

#### 6.4 Verification

- `cargo build --workspace` -- all crates compile
- `cargo test --workspace` -- all tests pass
- `cargo clippy --workspace -- -D warnings` -- no warnings
- `cargo fmt --all -- --check` -- formatting clean

## Agent Team Assignment

| Agent | Components | Files |
|-------|-----------|-------|
| alert-builder | Alert, AlertDialog | alert.rs, alert_dialog.rs |
| overlay-builder | Tooltip, Popover, HoverCard | tooltip.rs, popover.rs, hover_card.rs |
| menu-builder | DropdownMenu | dropdown_menu.rs |
| panel-builder | Sheet, Drawer | sheet.rs, drawer.rs |
| notify-builder | Toast, Sonner | toast.rs, sonner.rs |

All agents run in parallel. After completion, team lead updates integration files and runs verification.

## File Checklist

New files to create:
- [ ] `components/alert.rs`
- [ ] `components/alert_dialog.rs`
- [ ] `components/tooltip.rs`
- [ ] `components/popover.rs`
- [ ] `components/hover_card.rs`
- [ ] `components/dropdown_menu.rs`
- [ ] `components/sheet.rs`
- [ ] `components/drawer.rs`
- [ ] `components/toast.rs`
- [ ] `components/sonner.rs`

Files to update:
- [ ] `components/mod.rs` -- add 10 module declarations + re-exports
- [ ] `crates/cli/src/component_sources.rs` -- add 10 include_str entries
- [ ] `crates/registry/src/lib.rs` -- add 10 ComponentMeta entries, bump version
