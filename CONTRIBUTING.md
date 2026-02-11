# Contributing to shadcn-ui-rs

Thank you for your interest in contributing to shadcn-ui-rs. This document covers the development setup, coding standards, and pull request process.

## Prerequisites

- **Rust** 1.93.0 or later (`rustup update stable`)
- **Xcode** (macOS only) -- required for GPUI's native rendering pipeline
- **Git**

## Getting Started

1. Fork the repository and clone your fork:

   ```bash
   git clone https://github.com/<your-username>/shadcn-ui-rs.git
   cd shadcn-ui-rs
   ```

2. Build the entire workspace:

   ```bash
   cargo build --workspace
   ```

3. Run the tests:

   ```bash
   cargo test --workspace
   ```

4. Run clippy to check for lint issues:

   ```bash
   cargo clippy --workspace
   ```

## Project Structure

```
shadcn-ui-rs/
├── crates/
│   ├── cli/              # CLI tool (shadcn-ui binary)
│   ├── registry/         # Component registry definitions
│   └── theme/            # Theme system core
├── components/           # Component source files (copied into user projects)
├── templates/            # Project templates used by `shadcn-ui init`
├── examples/             # Example projects
└── docs/                 # Documentation
```

## Adding a New Component

1. Create the component source file in `components/ui/`. Follow the builder pattern used by existing components (see `components/ui/button.rs` for reference).

2. Register the component in `crates/registry/src/lib.rs` with its metadata, dependencies, and source path.

3. Add documentation for the component in `docs/components.md`.

4. Write unit tests for the component.

5. If the component has dependencies on other components, declare them in the registry entry so the CLI can resolve them automatically.

## Code Style

- **Format** all code with `rustfmt` before committing. The project uses the default rustfmt configuration.
- **Lint** with `cargo clippy --workspace` and fix all warnings.
- **Document** all public APIs with doc comments (`///`).
- Use the **builder pattern** for component APIs:

  ```rust
  Button::new("Label")
      .variant(ButtonVariant::Outline)
      .size(ButtonSize::Lg)
      .on_click(|event, window, cx| { /* ... */ })
  ```

- Keep functions focused and small. Prefer composition over inheritance.

## Testing

- Every component should have basic unit tests.
- CLI commands should have integration tests.
- Run the full test suite before submitting a pull request:

  ```bash
  cargo test --workspace
  ```

## Pull Request Process

1. Create a feature branch from `main`:

   ```bash
   git checkout -b feat/my-feature main
   ```

2. Make your changes in small, focused commits.

3. Ensure all checks pass:

   ```bash
   cargo build --workspace
   cargo test --workspace
   cargo clippy --workspace
   ```

4. Push your branch and open a pull request against `main`.

5. In the PR description, explain what your change does and why. Link any relevant issues.

6. A maintainer will review your PR. Address any feedback and push additional commits as needed.

## Reporting Issues

- Use [GitHub Issues](https://github.com/martinadamsdev/shadcn-ui-rs/issues) to report bugs or request features.
- Include reproduction steps, expected behavior, and your environment (OS, Rust version).

## License

By contributing to shadcn-ui-rs, you agree that your contributions will be licensed under the [Apache License 2.0](LICENSE).
