# Roadmap

This document outlines the planned development phases for shadcn-ui-rs.

## GPUI Capability Baseline

Analysis of gpui 0.2.2 confirms the following APIs are available and will be used throughout:

| Capability | GPUI API | Used By |
|------------|----------|---------|
| Layered rendering | `deferred()` + `.with_priority()` | Dialog, Select (Phase 1) |
| Click outside dismiss | `.on_mouse_down_out()` | Select (Phase 1) |
| Right-click events | `MouseButton::Right`, `ClickEvent::is_right_click()` | ContextMenu (Phase 4) |
| Mouse position tracking | `MouseMoveEvent.position`, `MouseDownEvent.position` | Resizable, Slider, Carousel (Phase 5) |
| Text input / IME | `InputHandler` trait, `EntityInputHandler` | Input, Textarea, Combobox (Phase 3) |
| Animation | `Animation`, `with_animation()`, easing functions | Sheet, Drawer, Toast (Phase 2) |
| Custom painting | `canvas()`, `PathBuilder`, `window.paint_path()` | Chart (Phase 6) |
| Focus management | `FocusHandle`, `.track_focus()`, `window.focus_next()` | Tabs, Command, InputOTP (Phase 4-5) |
| Scroll areas | `.overflow_y_scroll()` | ScrollArea, DataTable (Phase 3, 6) |
| Keyboard events | `.on_key_down()` | Calendar, Command (Phase 5) |

---

## Phase 1 -- v0.1.0 (Current) ✅

Core foundation: CLI tooling, theme system, and 12 essential components.

### Completed

- [x] CLI: init, add, remove, list, diff, update, theme commands
- [x] Theme system: 5 presets (zinc, slate, stone, gray, neutral) with light/dark mode
- [x] Components: Button, Input (display-only), Label, Checkbox, Radio, Switch, Slider (display-only), Select, Toggle, ToggleGroup, Card, Dialog
- [x] Example project: basic-form with all 12 components
- [x] Documentation: getting-started, components, theming guides
- [x] LICENSE, CHANGELOG, CONTRIBUTING

---

## Phase 2 -- v0.2.0

Overlay and feedback components. Built on `deferred()` layered rendering and `with_animation()` for transitions.

### Components (10)

- [ ] **Alert** -- Static alert box with icon, title, and description
  - Variants: default, destructive
  - Pure layout component, no interactivity
- [ ] **AlertDialog** -- Modal confirmation dialog with action/cancel buttons
  - Reuse Dialog backdrop and deferred rendering
  - Add `on_confirm` / `on_cancel` callbacks
- [ ] **Tooltip** -- Hover-triggered overlay with text content
  - Use `deferred()` for rendering above content
  - Position relative to trigger element using `Bounds` from layout
  - Show/hide on hover with optional delay
- [ ] **Popover** -- Click-triggered overlay with arbitrary content
  - Same positioning strategy as Tooltip
  - Dismiss with `on_mouse_down_out()`
  - Support `side` and `align` options
- [ ] **HoverCard** -- Hover-triggered card overlay with rich content
  - Same as Tooltip but accepts child elements
- [ ] **DropdownMenu** -- Click-triggered menu with items, separators, and sub-menus
  - Extend Select's dropdown pattern
  - Add support for groups, labels, separators, keyboard navigation
- [ ] **Sheet** -- Slide-in overlay panel from screen edge
  - Use `deferred()` + absolute positioning
  - Animate slide-in/out with `with_animation()` and `ease_in_out`
  - Support `side`: top, right, bottom, left
- [ ] **Drawer** -- Bottom sheet variant of Sheet
  - Reuse Sheet implementation with `side: bottom` default
- [ ] **Toast** -- Temporary notification with auto-dismiss
  - Use `deferred()` for rendering at screen corner
  - Timer-based auto-dismiss via `cx.spawn()` with delay
  - Support action button and close button
- [ ] **Sonner** -- Stacked toast notification system
  - Toast manager for queuing/stacking multiple toasts
  - Track toast list state, animate stack positioning

### Infrastructure

