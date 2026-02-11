# Phase 3 Implementation Plan (v0.3.0)

Visual display components. 11 components (Spinner deferred to Phase 6).

## Design Decisions

- **Spinner deferred to Phase 6**: Spinner requires `canvas()` + `PathBuilder` for arc drawing. Phase 6 Chart also needs these APIs. Build both together to avoid introducing custom painting prematurely.
- **Typography as multiple structs**: H1, H2, H3, H4, Paragraph, Blockquote, InlineCode, Lead -- each a separate struct with `RenderOnce`, matching the CardTitle/CardDescription pattern.
- **Table is pure layout**: No column alignment, sorting, or selection. Just styled flex containers. Phase 6 DataTable adds interactivity.
- **Skeleton is static**: No `with_animation()` pulse. Static muted background block, consistent with Phase 2's no-animation strategy.

## Dependency Graph

```
[All Independent -- no shared infrastructure needed]

  Badge         (pure styled container)
  Avatar        (circular container + fallback)
  Separator     (single div with border)
  Skeleton      (static muted block)
  Progress      (two nested divs: track + fill)
  Kbd           (styled inline container)
  Typography    (collection of styled text structs)
  Table         (flex layout: Header, Body, Row, Cell, Caption)
  ScrollArea    (wraps overflow_y_scroll)
  Textarea      (multi-line Input variant)
  Empty         (centered layout with optional icon/action)
```

No shared infrastructure required. All 11 components are independent and can be built in parallel.

## Implementation Order

All components are independent, so the only ordering constraint is integration (Step 2) after all components (Step 1).

### Step 1: Components (all parallel)

#### 1.1 `components/badge.rs` -- Badge

Inline status label with variants. Pure styled container, similar to Button without interactivity.

```rust
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Destructive,
}

#[derive(IntoElement)]
pub struct Badge {
    label: SharedString,
    variant: BadgeVariant,
}
```

API:
```rust
Badge::new("New")
    .variant(BadgeVariant::Destructive)
```

Rendering:
- Inline flex container with horizontal padding, small text
- Default: `primary` bg + `primary_foreground` text
- Secondary: `secondary` bg + `secondary_foreground` text
- Outline: transparent bg + `foreground` text + `border` border
- Destructive: `destructive` bg + `destructive_foreground` text
- Rounded full (`rounded_full()`) for pill shape
- No click handler, no id needed

Tests:
- `test_badge_defaults` -- variant is Default, label stored
- `test_badge_variants` -- each variant
- `test_badge_builder` -- full builder chain

#### 1.2 `components/avatar.rs` -- Avatar

User avatar with fallback initials. Circular container.

```rust
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AvatarSize {
    Sm,    // 32px
    #[default]
    Default, // 40px
    Lg,    // 48px
}

#[derive(IntoElement)]
pub struct Avatar {
    fallback: SharedString,  // initials text (e.g. "MA")
    size: AvatarSize,
}
```

API:
```rust
Avatar::new("MA")
    .size(AvatarSize::Lg)
```

Rendering:
- Circular container: `rounded_full()` with `overflow_hidden()`
- Fixed width/height based on size
- `muted` background with `muted_foreground` text for fallback
- Center text with `flex().items_center().justify_center()`
- Text size scales with avatar size (xs for Sm, sm for Default, base for Lg)

Note: Image support not included. GPUI image loading requires `img()` element or `ImageSource` -- add in a future enhancement if needed. Fallback initials cover the common case.

Tests:
- `test_avatar_defaults` -- size Default, fallback stored
- `test_avatar_sizes` -- each size variant
- `test_avatar_builder` -- full chain

#### 1.3 `components/separator.rs` -- Separator

Horizontal or vertical dividing line.

```rust
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(IntoElement)]
pub struct Separator {
    orientation: SeparatorOrientation,
}
```

API:
```rust
Separator::new()
    .orientation(SeparatorOrientation::Vertical)
```

Rendering:
- Horizontal: `w_full().h(px(1.0)).bg(colors.border)`
- Vertical: `h_full().w(px(1.0)).bg(colors.border)`
- Shrink-0 to prevent flex compression

Tests:
- `test_separator_defaults` -- orientation Horizontal
- `test_separator_vertical` -- orientation Vertical

#### 1.4 `components/skeleton.rs` -- Skeleton

