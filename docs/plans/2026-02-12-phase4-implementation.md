# Phase 4 Implementation Plan (v0.4.0)

Navigation and structural components. 12 components.

## Dependency Graph

```
[Simple layout -- no dependencies, no interactivity]
  Breadcrumb
  ButtonGroup
  Field
  Item

[Toggle pattern -- click to show/hide content]
  Collapsible
  Accordion (extends Collapsible pattern)
  Tabs

[Menu pattern -- builds on Phase 2 DropdownMenu]
  ContextMenu (right-click trigger, reuses DropdownMenuEntry types)
  Menubar (horizontal bar of dropdown menus)
  NavigationMenu (wide panel dropdown navigation)

[Panel / Controls]
  Sidebar (collapsible side panel)
  Pagination (page navigation)
```

## Design Decisions

### Deferred to Phase 5

| Feature | Reason |
|---------|--------|
| Keyboard navigation (arrow keys for Tabs, Menubar) | Requires `FocusHandle` which needs View-level entity, not RenderOnce |
| Focus trapping for menus | Same FocusHandle limitation |
| Hover-to-switch for Menubar | Complex mouse tracking; click-to-switch is sufficient |
| Accordion expand/collapse animation | Consistent with Phase 2 no-animation strategy |
| Sidebar collapse animation | Same as above |
| Shared FocusGroup utility | Deferred with keyboard nav |

### Phase 4 Scope

All 12 components use click-based interaction only. No animations, no keyboard navigation, no focus management. These features will be added when Phase 5 introduces `FocusHandle` and animation infrastructure.

ContextMenu, Menubar, and NavigationMenu reuse `DropdownMenuEntry` and `DropdownMenuItem` from Phase 2's `dropdown_menu.rs` rather than duplicating types. Components import directly from `crate::dropdown_menu`.

## Implementation Order

### Step 1: Simple Layout Components (parallel)

#### 1.1 `components/breadcrumb.rs` -- Breadcrumb

Navigation path indicator. Pure layout, no state.

```rust
pub struct BreadcrumbItem {
    label: SharedString,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

pub struct BreadcrumbSeparator;

pub struct Breadcrumb {
    children: Vec<AnyElement>,
}
```

Pattern: Horizontal flex row. BreadcrumbItem is a clickable text link. BreadcrumbSeparator renders "/" between items.

Styling:
- Container: `flex().flex_row().items_center().gap(px(4.0))`
- Item: `text_sm()`, clickable items use `muted_foreground` color, last item uses `foreground` (current page, not clickable)
- Separator: `text_sm().text_color(muted_foreground)`, renders "/" character
- Clickable items: `cursor_pointer()` with hover underline

Tests:
- `test_breadcrumb_item` -- label stored correctly
- `test_breadcrumb_separator` -- renders
- `test_breadcrumb_builder` -- children added

#### 1.2 `components/button_group.rs` -- ButtonGroup

Grouped buttons with connected borders.

```rust
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

pub struct ButtonGroup {
    orientation: ButtonGroupOrientation,
    children: Vec<AnyElement>,
}
```

Pattern: Flex container that visually connects children. Uses border-radius removal on inner edges.

Styling:
- Container: `flex().gap(px(-1.0))` (negative gap overlaps borders)
- Horizontal: `flex_row()`
- Vertical: `flex_col()`
- First child: keeps left/top radius
- Last child: keeps right/bottom radius
- Middle children: no radius

Since GPUI does not support `nth-child` selectors, ButtonGroup simply renders a container with negative gap. Border overlap handles the visual connection. Children are rendered as-is; the parent provides the grouping context.

Tests:
- `test_button_group_defaults` -- horizontal orientation
- `test_button_group_vertical` -- vertical orientation
- `test_button_group_builder` -- children added

#### 1.3 `components/field.rs` -- Field

Form field wrapper with label, input slot, description, and error message.

```rust
pub struct Field {
    children: Vec<AnyElement>,  // input slot
    label: Option<SharedString>,
    description: Option<SharedString>,
    error: Option<SharedString>,
}
```

Pattern: Vertical flex column. Label on top, input slot, description below, error below that.

Styling:
- Container: `flex().flex_col().gap(px(8.0))`
- Label: `text_sm().font_weight(MEDIUM)` (reuses Label styling)
- Description: `text_sm().text_color(muted_foreground)`
- Error: `text_sm().text_color(destructive)`
- When error is present: input border turns `destructive` color (apply via wrapper class)