- [ ] Shared `Overlay` utility for deferred + positioned rendering
- [ ] Shared `Dismissable` behavior (click outside, escape key)
- [ ] Animation helpers: slide-in, fade-in, scale-in
- [ ] Add all 10 components to CLI registry (`component_sources.rs`)
- [ ] Unit tests for each component builder
- [ ] Update example project to demonstrate overlay components

---

## Phase 3 -- v0.3.0

Visual display components and editable text input.

### Components (12)

- [ ] **Badge** -- Inline status label with variants
  - Variants: default, secondary, outline, destructive
  - Pure styled container, similar to Button without interactivity
- [ ] **Avatar** -- User avatar with image or fallback initials
  - Circular container with `rounded_full()`
  - Fallback to initials text when no image available
  - Size presets: sm, default, lg
- [ ] **Separator** -- Horizontal or vertical dividing line
  - Single div with border, support `orientation` prop
- [ ] **Skeleton** -- Loading placeholder with pulse animation
  - Use `with_animation()` with `pulsating_between()` for opacity pulse
  - Support arbitrary size via width/height props
- [ ] **Progress** -- Horizontal progress bar
  - Two nested divs: track + filled portion
  - `value` prop (0.0-100.0) controls fill width
- [ ] **Spinner** -- Loading spinner with rotation animation
  - Use `with_animation()` with repeating rotation
  - Custom paint via `canvas()` + `PathBuilder` for arc drawing
- [ ] **Kbd** -- Keyboard shortcut display label
  - Styled inline container with monospace font and border
- [ ] **Typography** -- Text styling presets (h1-h4, p, blockquote, code, etc.)
  - Collection of styled text elements matching shadcn typography
- [ ] **Table** -- Data table with header, body, rows, and cells
  - Flex-based layout: TableHeader, TableBody, TableRow, TableCell
  - Support column alignment and fixed header
- [ ] **ScrollArea** -- Scrollable container with styled scrollbar
  - Wrap GPUI's `.overflow_y_scroll()` with themed styling
  - Optional horizontal scroll
- [ ] **Textarea** -- Multi-line text display (display-only, upgraded in Phase 5)
  - Multi-line variant of Input with min/max rows
- [ ] **Empty** -- Empty state placeholder with icon, title, and action
  - Centered layout container with optional icon and action button

### Infrastructure

- [ ] Add all 12 components to CLI registry
- [ ] Unit tests for each component
- [ ] Update docs/components.md with new component API references

---

## Phase 4 -- v0.4.0

Navigation and structural components. Click-driven panel switching, focus management with `FocusHandle`, and right-click context menus.

### Components (12)

- [ ] **Tabs** -- Tabbed content panels
  - TabsList + TabsTrigger + TabsContent pattern
  - Track active tab state, show/hide panels
  - Keyboard navigation: arrow keys between tabs via `on_key_down()`
  - Focus management with `FocusHandle` and `.track_focus()`
- [ ] **Accordion** -- Expandable/collapsible content sections
  - Single or multiple open sections
  - Animate expand/collapse height with `with_animation()`
  - AccordionItem + AccordionTrigger + AccordionContent pattern
- [ ] **Collapsible** -- Single expandable section
  - Simplified Accordion for a single section
  - `open` state prop with `on_open_change` callback
- [ ] **Breadcrumb** -- Navigation path indicator
  - BreadcrumbList + BreadcrumbItem + BreadcrumbSeparator
  - Each item clickable with `on_click` callback
- [ ] **Pagination** -- Page navigation controls
  - Previous/Next buttons + page number buttons
  - `current_page`, `total_pages` props
  - `on_page_change` callback
- [ ] **ButtonGroup** -- Grouped buttons with connected borders
  - Container that merges adjacent Button borders
  - Support horizontal and vertical orientation
- [ ] **Field** -- Form field wrapper with label, input, description, and error
  - Composite layout: Label + slot + description/error text
  - Error state styling
- [ ] **ContextMenu** -- Right-click triggered menu
  - Use `on_mouse_down(MouseButton::Right, ...)` to capture position
  - Use `MouseDownEvent.position` for menu placement
  - Reuse DropdownMenu rendering for menu items
  - Dismiss with `on_mouse_down_out()` and Escape key
