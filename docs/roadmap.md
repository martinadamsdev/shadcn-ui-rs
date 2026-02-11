# Roadmap

This document outlines the planned development phases for shadcn-ui-rs.

## Phase 1 -- v0.1.0 (Current)

Core foundation: CLI tooling, theme system, and 12 essential components.

- **CLI**: init, add, remove, list, diff, update, theme commands
- **Theme system**: 5 presets (zinc, slate, stone, gray, neutral) with light/dark mode
- **Components (12)**: Button, Input, Label, Checkbox, Radio, Switch, Slider, Select, Toggle, ToggleGroup, Card, Dialog

## Phase 2 -- v0.2.0

Overlay and feedback components. These all use GPUI's `deferred()` rendering for layered display with absolute positioning and `on_mouse_down_out()` dismissal -- patterns already validated by the Dialog and Select components.

- **Components (10)**: AlertDialog, Tooltip, Popover, HoverCard, Sheet, Drawer, DropdownMenu, Toast, Sonner, Alert
- Focus areas: layered rendering with `deferred()`, timed dismissal, slide-in positioning

## Phase 3 -- v0.3.0

Visual and data display components. Pure layout or simple state-driven rendering, all within GPUI's proven flex/styling capabilities.

- **Components (12)**: Badge, Avatar, Separator, Skeleton, Progress, Spinner, Empty, Kbd, Typography, Table, ScrollArea, Textarea
- Focus areas: `overflow_y_scroll()` for ScrollArea, flex-based table layout

## Phase 4 -- v0.4.0

Navigation and interaction pattern components. Click-driven panel switching, conditional rendering, and simple keyboard event handling via `on_key_down()`.

- **Components (8)**: Tabs, Accordion, Collapsible, Breadcrumb, Pagination, ButtonGroup, Field, Item
- Focus areas: active tab tracking, expand/collapse state management

## Phase 5 -- v0.5.0

Platform integration. Components requiring deeper GPUI integration or custom `Element` implementations for features beyond standard div-based rendering.

- **Interactive Slider**: Drag interaction via custom GPUI `Element` with hit-testing
- **Editable Input / Textarea**: Platform text input integration (e.g. `gpui::TextInput`)
- **NativeSelect**: Native OS select dropdown
- Focus areas: custom `Element` trait implementations, platform API integration

## Phase 6 -- v1.0.0

Polish, infrastructure, and stable release.

- **Registry server**: HTTP API for component distribution
- **Documentation site**: Built with mdbook, hosted publicly
- **crates.io publication**: Publish CLI and theme crates
- **Homebrew formula**: `brew install shadcn-ui-rs`
- **CI/CD**: GitHub Actions for testing, linting, and release automation
- **Stability**: API stabilization, comprehensive test coverage, performance benchmarks

---

## Components Not Planned for Port

The following shadcn/ui components are not suitable for a GPUI port due to fundamental framework limitations. They are excluded from the roadmap unless GPUI adds the required capabilities in a future release.

| Component | Reason |
|-----------|--------|
| **Chart** | Requires canvas or SVG rendering for drawing lines, bars, and areas. GPUI has no canvas/SVG drawing API -- only div-based box layout. |
| **DataTable** | Requires virtualized scrolling for large datasets, column resizing (drag), sortable headers, row selection state, and inline editing. The combination of these features is beyond what div-based layout can handle performantly. |
| **Calendar** | Requires a complex 7-column date grid with month/year navigation, date range selection, and keyboard arrow-key navigation across cells. While theoretically possible, the effort is disproportionate to the value in a desktop framework context. |
| **DatePicker** | Depends on Calendar + Popover + editable text input. Cannot be built until all three dependencies are available. |
| **Carousel** | Requires drag/swipe gestures with smooth animated transitions between slides. GPUI does not expose element-relative drag positions or animation interpolation. |
| **Resizable** | Requires dragging a split handle to resize adjacent panels. GPUI does not expose element-relative mouse positions needed to calculate resize deltas during drag. |
| **Combobox** | Requires a live editable text input with real-time filtering of a dropdown list as the user types. Depends on editable text input, which GPUI does not natively provide. |
| **Command** | A command palette with fuzzy search. Same dependency on editable text input plus real-time filtering and keyboard navigation across dynamic results. |
| **InputOTP** | Requires multiple independent single-character editable input fields with automatic focus advance. Depends on editable text input and precise focus management between fields. |
| **ContextMenu** | Requires capturing the right-click event with precise cursor coordinates to position a menu. GPUI's event model does not expose right-click position data. |
| **Menubar** | Requires multi-level nested dropdown menus with hover-to-open behavior across menu items, precise sub-menu positioning, and complex focus trapping. The interaction model is significantly more complex than a simple dropdown. |
| **NavigationMenu** | Same issues as Menubar: multi-level hover-triggered dropdowns with precise positioning and focus management. |
| **Sidebar** | Typically requires collapsible + resizable behavior. Could be partially implemented as a static layout, but the interactive features (drag to resize, collapse animation) depend on unsupported GPUI capabilities. |
| **AspectRatio** | GPUI's layout model does not support CSS-style `aspect-ratio` constraints. A fixed-size wrapper is possible but defeats the purpose of a responsive aspect-ratio component. |
| **Direction** | Provides RTL (right-to-left) text direction support. GPUI does not have native RTL layout capabilities. |
