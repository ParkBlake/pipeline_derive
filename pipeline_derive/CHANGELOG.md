# Changelog
All notable changes to this project will be documented in this file.

## [0.1.1] - Pipeline Attributes and Major Refactor

### Added
- `#[pipeline(skip = true)]` attribute support: generates pipeline methods (`process3`, `process4`) that always return `None`, effectively skipping processing.
- `#[pipeline(timeout = <milliseconds>)]` attribute support: pipeline methods print timeout info when called.
- Automatic `T: Clone` trait bound added to generic type parameter to support cloning of inner values in generated methods.
- Robust attribute parsing that:
  - Handles `skip` and `timeout` attributes explicitly.
  - Captures unknown attributes without breaking the derive macro.
- Updated example usage demonstrating attributes and behaviours.

### Changed
- **Massive internal refactor:** Redesigned attribute parsing, error handling, and codegen for better modularity, maintainability, and extensibility.
- Pipeline methods now take `&self` and internally clone the inner `Option<T>` value as needed, improving ergonomics and avoiding ownership consumption.
- Improved error handling with detailed spans for attribute parsing and type checks.

## [0.1.0] - Initial Release

- Provided basic `#[derive(Pipeline)]` macro to generate `process3` and `process4` methods chaining closures over a single `Option<T>` field.
- Pipeline methods short-circuit on `None` using `Option::and_then`.
- Minimal API focused on monadic-style composition over `Option<T>`.