Static loading placeholder block.

```rust
#[derive(IntoElement)]
pub struct Skeleton {
    width: Option<Pixels>,
    height: Option<Pixels>,
    rounded: bool,  // true = rounded_full (for circular skeletons)
}
```

API:
```rust
Skeleton::new()
    .width(px(200.0))
    .height(px(20.0))

// Circular skeleton for avatar placeholder
Skeleton::new()
    .width(px(40.0))
    .height(px(40.0))
    .rounded(true)
```

Rendering:
- `muted` background color
- Default border radius from theme, or `rounded_full()` if `rounded` is true
- Width/height applied if provided, otherwise fills parent
- No animation (static block)

Tests:
- `test_skeleton_defaults` -- no width, no height, not rounded
- `test_skeleton_builder` -- width, height, rounded
- `test_skeleton_rounded` -- rounded flag

#### 1.5 `components/progress.rs` -- Progress

Horizontal progress bar with track and fill.

```rust
#[derive(IntoElement)]
pub struct Progress {
    value: f32,  // 0.0 to 100.0
}
```

API:
```rust
Progress::new(65.0)
```

Rendering:
- Outer track: `w_full().h(px(8.0)).rounded_full().bg(colors.secondary)` with `overflow_hidden()`
- Inner fill: `h_full().rounded_full().bg(colors.primary)`
- Fill width: percentage via `.w(relative(value / 100.0))` or manual `px()` calculation
- Value clamped to 0.0..=100.0

Tests:
- `test_progress_defaults` -- value 0.0
- `test_progress_value` -- value stored, clamped
- `test_progress_clamp` -- values outside 0-100 are clamped

#### 1.6 `components/kbd.rs` -- Kbd

Keyboard shortcut display label.

```rust
#[derive(IntoElement)]
pub struct Kbd {
    keys: SharedString,
}
```

API:
```rust
Kbd::new("⌘K")
```

Rendering:
- Inline container with `muted` background
- `border` border, rounded corners (theme radius)
- Small text, monospace-style (slightly smaller than body)
- Horizontal padding for visual spacing
- `muted_foreground` text color

Tests:
- `test_kbd_new` -- keys stored
- `test_kbd_builder` -- builder chain

#### 1.7 `components/typography.rs` -- Typography

Collection of styled text elements. Each is a separate struct.

```rust
#[derive(IntoElement)]
pub struct H1 { text: SharedString }

#[derive(IntoElement)]
pub struct H2 { text: SharedString }

#[derive(IntoElement)]
pub struct H3 { text: SharedString }

#[derive(IntoElement)]
pub struct H4 { text: SharedString }

#[derive(IntoElement)]
pub struct Paragraph {
    text: SharedString,
    children: Vec<AnyElement>,
}

#[derive(IntoElement)]
pub struct Blockquote {
    children: Vec<AnyElement>,
}

#[derive(IntoElement)]
pub struct InlineCode {
    text: SharedString,
}

#[derive(IntoElement)]
pub struct Lead {
    text: SharedString,
}
```

API:
```rust
H1::new("Page Title")
H2::new("Section Title")
Paragraph::new("Body text here")
Blockquote::new().child(Paragraph::new("Quoted text"))
InlineCode::new("let x = 1;")
Lead::new("A lead paragraph with larger, muted text")
```

Rendering:
- H1: `text_3xl()` + `EXTRA_BOLD` + tight line height
- H2: `text_2xl()` + `SEMIBOLD` + tracking tight (if available, else normal)
- H3: `text_xl()` + `SEMIBOLD`
- H4: `text_lg()` + `SEMIBOLD`
- Paragraph: `text_base()` + normal line height. Implements `ParentElement` for mixed content.
- Blockquote: left border (2px `border` color) + left padding + italic. Implements `ParentElement`.
- InlineCode: `muted` bg + rounded sm + `px(4.0)` padding + smaller text
- Lead: `text_xl()` + `muted_foreground` color
- All headings use `foreground` color from theme

Tests:
- `test_h1_new` through `test_h4_new` -- text stored
- `test_paragraph_new` -- text stored
- `test_blockquote_new` -- children supported
- `test_inline_code_new` -- text stored
- `test_lead_new` -- text stored

#### 1.8 `components/table.rs` -- Table

Pure layout data table with flex-based structure.