Tests:
- `test_field_defaults` -- no label, no description, no error
- `test_field_builder` -- label, description, error set
- `test_field_error` -- error text stored

#### 1.4 `components/item.rs` -- Item

Generic list item with icon, label, and trailing action.

```rust
pub struct Item {
    id: ElementId,
    label: SharedString,
    description: Option<SharedString>,
    icon: Option<SharedString>,      // text/emoji icon
    action: Option<AnyElement>,      // trailing element (e.g. badge, button)
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}
```

Pattern: Horizontal flex row used in menus, lists, sidebars.

Styling:
- Container: `flex().flex_row().items_center().gap(px(12.0)).px(px(12.0)).py(px(8.0))`
- Icon: `w(px(20.0)).text_center()`
- Label: `flex_1().text_sm()`
- Description: `text_xs().text_color(muted_foreground)` below label
- Action: rendered at trailing edge
- Hover: `bg(accent).text_color(accent_foreground)` when clickable
- Disabled: `opacity(0.5)`, no click handler

Tests:
- `test_item_defaults` -- label set, no icon, no action, not disabled
- `test_item_builder` -- all fields
- `test_item_disabled` -- disabled flag

### Step 2: Toggle Components (parallel)

#### 2.1 `components/collapsible.rs` -- Collapsible

Single expandable/collapsible section.

```rust
pub struct Collapsible {
    id: ElementId,
    open: bool,
    on_open_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
    trigger: Vec<AnyElement>,
    children: Vec<AnyElement>,  // content shown when open
}
```

Pattern: Controlled component. Click trigger to toggle `open` state. Content shown/hidden with `.when()`.

API:
```rust
Collapsible::new("my-section")
    .open(is_open)
    .on_open_change(|open, _window, _cx| { /* toggle state */ })
    .trigger(div().child("Click to expand"))
    .child("Hidden content revealed when open")
```

Rendering:
- Outer container with trigger always visible
- Content rendered with `.when(self.open, |el| el.children(self.children))`
- Trigger wraps its elements with an on_click that calls `on_open_change(!open)`

Tests:
- `test_collapsible_defaults` -- not open
- `test_collapsible_builder` -- open state, trigger set
- `test_collapsible_open` -- open flag

#### 2.2 `components/accordion.rs` -- Accordion

Multiple expandable/collapsible sections.

```rust
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AccordionType {
    #[default]
    Single,
    Multiple,
}

pub struct Accordion {
    id: ElementId,
    type_: AccordionType,
    value: Vec<SharedString>,  // currently open item values
    on_value_change: Option<Rc<dyn Fn(Vec<SharedString>, &mut Window, &mut App)>>,
    children: Vec<AnyElement>,
}

pub struct AccordionItem {
    value: SharedString,
    open: bool,
    on_toggle: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    children: Vec<AnyElement>,
}

pub struct AccordionTrigger {
    children: Vec<AnyElement>,
}

pub struct AccordionContent {
    children: Vec<AnyElement>,
}
```

Pattern: Container with multiple AccordionItem sections. Each item has a trigger and content. In Single mode, opening one closes others.

API:
```rust
Accordion::new("faq")
    .type_(AccordionType::Single)
    .value(vec!["item-1".into()])
    .on_value_change(|values, _window, _cx| { /* update state */ })
    .child(
        AccordionItem::new("item-1")
            .open(true)
            .child(AccordionTrigger::new().child("Section 1"))
            .child(AccordionContent::new().child("Content 1"))
    )
    .child(
        AccordionItem::new("item-2")
            .child(AccordionTrigger::new().child("Section 2"))
            .child(AccordionContent::new().child("Content 2"))
    )
```

Note: Since components are RenderOnce, the parent View manages which items are open. AccordionItem receives `open` as a prop and `on_toggle` for click handling. The Accordion container renders children as-is (it does not manage state internally).

Rendering:
- Accordion: vertical flex column with border-bottom on each item
- AccordionItem: wrapper with border-bottom, renders trigger and conditionally renders content
- AccordionTrigger: clickable row with text and chevron indicator (▼/▲)
- AccordionContent: `.when(open, |el| el.children(content))` to show/hide

