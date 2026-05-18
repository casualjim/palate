## Context

Palate is now a Cargo workspace with a focused Rust core crate (`crates/palate`), CLI crate, and C adapter crate. The core detection surface is already small: given a path/name and content, return a canonical `FileType` or no match; `detect` adds the text fallback while `try_detect` does not. The C adapter already exposes that shape as static canonical strings behind `palate_version`, `palate_detect`, and `palate_try_detect`.

The next boundary is runtime packaging. Python and Node users expect installable packages with native bindings, Go can naturally consume the existing C ABI through cgo, and WASM only needs a target compatibility guarantee for now. Because the product surface is small, the change should bias toward thin adapters and smoke tests instead of a large cross-language abstraction layer.

## Goals / Non-Goals

**Goals:**

- Add first-class Python bindings using PyO3 and package metadata suitable for maturin-based builds.
- Add first-class Node.js bindings using napi-rs and package metadata suitable for npm/native addon builds.
- Add a Go package that calls the existing `palate-capi` API through cgo.
- Add a CI check that the core `palate` crate builds for `wasm32-unknown-unknown` with detection enabled, installing the target as needed.
- Keep all runtime surfaces limited to `version`, `detect`, and `try_detect` over path/name plus caller-provided content.
- Keep runtime adapter code thin, tested, and semantically aligned with Rust/C detection.

**Non-Goals:**

- Do not add a dedicated WASM crate or wasm-bindgen JavaScript wrapper in this change.
- Do not add file-reading, directory scanning, ignore-file handling, or CLI-like traversal helpers to the runtime bindings.
- Do not expose Rust enum layouts, numeric language IDs, or generated variant tables directly to runtimes.
- Do not route Python or Node through the C API unless a later packaging constraint makes that preferable.
- Implement automated publishing for Python and Node bindings through trusted publishers; Go distribution remains source/module based unless a separate packaging requirement appears.

## Decisions

### Use runtime-native bindings where package UX matters

Add two Rust workspace members:

- `crates/palate-py` for PyO3/maturin.
- `crates/palate-napi` for napi-rs.

Both crates depend on `palate` and translate host values into the existing Rust detection calls. They should return canonical file type names as strings, not expose `FileType` values across the runtime boundary.

Alternatives considered:

- Reuse the C API for Python and Node through FFI packages: rejected for the initial package because install ergonomics and type mapping are worse than PyO3/napi-rs for these ecosystems.
- Put PyO3 or napi-rs features on the core crate: rejected because it would couple the Rust library to runtime packaging and build-system concerns.

### Use the C API for Go

Add a Go package under a non-Rust workspace path such as `bindings/go` that uses cgo to include `crates/palate-capi/include/palate.h` and link against the `palate-capi` artifact. The Go package should translate C status codes into idiomatic Go returns:

- `Version() string`
- `Detect(path string, content []byte) (string, error)`
- `TryDetect(path string, content []byte) (string, bool, error)`

Alternatives considered:

- Add a Rust-generated Go-specific library: rejected because the existing C ABI is already the stable foreign-function boundary.
- Shell out to the CLI from Go: rejected because it changes performance, packaging, and error semantics.

### Treat WASM as compatibility, not a package

For this change, WASM means the core crate must compile for `wasm32-unknown-unknown` with the detection surface available. CI should install the target and run a build check such as `cargo build -p palate --target wasm32-unknown-unknown`.

Alternatives considered:

- Add `crates/palate-wasm`: rejected for now because the API surface is too small and there is no committed browser/npm wrapper requirement.
- Reuse the C ABI as a raw WASM ABI: rejected as a user-facing contract because JavaScript callers would need manual memory/pointer handling.

### Standardize byte handling across adapters

The primary content input for non-Rust runtimes should be bytes (`bytes` in Python, `Buffer`/`Uint8Array` in Node, `[]byte` in Go). Text/string conveniences may exist where idiomatic, but all byte inputs should be converted consistently with the existing C adapter behavior: lossy UTF-8 conversion before calling core detection.

If implementation would otherwise duplicate that conversion across adapters, add a small documented helper in the core crate, for example byte-oriented `detect`/`try_detect` wrappers that preserve the current Rust API and return `FileType`/`Option<FileType>`.

Alternatives considered:

- Require UTF-8 strings only: rejected because host runtimes commonly hold file contents as bytes and the C API already accepts byte buffers.
- Add a shared adapter crate: rejected initially because two direct Rust adapters plus the existing C adapter do not justify another package layer unless duplication becomes significant.

### Keep naming idiomatic but semantically equivalent

Python should use snake_case for `try_detect`; Node should use camelCase for `tryDetect`; Go should use exported PascalCase. `detect`/`Detect` must include the text fallback; `try_detect`/`tryDetect`/`TryDetect` must report no match without falling back to text.

The absence value should be idiomatic per runtime: `None` for Python, `null` or `undefined` for Node with one documented choice, and `(value, false, nil)` for Go.

## Risks / Trade-offs

- CI and toolchain breadth increases → Keep tests to package smoke tests and build checks first; keep publishing automation focused.
- Python/Node native package publishing is platform-sensitive → Publish both packages through trusted publishing/OIDC with package-specific build jobs.
- Go cgo requires a compiled/linkable C library → Document the local build/link requirement and add a smoke test that builds `palate-capi` first.
- WASM target may expose dependency incompatibilities → Start with a target build check and treat any incompatibility as part of the task before adding runtime wrappers.
- Runtime APIs can drift semantically → Use shared fixtures across Python, Node, Go, C, and Rust smoke tests for `detect`, `try_detect`, no-match, and binary-content cases.

## Migration Plan

1. Add any minimal byte-detection helper to `crates/palate` only if it reduces adapter duplication without changing existing Rust APIs.
2. Add `crates/palate-py` with PyO3 exports, package metadata, and Python smoke tests.
3. Add `crates/palate-napi` with napi-rs exports, package metadata, TypeScript declarations, and Node smoke tests.
4. Add `bindings/go` with cgo wrapper code, module metadata, documentation, and Go smoke tests against `palate-capi`.
5. Add a WASM target build check for the core crate to CI.
6. Update README/runtime binding documentation with install/build examples and the shared API contract.
7. Wire release publishing for Python and Node through cargo-dist custom jobs/reusable workflows so generated release CI remains reproducible.

## Open Questions

- What final package name should be used on PyPI?
- Should Node's no-match result be `null` or `undefined`?
- Should the Python package expose only module-level functions initially, or also a tiny typed wrapper class later?
