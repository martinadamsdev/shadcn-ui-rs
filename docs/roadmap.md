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
| Text input / IME | `InputHandler` trait, `EntityInputHandler` | Input, Textarea, Combobox (Phase 5) |
| Animation | `Animation`, `with_animation()`, easing functions | Sheet, Drawer, Toast (Phase 2) |
| Custom painting | `canvas()`, `PathBuilder`, `window.paint_path()` | Chart, Spinner (Phase 6) |
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

## Phase 2 -- v0.2.0 ✅

Overlay and feedback components. Built on `deferred()` layered rendering and `with_animation()` for transitions.

### Completed

- [x] **Alert** -- Static alert box with icon, title, and description (default, destructive variants)
- [x] **AlertDialog** -- Modal confirmation dialog with action/cancel buttons
- [x] **Tooltip** -- Hover-triggered overlay with text content (top, right, bottom, left sides)
- [x] **Popover** -- Click-triggered overlay with arbitrary content (side + align options)
- [x] **HoverCard** -- Hover-triggered card overlay with rich content
- [x] **DropdownMenu** -- Click-triggered menu with items, separators, and labels
- [x] **Sheet** -- Slide-in overlay panel from screen edge (top, right, bottom, left)
- [x] **Drawer** -- Bottom sheet variant with drag handle
- [x] **Toast** -- Temporary notification with auto-dismiss (default, success, error, warning variants)
- [x] **Sonner** -- Stacked toast notification system (top-left/right, bottom-left/right positions)
- [x] All 10 components added to CLI registry (`component_sources.rs`)
- [x] Unit tests for each component builder
- [x] Cross-platform CI: macOS, Linux, Windows

---

## Phase 3 -- v0.3.0 ✅

Visual display components. 11 components (Spinner deferred to Phase 6).

### Completed

- [x] **Badge** -- Inline status label with variants (default, secondary, outline, destructive)
- [x] **Avatar** -- User avatar with fallback initials (sm, default, lg sizes)
- [x] **Separator** -- Horizontal or vertical dividing line
- [x] **Skeleton** -- Static loading placeholder block
- [x] **Progress** -- Horizontal progress bar (0.0-100.0 value)
- [x] **Kbd** -- Keyboard shortcut display label
- [x] **Typography** -- Text styling presets (H1, H2, H3, H4, Paragraph, Blockquote, InlineCode, Lead)
- [x] **Table** -- Data table (Table, TableHeader, TableBody, TableRow, TableHead, TableCell, TableCaption)
- [x] **ScrollArea** -- Scrollable container with configurable orientation
- [x] **Textarea** -- Multi-line text display (display-only, upgraded in Phase 5)
- [x] **Empty** -- Empty state placeholder with title, description, and action
- [x] All 11 components added to CLI registry
- [x] Unit tests for each component
- [x] Example project updated to showcase all 33 Phase 1-3 components

### Deferred Items

| Item | Original Scope | Deferred To | Reason |
|------|---------------|-------------|--------|
| **Spinner** component | Phase 3 component | **Phase 6** (with Chart) | Requires `canvas()` + `PathBuilder` custom painting; Phase 6 Chart uses same APIs |
| **Skeleton pulse animation** | `with_animation()` opacity pulse | **Phase 5** (with animation infrastructure) | Consistent with Phase 2 no-animation strategy; static placeholder shipped instead |
| **Table column alignment** | Column align (left/center/right) | **Phase 6** (with DataTable) | Pure layout sufficient for Phase 3; alignment added when DataTable builds on Table |
| **Avatar image support** | Image loading via `img()` | **Future enhancement** | GPUI image loading needs investigation; fallback initials cover common case |

---

## Phase 4 -- v0.4.0 ✅

Navigation and structural components. Click-driven panel switching and right-click context menus.

### Completed

- [x] **Tabs** -- Tabbed content panels (TabsList + TabsTrigger + TabsContent)
- [x] **Accordion** -- Expandable/collapsible content sections (Single/Multiple mode)
- [x] **Collapsible** -- Single expandable section with open/close toggle
- [x] **Breadcrumb** -- Navigation path indicator with clickable items and separators
- [x] **Pagination** -- Page navigation with previous/next and page number buttons
- [x] **ButtonGroup** -- Grouped buttons with connected borders (horizontal/vertical)
- [x] **Field** -- Form field wrapper with label, input, description, and error
- [x] **ContextMenu** -- Right-click triggered menu using `MouseButton::Right` and position tracking
- [x] **Menubar** -- Application menu bar with dropdown menus (MenubarMenu + MenubarSeparator)
- [x] **NavigationMenu** -- Multi-level navigation with wide dropdown panels
- [x] **Sidebar** -- Collapsible side navigation panel (Header/Content/Footer sections)
- [x] **Item** -- Generic list item with icon, label, and trailing action
- [x] ContextMenu and Menubar reuse `DropdownMenuEntry`/`DropdownMenuItem` from Phase 2
- [x] All 12 components added to CLI registry
- [x] Unit tests for each component

### Deferred Items

| Item | Original Scope | Deferred To | Reason |
|------|---------------|-------------|--------|
| **Tabs keyboard navigation** | Arrow keys between tabs via `on_key_down()` | **Phase 5** | Requires `FocusHandle` which needs View-level entity, not RenderOnce |
| **Menubar hover-to-switch** | Hover to switch between open menus | **Phase 5** | Complex mouse tracking; click-to-switch sufficient for Phase 4 |
| **Menubar keyboard navigation** | Arrow keys between menus, up/down within | **Phase 5** | Requires `FocusHandle` and focus trapping |
| **Accordion animation** | `with_animation()` expand/collapse | **Phase 5** | Consistent with Phase 2 no-animation strategy |
| **Sidebar animation** | `with_animation()` width collapse | **Phase 5** | Same as above |
| **Shared Menu utility** | Shared rendering for DropdownMenu/ContextMenu/Menubar | **Future** | Direct reuse of DropdownMenuEntry types sufficient |
| **Shared FocusGroup helper** | Keyboard navigation patterns | **Phase 5** | Deferred with keyboard nav features |
| **Focus trap utility** | Keep focus within container | **Phase 5** | Same as above |

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

### Components (4)

- [ ] **Spinner** -- Loading spinner with rotation animation
  - Use `with_animation()` with repeating rotation
  - Custom paint via `canvas()` + `PathBuilder` for arc drawing
  - Deferred from Phase 3: shares custom painting infrastructure with Chart
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
| v0.2.0 | Phase 2 ✅ | 10 overlay/feedback | 22 |
| v0.3.0 | Phase 3 ✅ | 11 visual/display | 33 |
| v0.4.0 | Phase 4 ✅ | 12 navigation/structure | 45 |
| v0.5.0 | Phase 5 | 10 advanced interactive | 55 |
| v0.6.0 | Phase 6 | 4 data visualization + spinner | 59 |
| v1.0.0 | Phase 7 | Infrastructure + stability | 59 |