Styling:
- Item border: `border_b_1().border_color(border)`
- Trigger: `flex().flex_row().items_center().justify_between().py(px(16.0)).cursor_pointer()`
- Trigger hover: `hover(|s| s.text_color(muted_foreground))` to indicate clickability
- Content: `pb(px(16.0))` padding when visible

Tests:
- `test_accordion_defaults` -- type Single, empty value
- `test_accordion_item` -- value stored, open flag
- `test_accordion_trigger` -- children added
- `test_accordion_content` -- children added
- `test_accordion_type` -- Single vs Multiple

#### 2.3 `components/tabs.rs` -- Tabs

Tabbed content panels.

```rust
pub struct Tabs {
    id: ElementId,
    value: SharedString,  // active tab value
    on_value_change: Option<Rc<dyn Fn(&str, &mut Window, &mut App)>>,
    children: Vec<AnyElement>,
}

pub struct TabsList {
    children: Vec<AnyElement>,
}

pub struct TabsTrigger {
    id: ElementId,
    value: SharedString,
    label: SharedString,
    active: bool,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

pub struct TabsContent {
    value: SharedString,
    active: bool,
    children: Vec<AnyElement>,
}
```

Pattern: Controlled component. Parent tracks active tab value. TabsTrigger buttons switch tabs via `on_value_change`. TabsContent is shown only when its `value` matches the active tab.

API:
```rust
Tabs::new("my-tabs")
    .value("tab1")
    .on_value_change(|value, _window, _cx| { /* set active tab */ })
    .child(
        TabsList::new()
            .child(TabsTrigger::new("tab1", "Account").active(true))
            .child(TabsTrigger::new("tab2", "Password"))
    )
    .child(TabsContent::new("tab1").active(true).child("Account settings"))
    .child(TabsContent::new("tab2").child("Password settings"))
```

Note: Since RenderOnce has no internal state, the parent View passes `active` to both TabsTrigger and TabsContent. This is the controlled component pattern used throughout the library.

Rendering:
- Tabs: simple container wrapping children
- TabsList: horizontal flex row with muted background, rounded container
- TabsTrigger: button within the list. Active: `bg(background).shadow_sm()`. Inactive: transparent
- TabsContent: `.when(self.active, |el| el.children(content))`

Styling:
- TabsList: `flex().flex_row().items_center().h(px(40.0)).rounded(px(radius)).bg(muted).p(px(4.0))`
- TabsTrigger active: `bg(background).text_color(foreground).shadow_sm().rounded(px(radius - 2.0))`
- TabsTrigger inactive: `text_color(muted_foreground)` with hover effect
- TabsTrigger: `px(px(12.0)).py(px(6.0)).text_sm().font_weight(MEDIUM).cursor_pointer()`
- TabsContent: `mt(px(8.0))`

Tests:
- `test_tabs_defaults` -- value stored
- `test_tabs_list` -- children added
- `test_tabs_trigger` -- value, label, active, disabled
- `test_tabs_content` -- value, active, children
- `test_tabs_trigger_active` -- active state

### Step 3: Menu Components (parallel, after Step 2 patterns established)

#### 3.1 `components/context_menu.rs` -- ContextMenu

Right-click triggered menu. Reuses `DropdownMenuEntry` and `DropdownMenuItem` types from `dropdown_menu.rs`.

```rust
pub struct ContextMenu {
    id: ElementId,
    open: bool,
    position: (f32, f32),  // x, y in pixels (window coordinates)
    on_open: Option<Rc<dyn Fn(f32, f32, &mut Window, &mut App)>>,  // called with (x, y) on right-click
    on_close: Option<Box<dyn Fn(&mut Window, &mut App)>>,
    on_select: Option<Rc<dyn Fn(&str, &mut Window, &mut App)>>,
    entries: Vec<DropdownMenuEntry>,  // reuse from dropdown_menu
    trigger: Vec<AnyElement>,
}
```

Pattern: Controlled component. Right-clicking the trigger calls `on_open(x, y)`. The parent stores the position and sets `open = true`. Menu renders at the stored position using `deferred()`.

