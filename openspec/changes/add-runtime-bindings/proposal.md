## Why

Palate now has a clean Rust core crate and a C adapter, but non-Rust consumers still need first-class package surfaces. The detection API is small enough to expose consistently across Python, Node.js, Go, and WASM without expanding the core library or creating unnecessary adapter crates.

## What Changes

- Add a Python binding package using PyO3 for idiomatic Python wheels.
- Add a Node.js binding package using napi-rs for idiomatic npm/native addon consumption.
- Add a Go package that wraps the existing `palate-capi` library through cgo instead of adding another Rust adapter crate.
- Add a WASM compatibility target check for the core `palate` crate using `wasm32-unknown-unknown`, without adding a separate WASM wrapper crate.
- Keep the exposed runtime API intentionally small: `version`, `detect`, and `try_detect` over a path/name plus provided content bytes/text.
- Preserve the core Rust API and keep runtime/ABI packaging concerns outside `crates/palate` except for any minimal shared helper needed to avoid inconsistent byte handling.

## Capabilities

### New Capabilities
- `runtime-bindings`: Defines Python, Node.js, Go, and WASM consumption surfaces for Palate detection while preserving the small shared runtime API and existing core/package boundaries.

### Modified Capabilities

## Impact

- Affected code: workspace manifests, new Python and Node adapter packages/crates, Go package files, C API consumer packaging, CI, README/package documentation, and possibly a small shared byte-detection helper in `crates/palate`.
- Affected APIs: adds runtime-specific APIs for Python, Node.js, and Go; adds WASM build compatibility; preserves existing Rust and C APIs.
- Affected dependencies: adds PyO3/maturin-side dependencies for Python packaging, napi-rs-side dependencies for Node packaging, Go/cgo wrapper tooling, and a Rust WASM target check.
- Affected systems: CI matrix, release/package documentation, language-specific smoke tests, and downstream package installation guidance.