```rust
#[derive(IntoElement)]
pub struct Table { children: Vec<AnyElement> }

#[derive(IntoElement)]
pub struct TableHeader { children: Vec<AnyElement> }

#[derive(IntoElement)]
pub struct TableBody { children: Vec<AnyElement> }

#[derive(IntoElement)]
pub struct TableRow { children: Vec<AnyElement> }

#[derive(IntoElement)]
pub struct TableHead { text: SharedString }

#[derive(IntoElement)]
pub struct TableCell { children: Vec<AnyElement> }

#[derive(IntoElement)]
pub struct TableCaption { text: SharedString }
```

API:
```rust
Table::new()
    .child(TableHeader::new()
        .child(TableRow::new()
            .child(TableHead::new("Name"))
            .child(TableHead::new("Email"))))
    .child(TableBody::new()
        .child(TableRow::new()
            .child(TableCell::new().child("Alice"))
            .child(TableCell::new().child("alice@example.com"))))
    .child(TableCaption::new("A list of users"))
```

Rendering:
- Table: `w_full().flex().flex_col()` + `text_sm()` + caption at bottom
- TableHeader: flex column container
- TableBody: flex column container
- TableRow: `flex().flex_row().items_center()` + `border_b()` with `border` color + hover bg `muted` (with low opacity)
- TableHead: `flex_1()` + `h(px(48.0))` + `items_center()` + `muted_foreground` text + `MEDIUM` weight + left-aligned padding
- TableCell: `flex_1()` + `py(px(16.0))` + left-aligned padding
- TableCaption: `mt(px(16.0))` + `text_sm()` + `muted_foreground`

All container types (Table, TableHeader, TableBody, TableRow, TableCell) implement `ParentElement`.

Tests:
- `test_table_new` -- creates Table
- `test_table_head` -- text stored
- `test_table_caption` -- text stored
- `test_table_cell_new` -- creates TableCell

#### 1.9 `components/scroll_area.rs` -- ScrollArea

Scrollable container with themed styling.

```rust
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ScrollOrientation {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

#[derive(IntoElement)]
pub struct ScrollArea {
    orientation: ScrollOrientation,
    max_height: Option<Pixels>,
    children: Vec<AnyElement>,
}
```

API:
```rust
ScrollArea::new()
    .max_height(px(300.0))
    .child(long_content)

ScrollArea::new()
    .orientation(ScrollOrientation::Horizontal)
    .child(wide_content)
```

Rendering:
- Vertical: `.overflow_y_scroll()` + optional `.max_h()`
- Horizontal: `.overflow_x_scroll()` + optional `.max_w()` (if needed)
- Both: `.overflow_scroll()`
- `relative()` positioning for potential scrollbar overlay
- Implements `ParentElement`

Tests:
- `test_scroll_area_defaults` -- orientation Vertical, no max_height
- `test_scroll_area_builder` -- orientation, max_height
- `test_scroll_area_horizontal` -- orientation Horizontal

#### 1.10 `components/textarea.rs` -- Textarea

Multi-line text display. Display-only, same as Input (editable in Phase 5).

```rust
#[derive(IntoElement)]
pub struct Textarea {
    id: ElementId,
    placeholder: Option<SharedString>,
    value: Option<SharedString>,
    disabled: bool,
    min_rows: u32,  // default 3
}
```

API:
```rust
Textarea::new("description")
    .placeholder("Enter description...")
    .value("Some long text\nwith multiple lines")
    .min_rows(5)
```

Rendering:
- Same styling as Input (border, background, text colors, radius)
- Multi-line: use `min_h()` based on `min_rows * line_height` instead of fixed `h(px(36.0))`
- Placeholder/value logic identical to Input
- Line height ~20px, so min_rows=3 gives min_h(px(60.0)) + padding

Tests:
- `test_textarea_defaults` -- no placeholder, no value, min_rows 3
- `test_textarea_builder` -- all props
- `test_textarea_disabled` -- disabled state

#### 1.11 `components/empty.rs` -- Empty

Empty state placeholder with centered layout.

```rust
#[derive(IntoElement)]
pub struct Empty {
    title: SharedString,
    description: Option<SharedString>,
    action: Option<AnyElement>,
}
```

API:
```rust
Empty::new("No results")
    .description("Try adjusting your search query")
    .action(Button::new("Clear filters"))
```