API:
```rust
ContextMenu::new("ctx")
    .open(self.ctx_open)
    .position(self.ctx_x, self.ctx_y)
    .on_open(|x, y, _window, _cx| { /* store position, set open=true */ })
    .on_close(|_window, _cx| { /* set open=false */ })
    .on_select(|value, _window, _cx| { /* handle selection */ })
    .trigger(div().child("Right-click this area"))
    .item(DropdownMenuItem::new("cut", "Cut"))
    .item(DropdownMenuItem::new("copy", "Copy"))
    .separator()
    .item(DropdownMenuItem::new("paste", "Paste"))
```

Rendering:
- Wrap trigger in a container with `on_mouse_down(MouseButton::Right, ...)` to capture position
- When open, render `deferred()` menu at `position` using absolute positioning
- Menu has `on_mouse_down_out()` and Escape key to close
- Menu items reuse DropdownMenu's item rendering (hover highlight, destructive color, etc.)

Tests:
- `test_context_menu_defaults` -- not open, position (0,0)
- `test_context_menu_builder` -- all props set
- `test_context_menu_entries` -- items, separators, labels

#### 3.2 `components/menubar.rs` -- Menubar

Application menu bar with multiple dropdown menus.

```rust
pub struct Menubar {
    id: ElementId,
    children: Vec<AnyElement>,
}

pub struct MenubarMenu {
    id: ElementId,
    open: bool,
    on_open_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
    on_select: Option<Rc<dyn Fn(&str, &mut Window, &mut App)>>,
    trigger_label: SharedString,
    entries: Vec<DropdownMenuEntry>,  // reuse from dropdown_menu
}

pub struct MenubarSeparator;
```

Pattern: Horizontal bar containing MenubarMenu items. Each MenubarMenu has a trigger label and dropdown entries.

API:
```rust
Menubar::new("main-menu")
    .child(
        MenubarMenu::new("file")
            .trigger("File")
            .open(self.file_menu_open)
            .on_open_change(|open, _window, _cx| { /* toggle */ })
            .on_select(|value, _window, _cx| { /* handle */ })
            .item(DropdownMenuItem::new("new", "New File"))
            .item(DropdownMenuItem::new("open", "Open..."))
            .separator()
            .item(DropdownMenuItem::new("quit", "Quit"))
    )
    .child(
        MenubarMenu::new("edit")
            .trigger("Edit")
            .open(self.edit_menu_open)
            .on_open_change(|open, _window, _cx| { /* toggle */ })
            .item(DropdownMenuItem::new("undo", "Undo"))
            .item(DropdownMenuItem::new("redo", "Redo"))
    )
```

Rendering:
- Menubar: horizontal flex row with border-bottom, background
- MenubarMenu: trigger button + deferred dropdown (same as DropdownMenu pattern)
- MenubarSeparator: vertical line between menus
- Dropdown entries rendered identically to DropdownMenu

Styling:
- Menubar: `flex().flex_row().items_center().h(px(40.0)).border_b_1().border_color(border).bg(background).px(px(8.0))`
- Trigger: `px(px(12.0)).py(px(4.0)).text_sm().font_weight(MEDIUM).rounded(px(radius))`
- Trigger active: `bg(accent).text_color(accent_foreground)`
- Trigger hover: `hover(|s| s.bg(accent))`

Tests:
- `test_menubar_defaults` -- empty children
- `test_menubar_menu_defaults` -- not open, empty entries
- `test_menubar_menu_builder` -- trigger label, entries
- `test_menubar_menu_entries` -- items, separators

#### 3.3 `components/navigation_menu.rs` -- NavigationMenu

Multi-level navigation with wide dropdown panels.

```rust
pub struct NavigationMenu {
    id: ElementId,
    children: Vec<AnyElement>,
}

pub struct NavigationMenuItem {
    id: ElementId,
    open: bool,
    on_open_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
    trigger_label: SharedString,
    content: Vec<AnyElement>,  // wide panel content (arbitrary elements)
}

pub struct NavigationMenuLink {
    label: SharedString,
    description: Option<SharedString>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}
```

Pattern: Similar to Menubar but with wider content panels instead of narrow dropdown menus. Content is arbitrary (links, descriptions, groups).

API:
```rust
NavigationMenu::new("main-nav")
    .child(
        NavigationMenuItem::new("products")
            .trigger("Products")
            .open(self.products_open)
            .on_open_change(|open, _window, _cx| { /* toggle */ })
            .child(
                div().flex().gap(px(16.0))
                    .child(NavigationMenuLink::new("Widget A").description("Our flagship widget"))
                    .child(NavigationMenuLink::new("Widget B").description("Budget option"))
            )
    )
```

