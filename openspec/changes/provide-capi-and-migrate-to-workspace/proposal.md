## Why

Palate currently mixes the idiomatic Rust library and CLI concerns in one package, and it has no dedicated surface for C consumers. Splitting the repository into focused workspace crates lets the Rust API stay clean while adding a stable C adapter and keeping CLI dependencies out of the core library.

## What Changes

- Convert the repository into a Cargo workspace with focused member crates.
- Keep the core `palate` crate idiomatic Rust-only and free of C ABI, header, allocator, and CLI packaging concerns.
- Move the existing command-line application into a dedicated CLI crate that depends on the core library.
- Add a dedicated C API adapter crate that depends on the core library and exposes C-compatible dynamic/static library outputs plus a public header.
- Add C consumer smoke tests and CI coverage for the workspace split and C API package.
- Preserve the existing Rust detection API and CLI binary behavior unless explicitly changed by the split.
- **BREAKING**: Installing the command-line tool may move from the core `palate` package to the dedicated CLI package while preserving the installed binary name `palate`.

## Capabilities

### New Capabilities
- `workspace-packaging`: Defines the workspace layout and package boundaries for the core library, CLI, helper crates, and adapters with focused dependency ownership.
- `c-api-bindings`: Defines the C-compatible detection API, ABI stability rules, ownership model, header generation, and C consumer validation.

### Modified Capabilities

## Impact

- Affected code: root `Cargo.toml`, current `src/bin/palate.rs`, the tokenizer helper crate metadata, workspace manifests, and new adapter crate sources.
- Affected APIs: adds a C API; preserves the existing Rust crate API; preserves CLI user-facing behavior through a new package boundary.
- Affected dependencies: moves CLI-only dependencies such as `clap`, `ignore`, `termcolor`, and file scanning concerns out of the core library package where possible.
- Affected systems: CI/test matrix, release packaging, generated C headers, and downstream C consumers.
