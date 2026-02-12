# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-02-12

### Added

- 12 Phase 4 navigation and structural components: Tabs, Accordion, Collapsible, Breadcrumb, Pagination, ButtonGroup, Field, ContextMenu, Menubar, NavigationMenu, Sidebar, and Item.
- ContextMenu and Menubar reuse DropdownMenuEntry/DropdownMenuItem types from Phase 2, enabling consistent menu rendering across the library.
- Public accessor methods on DropdownMenuItem (value, label, is_disabled, is_destructive) for cross-module reuse.

## [0.3.0] - 2026-02-12

### Added

- 11 Phase 3 visual display components: Badge, Avatar, Separator, Skeleton, Progress, Kbd, Typography (H1-H4, Paragraph, Blockquote, InlineCode, Lead), Table, ScrollArea, Textarea, and Empty.
- Updated example project to showcase all 33 Phase 1-3 components.

### Fixed

- Component source files now include all required GPUI trait imports (`prelude::*`), ensuring components compile when added to user projects via `shadcn-ui add`.
- Theme template generates valid float literals in `hsl()` calls (e.g. `hsl(0.0, 0.0, 98.0)` instead of `hsl(0, 0, 98)`).
- Theme template derives `Clone` on `ThemeColors` struct.

## [0.2.0] - 2026-02-12

### Added

- 10 Phase 2 overlay and feedback components: Alert, AlertDialog, Tooltip, Popover, HoverCard, DropdownMenu, Sheet, Drawer, Toast, and Sonner.
- Cross-platform CI: macOS, Linux, Windows.

## [0.1.0] - 2026-02-12

### Added

- CLI tool (`shadcn-ui`) with init, add, remove, list, diff, update, and theme commands.
- Theme system with 5 presets: zinc, slate, stone, gray, and neutral.
- Light and dark mode support for all theme presets.
- 12 Phase 1 components: Button, Input, Label, Checkbox, Radio, Switch, Slider, Select, Toggle, ToggleGroup, Card, and Dialog.
- Builder pattern API for all components, following idiomatic GPUI conventions.
- Component dependency resolution in the CLI (e.g., adding Dialog automatically pulls in Button).
- Example project (`basic-form`) demonstrating component usage.
- Project templates for quick scaffolding via `shadcn-ui init`.
- Documentation: getting started guide, full component API reference, and theming guide.
- Component registry with metadata, dependencies, and source code for all 12 components.

[0.4.0]: https://github.com/martinadamsdev/shadcn-ui-rs/releases/tag/v0.4.0
[0.3.0]: https://github.com/martinadamsdev/shadcn-ui-rs/releases/tag/v0.3.0
[0.2.0]: https://github.com/martinadamsdev/shadcn-ui-rs/releases/tag/v0.2.0
[0.1.0]: https://github.com/martinadamsdev/shadcn-ui-rs/releases/tag/v0.1.0