- [ ] **Menubar** -- Application menu bar with dropdown menus
  - Horizontal bar with MenubarMenu + MenubarTrigger + MenubarContent
  - Hover-to-switch between open menus when one is already open
  - Keyboard navigation: arrow keys between menus, up/down within menus
  - Focus trapping with `FocusHandle`
- [ ] **NavigationMenu** -- Multi-level navigation with dropdown panels
  - Similar to Menubar but with wider content panels
  - Support links, groups, and descriptions within panels
- [ ] **Sidebar** -- Collapsible side navigation panel
  - Fixed-width panel with collapse toggle
  - Animate width change with `with_animation()`
  - Support SidebarHeader, SidebarContent, SidebarFooter sections
- [ ] **Item** -- Generic list item with icon, label, and action
  - Reusable row component for menus, lists, sidebars

### Infrastructure

- [ ] Shared `Menu` utility for DropdownMenu, ContextMenu, Menubar
- [ ] Shared `FocusGroup` helper for keyboard navigation patterns
- [ ] Focus trap utility: keep focus within a container (for dialogs, menus)
- [ ] Unit tests for all 12 components
- [ ] Update CLI registry with all new components

---

## Phase 5 -- v0.5.0

Editable text input and advanced interactive components. Built on `InputHandler` for text editing, `MouseMoveEvent` for drag tracking, and `FocusHandle` for multi-field focus control.

### Components (10)

- [ ] **Editable Input** -- Upgrade Phase 1 Input to support real text editing
  - Implement `EntityInputHandler` trait for the Input view
  - Handle `replace_text_in_range()` for character input
  - Handle `selected_text_range()` for cursor position
  - Support IME composition via `marked_text_range()` / `replace_and_mark_text_in_range()`
  - Call `window.handle_input()` during paint to activate platform text input
  - Cursor rendering with blink animation via `with_animation()`
  - Text selection rendering
- [ ] **Editable Textarea** -- Upgrade Phase 3 Textarea with text editing
  - Multi-line version of editable Input
  - Line wrapping and scroll when content exceeds visible area
- [ ] **Interactive Slider** -- Upgrade Phase 1 Slider with drag interaction
  - Track mouse position via `MouseMoveEvent.position` during drag
  - Calculate value from element-relative position using `Bounds`
  - Use `on_mouse_down()` to start drag, `on_mouse_up()` to end
- [ ] **Combobox** -- Searchable select dropdown
  - Combine editable Input + Select dropdown
  - Filter items in real-time as user types
  - Keyboard navigation through filtered results
- [ ] **Command** -- Command palette with fuzzy search
  - Full-screen centered overlay (like Dialog)
  - Editable Input at top for search query
  - Fuzzy search against command list using string matching
  - Keyboard up/down to navigate, Enter to select, Escape to close
  - CommandInput + CommandList + CommandGroup + CommandItem pattern
- [ ] **Calendar** -- Month calendar with date selection
  - 7-column grid layout for day cells
  - Month/year navigation with previous/next buttons
  - `on_select` callback with selected date
  - Keyboard navigation: arrow keys across dates via `on_key_down()`
  - Focus management with `FocusHandle` for active date cell
- [ ] **DatePicker** -- Date input with calendar dropdown
  - Combine editable Input + Popover + Calendar
  - Display selected date in input, open calendar on click/focus
  - Parse typed date strings
- [ ] **InputOTP** -- One-time password input with individual character slots
  - Multiple single-character input slots
  - Implement `InputHandler` per slot
  - Auto-advance focus to next slot via `FocusHandle.focus()`
  - Paste support: distribute pasted characters across slots
- [ ] **Carousel** -- Slideable content container
  - Horizontal scroll container with snapping
  - Previous/Next arrow button navigation
  - Animate slide transition with `with_animation()` and `ease_in_out`
  - Optional: drag scroll via `MouseMoveEvent` position tracking
- [ ] **Resizable** -- Resizable split panels
  - ResizablePanelGroup + ResizablePanel + ResizableHandle
  - Track handle drag via `MouseMoveEvent.position`
  - Calculate panel size deltas from mouse movement
  - Constrain with min/max panel sizes
  - Horizontal and vertical orientation support