Rendering:
- Centered container: `flex().flex_col().items_center().justify_center().gap(px(8.0))`
- Title: `text_lg()` + `SEMIBOLD` + `foreground` color
- Description: `text_sm()` + `muted_foreground` color
- Action: rendered below description with `mt(px(16.0))`
- Padding: `py(px(40.0))` for vertical spacing

Tests:
- `test_empty_defaults` -- title stored, no description, no action
- `test_empty_builder` -- title, description
- `test_empty_description` -- description stored

### Step 2: Integration

After all 11 component files are created:

#### 2.1 Update `components/mod.rs`

Add module declarations and re-exports for all 11 new components:

```rust
// Phase 3
pub mod avatar;
pub mod badge;
pub mod empty;
pub mod kbd;
pub mod progress;
pub mod scroll_area;
pub mod separator;
pub mod skeleton;
pub mod table;
pub mod textarea;
pub mod typography;

// Phase 3 re-exports
pub use avatar::{Avatar, AvatarSize};
pub use badge::{Badge, BadgeVariant};
pub use empty::Empty;
pub use kbd::Kbd;
pub use progress::Progress;
pub use scroll_area::{ScrollArea, ScrollOrientation};
pub use separator::{Separator, SeparatorOrientation};
pub use skeleton::Skeleton;
pub use table::{Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow};
pub use textarea::Textarea;
pub use typography::{Blockquote, H1, H2, H3, H4, InlineCode, Lead, Paragraph};
```

#### 2.2 Update `crates/cli/src/component_sources.rs`

Add `include_str!()` entries for all 11 components:

```rust
// Phase 3
"badge" => Some(include_str!("../../../components/badge.rs")),
"avatar" => Some(include_str!("../../../components/avatar.rs")),
"separator" => Some(include_str!("../../../components/separator.rs")),
"skeleton" => Some(include_str!("../../../components/skeleton.rs")),
"progress" => Some(include_str!("../../../components/progress.rs")),
"kbd" => Some(include_str!("../../../components/kbd.rs")),
"typography" => Some(include_str!("../../../components/typography.rs")),
"table" => Some(include_str!("../../../components/table.rs")),
"scroll_area" => Some(include_str!("../../../components/scroll_area.rs")),
"textarea" => Some(include_str!("../../../components/textarea.rs")),
"empty" => Some(include_str!("../../../components/empty.rs")),
```

#### 2.3 Update `crates/registry/src/lib.rs`

Add `ComponentMeta` entries for all 11 components. All are version "0.3.0" with no dependencies.

Categories:
- Display: Badge, Avatar, Skeleton, Progress, Typography, Empty
- Layout: Separator, Table, ScrollArea
- Input: Textarea, Kbd

Update registry version to "0.3.0".

#### 2.4 Update `docs/roadmap.md`

- Move Spinner from Phase 3 to Phase 6
- Update Phase 3 component count from 12 to 11
- Update version summary table

#### 2.5 Verification

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

## Agent Team Assignment

| Agent | Components | Files |
|-------|-----------|-------|
| display-builder | Badge, Avatar, Skeleton, Progress | badge.rs, avatar.rs, skeleton.rs, progress.rs |
| text-builder | Typography, Kbd, Empty | typography.rs, kbd.rs, empty.rs |
| layout-builder | Separator, Table, ScrollArea, Textarea | separator.rs, table.rs, scroll_area.rs, textarea.rs |

All agents run in parallel. After completion, team lead updates integration files and runs verification.

## File Checklist

New files to create:
- [ ] `components/badge.rs`
- [ ] `components/avatar.rs`
- [ ] `components/separator.rs`
- [ ] `components/skeleton.rs`
- [ ] `components/progress.rs`
- [ ] `components/kbd.rs`
- [ ] `components/typography.rs`
- [ ] `components/table.rs`
- [ ] `components/scroll_area.rs`
- [ ] `components/textarea.rs`
- [ ] `components/empty.rs`

Files to update:
- [ ] `components/mod.rs` -- add 11 module declarations + re-exports
- [ ] `crates/cli/src/component_sources.rs` -- add 11 include_str entries
- [ ] `crates/registry/src/lib.rs` -- add 11 ComponentMeta entries, bump version
- [ ] `docs/roadmap.md` -- move Spinner to Phase 6, update counts
