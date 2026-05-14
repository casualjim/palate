## ADDED Requirements

### Requirement: Focused Cargo workspace packages
The repository SHALL be organized as a Cargo workspace with separate packages for the core Rust library, command-line application, C API adapter, and tokenizer helper crate.

#### Scenario: Workspace members are declared
- **WHEN** a developer inspects the workspace manifest
- **THEN** it lists workspace members for the core `palate` library, the CLI package, the C API adapter package, and `palate_polyglot_tokenizer`

#### Scenario: Workspace uses shared dependency resolution
- **WHEN** Cargo resolves the workspace
- **THEN** it uses workspace dependency resolution that prevents feature leakage between member packages

### Requirement: Core library remains idiomatic Rust-only
The core `palate` package SHALL preserve the existing Rust library API and SHALL NOT contain C ABI exports, C headers, C allocator helpers, or the user-facing CLI binary target.

#### Scenario: Rust library users depend on palate
- **WHEN** a Rust consumer depends on the core `palate` package
- **THEN** the consumer can use the existing Rust detection APIs including `detect`, `try_detect`, `is_text_file`, and `FileType`

#### Scenario: Core package excludes adapter concerns
- **WHEN** the core `palate` package is built by itself
- **THEN** it builds as an idiomatic Rust library without requiring CLI-only dependencies or C API adapter dependencies

### Requirement: CLI package owns command-line behavior
The CLI package SHALL provide the installed executable named `palate` and SHALL depend on the core library for detection behavior.

#### Scenario: CLI binary name is preserved
- **WHEN** the CLI package is built or installed
- **THEN** it produces an executable named `palate`

#### Scenario: CLI delegates detection to the core library
- **WHEN** the CLI analyzes a text file or directory
- **THEN** its file type results are derived from the core `palate` detection API

### Requirement: Package dependency ownership is focused
Each workspace package SHALL own only the dependencies required for its responsibility, with CLI scanning and argument parsing dependencies isolated from the core library package.

#### Scenario: CLI-only dependencies are isolated
- **WHEN** a developer inspects package manifests
- **THEN** dependencies used only for CLI argument parsing, directory walking, or terminal output are declared by the CLI package rather than the core library package

#### Scenario: Adapter-only dependencies are isolated
- **WHEN** a developer inspects package manifests
- **THEN** dependencies used only for C API packaging or C smoke testing are declared by the C API adapter package rather than the core library package

### Requirement: Workspace validation covers all packages
The project SHALL provide automated validation that builds and tests the workspace package set after the split.

#### Scenario: Workspace tests run in CI
- **WHEN** CI validates the project
- **THEN** it builds and tests all workspace packages required for the core library, CLI, tokenizer helper, and C API adapter
