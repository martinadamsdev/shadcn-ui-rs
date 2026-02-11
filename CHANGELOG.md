# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.1.0]: https://github.com/martinadamsdev/shadcn-ui-rs/releases/tag/v0.1.0
