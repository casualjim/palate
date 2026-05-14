## Context

The current repository has a single root `palate` package that contains the public Rust library and the `palate` CLI binary. Its manifest also carries CLI-only dependencies (`clap`, `ignore`, `termcolor`) alongside library dependencies, while the tokenizer helper crate lives under `crates/` but is not a workspace member. There is no existing C or WASM adapter crate, no C header, and no C consumer test.

The core library already exposes the right Rust-facing concepts: `detect`, `try_detect`, `is_text_file`, and `FileType` with canonical string conversion. The FFI boundary should adapt those concepts without changing the core library into a C-shaped API.

## Goals / Non-Goals

**Goals:**

- Make the repository a Cargo workspace with focused package ownership.
- Keep `palate` as the idiomatic Rust core library package and remove CLI-only dependencies from its normal dependency set.
- Move the CLI implementation to a dedicated package while keeping the executable name `palate`.
- Add a dedicated C adapter package that exposes a small, stable ABI and public C header.
- Validate the C header and linkable artifact with an automated C smoke test.
- Preserve the Rust library API surface for existing Rust users.

**Non-Goals:**

- Do not expose Rust's `FileType` enum layout directly to C.
- Do not add C ABI types, allocator functions, or header generation logic to the core `palate` library crate.
- Do not build a WASM adapter as part of this change.
- Do not redesign the detection pipeline or classifier beyond changes required by package boundaries.
- Do not require C consumers to understand Rust ownership or Rust panic behavior.

## Decisions

### Use a virtual workspace root

Create a virtual root `Cargo.toml` with `[workspace]`, resolver `2`, shared workspace metadata, and shared dependency declarations. Move the current core library sources into `crates/palate/`, make `crates/palate_polyglot_tokenizer/` a workspace member, and add new `crates/palate-cli/` and `crates/palate-capi/` members.

Alternatives considered:

- Keep the root as both workspace and package: rejected because it continues to blur package responsibilities.
- Rename the core library package: rejected because preserving `palate` for Rust library users is more important than preserving the old mixed package shape.

### Keep the CLI in its own package

Move `src/bin/palate.rs` into `crates/palate-cli/src/main.rs`. The package should depend on `palate` and own dependencies such as `clap`, `ignore`, and terminal/output helpers. The binary target remains named `palate`.

This likely changes the Cargo install package from `palate` to the CLI package name while preserving the installed binary name. Documentation and release notes should call out the migration.

Alternatives considered:

- Leave a CLI shim in the core package: rejected because it keeps CLI concerns attached to the core package.
- Make `palate` the CLI package and rename the core package: rejected because it would force Rust library users to change dependency declarations.

### Add a separate C adapter crate

Create `crates/palate-capi/` as the only crate that contains `extern "C"` functions, `#[no_mangle]` exports, C status codes, C-compatible structs, and header files. It depends on `palate` and builds `cdylib` and `staticlib` artifacts. The core crate should only expose safe Rust APIs; any C-compatible translation happens in the adapter.

Alternatives considered:

- Add an `ffi` feature to `palate`: rejected because it pollutes the core crate with ABI and ownership concerns.
- Use cbindgen over Rust core types directly: rejected because the macro-generated, non-exhaustive `FileType` enum is not a stable C contract.

### Expose strings, not Rust enum discriminants

The C API should return canonical file type names as `const char *` values with static lifetime. It should not expose `FileType` discriminants or a generated 1,000+ variant C enum. C callers that need comparisons can compare canonical names.

A minimal API shape should include:

- a version function,
- a fallback detection function equivalent to `palate::detect`,
- a try-detection function equivalent to `palate::try_detect`,
- status codes for success, no match, invalid arguments, invalid UTF-8 path, and panic containment.

Content should be accepted as `(const uint8_t *content, size_t content_len)` so embedded NUL bytes do not truncate the buffer. The adapter may convert content with `String::from_utf8_lossy`, matching the CLI's current approach. Paths should be accepted as UTF-8 C strings to keep behavior portable and aligned with the current `Path::to_str()`-oriented detection logic.

Alternatives considered:

- Return owned strings requiring a free function: rejected for the initial API because canonical names are static and no allocation is needed.
- Return numeric IDs: rejected unless a later requirement needs stable integer IDs; maintaining a stable ID table is extra contract surface.

### Contain panics at the FFI boundary

Every exported C function that calls Rust detection logic should use `std::panic::catch_unwind` and translate panics to a C status code. Panics must not unwind across C frames.

Alternatives considered:

- Rely on `panic = abort`: rejected because it is a build-profile policy and not a complete ABI safety story for library consumers.

### Check in the public C header

Provide a reviewed `include/palate.h` in the C adapter crate. The header is the source of truth for C callers and should be validated by a C smoke test. Automated generation can be added later if it proves valuable, but direct cbindgen generation from core Rust types is intentionally avoided.

Alternatives considered:

- Generate the header from Rust types: rejected for the initial implementation because the exported ABI should be deliberately smaller than the Rust internals.

## Risks / Trade-offs

- Cargo install compatibility changes → Document the new CLI package install command and preserve the binary name `palate`.
- Workspace move breaks paths in tests, docs, or codegen → Keep generated data and fixtures reachable, and add workspace-level test coverage before removing old paths.
- FFI status model underspecifies errors → Define explicit statuses in the header and add tests for null pointers, invalid paths, no match, and successful detection.
- C API over canonical strings may be less ergonomic for switch statements → Prefer a small stable initial ABI; add stable numeric IDs later only if needed.
- Panics or invalid UTF-8 could cross unsafe boundaries → Convert all exported functions through a shared guarded helper and return status codes.
- Static and dynamic library naming can vary by platform → Document expected artifact names and validate Linux/macOS where CI supports it.

## Migration Plan

1. Introduce the virtual workspace and move the existing core library into `crates/palate/` without changing public Rust APIs.
2. Register the tokenizer helper crate as a workspace member and update path dependencies.
3. Move the CLI source into `crates/palate-cli/`, keep the binary name `palate`, and move CLI-only dependencies there.
4. Add `crates/palate-capi/` with its manifest, C header, Rust adapter source, and C smoke test.
5. Update README/release documentation for workspace packages, CLI installation, and C API usage.
6. Update CI to test all workspace members and build the C adapter artifacts.
7. If the split causes unacceptable packaging regressions, roll back by reverting the workspace package move while keeping the OpenSpec artifacts for a revised compatibility design.

## Open Questions

- Should the C library artifact link name be `palate_capi` to avoid ambiguity, or `palate` for a friendlier C consumer experience?
- Should the initial C API include only detection-by-provided-content, or also file-reading helpers that mirror the CLI's file path behavior?
- Should codegen stay as a feature-gated tool in the core package or move to a future dedicated workspace tool crate after the main split lands?