Rendering:
- NavigationMenu: horizontal flex row
- NavigationMenuItem: trigger button + deferred wide panel below
- NavigationMenuLink: clickable card with title + description
- Panel: wide card with padding, border, shadow (unlike DropdownMenu's narrow list)

Styling:
- Menu container: `flex().flex_row().items_center().gap(px(4.0))`
- Trigger: same as Menubar trigger styling
- Panel: `absolute().top(px(44.0)).left_0().min_w(px(400.0)).p(px(16.0)).rounded(px(radius)).border_1().border_color(border).bg(popover).shadow_lg()`
- NavigationMenuLink: `flex().flex_col().gap(px(2.0)).p(px(12.0)).rounded(px(radius)).hover(|s| s.bg(accent))`
- Link title: `text_sm().font_weight(MEDIUM)`
- Link description: `text_sm().text_color(muted_foreground)`

Tests:
- `test_navigation_menu_defaults` -- empty children
- `test_navigation_menu_item` -- trigger label, open state
- `test_navigation_menu_link` -- label, description
- `test_navigation_menu_link_builder` -- all props

### Step 4: Panel and Controls (parallel)

#### 4.1 `components/sidebar.rs` -- Sidebar

Collapsible side navigation panel.

```rust
pub struct Sidebar {
    id: ElementId,
    open: bool,
    side: SidebarSide,
    width: Pixels,
    children: Vec<AnyElement>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

pub struct SidebarHeader {
    children: Vec<AnyElement>,
}

pub struct SidebarContent {
    children: Vec<AnyElement>,
}

pub struct SidebarFooter {
    children: Vec<AnyElement>,
}

pub struct SidebarTrigger {
    id: ElementId,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    children: Vec<AnyElement>,
}
```

Pattern: Fixed-width panel with Header/Content/Footer sections. When `open` is false, renders with zero width (hidden). Parent layout uses flex row to place Sidebar beside main content.

API:
```rust
div().flex().flex_row().size_full()
    .child(
        Sidebar::new("sidebar")
            .open(self.sidebar_open)
            .side(SidebarSide::Left)
            .width(px(280.0))
            .child(SidebarHeader::new().child("Logo"))
            .child(
                SidebarContent::new()
                    .child(Item::new("item-1", "Dashboard"))
                    .child(Item::new("item-2", "Settings"))
            )
            .child(SidebarFooter::new().child("User"))
    )
    .child(
        div().flex_1().child("Main content")
    )
```

Rendering:
- When open: renders flex column with specified width, border on inner edge
- When closed: renders with `w(px(0.0)).overflow_hidden()` (zero width, hidden)
- SidebarHeader: top section with padding and bottom border
- SidebarContent: flex-1 with overflow_y_scroll for scrollable items
- SidebarFooter: bottom section with padding and top border
- SidebarTrigger: a button wrapper for toggling open/close

Styling:
- Container: `flex().flex_col().h_full().bg(background).border_color(border)`
- Left side: `border_r_1()`
- Right side: `border_l_1()`
- Header: `flex().items_center().h(px(56.0)).px(px(16.0)).border_b_1().border_color(border)`
- Content: `flex_1().flex().flex_col().py(px(8.0)).overflow_y_scroll()` (needs `.id()`)
- Footer: `flex().items_center().h(px(56.0)).px(px(16.0)).border_t_1().border_color(border)`

Tests:
- `test_sidebar_defaults` -- open true, side Left, width 280
- `test_sidebar_builder` -- all props
- `test_sidebar_side` -- Left vs Right
- `test_sidebar_trigger` -- click handler

#### 4.2 `components/pagination.rs` -- Pagination

Page navigation controls.

```rust
pub struct Pagination {
    id: ElementId,
    current_page: usize,
    total_pages: usize,
    on_page_change: Option<Box<dyn Fn(usize, &mut Window, &mut App)>>,
}
```

Pattern: Horizontal row with Previous button, page number buttons, and Next button. Highlights current page.

API:
```rust
Pagination::new("pages")
    .current_page(3)
    .total_pages(10)
    .on_page_change(|page, _window, _cx| { /* update current page */ })
```

Rendering:
- Horizontal flex row with gap
- Previous button: disabled when page == 1
- Page numbers: show a window of pages around current (e.g. 1 ... 3 4 5 ... 10)
- Next button: disabled when page == total_pages
- Current page: `bg(primary).text_color(primary_foreground)`
- Other pages: `bg(transparent)` with hover `bg(accent)`

Page window algorithm: show first page, last page, and up to 2 pages around current. Fill gaps with "..." ellipsis.

Styling:
- Container: `flex().flex_row().items_center().gap(px(4.0))`
- Page button: `w(px(36.0)).h(px(36.0)).flex().items_center().justify_center().rounded(px(radius)).text_sm().cursor_pointer()`
- Previous/Next: same size with "←" / "→" text
- Disabled: `opacity(0.5)`, no click handler
- Current: `bg(primary).text_color(primary_foreground)`
- Ellipsis: `w(px(36.0)).h(px(36.0)).flex().items_center().justify_center().text_color(muted_foreground)`

Tests:
- `test_pagination_defaults` -- page 1, total 1
- `test_pagination_builder` -- current_page, total_pages
- `test_pagination_page_window` -- page range calculation
- `test_pagination_edge_cases` -- page 1, last page

### Step 5: Integration

After all component files are created:

#### 5.1 Update `components/mod.rs`

Add module declarations and re-exports for all 12 new components.

#### 5.2 Update `crates/cli/src/component_sources.rs`

Add `include_str!()` entries for all 12 components.

#### 5.3 Update `crates/registry/src/lib.rs`

Add `ComponentMeta` entries for all 12 components with:
- Categories: Navigation (Breadcrumb, Tabs, Menubar, NavigationMenu, ContextMenu, Sidebar, Pagination), Input (Field), Layout (ButtonGroup, Accordion, Collapsible, Item)
- Dependencies: context_menu depends on dropdown_menu, menubar depends on dropdown_menu, accordion depends on collapsible conceptually but no file dependency
- Version "0.4.0"

Update registry version to "0.4.0".

#### 5.4 Update `docs/roadmap.md`

Mark Phase 4 as complete with deferred items table.

#### 5.5 Update `CHANGELOG.md`

Add v0.4.0 entry listing all 12 components.

#### 5.6 Verification

- `cargo build --workspace` -- all crates compile
- `cargo test --workspace` -- all tests pass
- `cargo clippy --workspace -- -D warnings` -- no warnings
- `cargo fmt --all -- --check` -- formatting clean

## Agent Team Assignment

| Agent | Components | Files |
|-------|-----------|-------|
| layout-builder | Breadcrumb, ButtonGroup, Field, Item | breadcrumb.rs, button_group.rs, field.rs, item.rs |
| toggle-builder | Collapsible, Accordion, Tabs | collapsible.rs, accordion.rs, tabs.rs |
| menu-builder | ContextMenu, Menubar, NavigationMenu | context_menu.rs, menubar.rs, navigation_menu.rs |
| panel-builder | Sidebar, Pagination | sidebar.rs, pagination.rs |

All agents run in parallel. After completion, team lead handles integration (Step 5) and runs verification.

## Commit Strategy (granular)

1. Each component gets its own commit: `feat: add <ComponentName> component`
2. Integration updates get separate commits:
   - `feat: register Phase 4 components in mod.rs`
   - `feat: embed Phase 4 component sources in CLI`
   - `feat: add Phase 4 components to registry`
3. Documentation updates:
   - `docs: update roadmap and changelog for Phase 4`

## File Checklist

New files to create:
- [x] `components/breadcrumb.rs`
- [x] `components/button_group.rs`
- [x] `components/field.rs`
- [x] `components/item.rs`
- [x] `components/collapsible.rs`
- [x] `components/accordion.rs`
- [x] `components/tabs.rs`
- [x] `components/context_menu.rs`
- [x] `components/menubar.rs`
- [x] `components/navigation_menu.rs`
- [x] `components/sidebar.rs`
- [x] `components/pagination.rs`

Files to update:
- [x] `components/mod.rs` -- add 12 module declarations + re-exports
- [x] `crates/cli/src/component_sources.rs` -- add 12 include_str entries
- [x] `crates/registry/src/lib.rs` -- add 12 ComponentMeta entries, bump version
- [x] `docs/roadmap.md` -- mark Phase 4 complete
- [x] `CHANGELOG.md` -- add v0.4.0 entry