### Infrastructure

- [ ] Shared `TextEditor` core for Input and Textarea text editing
- [ ] Shared `DragTracker` utility for mouse position tracking during drag
- [ ] Fuzzy search utility for Command and Combobox filtering
- [ ] Date utilities: month grid generation, date arithmetic
- [ ] Unit tests and integration tests for all interactive components
- [ ] Update example project with interactive demos

---

## Phase 6 -- v0.6.0

Data visualization and complex data components. Built on `canvas()` and `PathBuilder` for custom rendering.

### Components (3)

- [ ] **Chart** -- Data visualization charts
  - Use `canvas()` element for custom paint area
  - Use `PathBuilder` with `line_to()`, `curve_to()` for drawing lines, areas
  - Use `window.paint_path()` for rendering filled/stroked paths
  - Support chart types: Line, Bar, Area (pie requires arc calculations)
  - Axis labels and grid lines via positioned text elements
  - Tooltip on hover showing data point values
  - ChartContainer + ChartConfig pattern
- [ ] **DataTable** -- Full-featured data table
  - Build on Phase 3 Table component
  - Column sorting: click header to toggle asc/desc
  - Row selection: checkbox column with select all
  - Column resizing: drag column border using `MouseMoveEvent` tracking
  - Pagination: integrate with Phase 4 Pagination component
  - Virtualized scroll via ScrollArea for large datasets
- [ ] **NativeSelect** -- OS-native dropdown selector
  - Platform-specific native select widget integration
  - Fallback to Phase 1 Select if native API unavailable

### Infrastructure

- [ ] Chart rendering engine: axis calculation, scale mapping, data point layout
- [ ] Virtual scroll utility: render only visible rows for large datasets
- [ ] Unit tests for chart rendering and data table features

---

## Phase 7 -- v1.0.0

Polish, infrastructure, and stable release.

### Release Infrastructure

- [ ] **Registry server**: HTTP API for component distribution
  - REST endpoints: `/components`, `/components/:name`, `/themes`
  - Serve latest component source code
  - Version tracking per component
- [ ] **Documentation site**: Built with mdbook, hosted publicly
  - Component showcase with live preview screenshots
  - API reference generated from doc comments
  - Migration guide from shadcn/ui (React) patterns
- [ ] **crates.io publication**: Publish CLI and theme crates
  - `shadcn-ui-cli` binary crate
  - `shadcn-ui-theme` library crate
  - Versioned releases with changelogs
- [ ] **Homebrew formula**: `brew install shadcn-ui-rs`
- [ ] **CI/CD**: GitHub Actions
  - Test matrix: macOS (primary), Linux (future GPUI support)
  - Automated clippy, fmt, test on PR
  - Automated release builds and crate publishing

### Stability

- [ ] API review: audit all public types and methods for consistency
- [ ] Deprecation policy: document breaking change process
- [ ] Comprehensive test coverage: target 80%+ line coverage
- [ ] Performance benchmarks: render time for complex component trees
- [ ] Accessibility audit: keyboard navigation coverage across all components

---

## Components Not Planned

The following shadcn/ui components are not planned due to limited value in a desktop framework context.

| Component | Reason |
|-----------|--------|
| **AspectRatio** | GPUI's layout engine (Taffy) does not support CSS `aspect-ratio`. Manual calculation with fixed dimensions is possible but provides no benefit over setting width/height directly. |
| **Direction** | RTL text layout requires bidirectional text shaping at the rendering engine level. GPUI's text layout does not currently support this. Not a priority for most desktop applications. |

---

## Version Summary

| Version | Phase | Components | Total |
|---------|-------|------------|-------|
| v0.1.0 | Phase 1 ✅ | 12 core components | 12 |
| v0.2.0 | Phase 2 | 10 overlay/feedback | 22 |
| v0.3.0 | Phase 3 | 12 visual/display | 34 |
| v0.4.0 | Phase 4 | 12 navigation/structure | 46 |
| v0.5.0 | Phase 5 | 10 advanced interactive | 56 |
| v0.6.0 | Phase 6 | 3 data visualization | 59 |
| v1.0.0 | Phase 7 | Infrastructure + stability | 59 |
