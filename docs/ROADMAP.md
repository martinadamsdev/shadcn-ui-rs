# Roadmap

This document outlines the planned development phases for shadcn-ui-rs.

## Phase 1 -- v0.1.0 (Current)

Core foundation: CLI tooling, theme system, and 12 essential components.

- **CLI**: init, add, remove, list, diff, update, theme commands
- **Theme system**: 5 presets (zinc, slate, stone, gray, neutral) with light/dark mode
- **Components (12)**: Button, Input, Label, Checkbox, Radio, Switch, Slider, Select, Toggle, ToggleGroup, Card, Dialog

## Phase 2 -- v0.2.0

Feedback and overlay components for richer user interactions.

- **Components (10)**: AlertDialog, Sheet, Drawer, Popover, Tooltip, HoverCard, Toast, Sonner, Alert, Dialog improvements
- Focus areas: animation support, layered rendering, dismissal behavior

## Phase 3 -- v0.3.0

Data display components for presenting information.

- **Components (10)**: Avatar, Badge, Calendar, Table, DataTable, Carousel, Chart, Progress, Skeleton, Textarea
- Focus areas: data binding patterns, virtualized rendering for large datasets

## Phase 4 -- v0.4.0

Navigation components for application structure.

- **Components (8)**: Tabs, NavigationMenu, Menubar, DropdownMenu, ContextMenu, Command, Breadcrumb, Pagination
- Focus areas: keyboard navigation, focus management, accessibility

## Phase 5 -- v0.5.0

Layout and specialized components to round out the library.

- **Components (12)**: Accordion, Collapsible, Separator, AspectRatio, ScrollArea, Resizable, Combobox, DatePicker, InputOTP, Sidebar, Stepper, Timeline
- Focus areas: complex state management, composite component patterns

## Phase 6 -- v1.0.0

Polish, infrastructure, and stable release.

- **Registry server**: HTTP API for component distribution
- **Documentation site**: Built with mdbook, hosted publicly
- **crates.io publication**: Publish CLI and theme crates
- **Homebrew formula**: `brew install shadcn-ui-rs`
- **CI/CD**: GitHub Actions for testing, linting, and release automation
- **Stability**: API stabilization, comprehensive test coverage, performance benchmarks
