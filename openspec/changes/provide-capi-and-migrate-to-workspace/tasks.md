## 1. Workspace Restructure

- [x] 1.1 Convert the root `Cargo.toml` into a virtual workspace manifest with resolver `2`, shared package metadata, and shared dependency declarations.
- [x] 1.2 Move the current core library sources, generated code, tests, examples, fixtures, and package metadata into `crates/palate/` while keeping the package name `palate`.
- [x] 1.3 Add `crates/palate_polyglot_tokenizer/` as a workspace member and update path dependencies to use workspace-relative paths.
- [x] 1.4 Update generated-code paths, fixture paths, README paths, and test paths that break after moving the core package.
- [x] 1.5 Verify the core package still exposes `detect`, `try_detect`, `is_text_file`, and `FileType` to Rust consumers.

## 2. CLI Package Split

- [x] 2.1 Create `crates/palate-cli/` with a package manifest that depends on the core `palate` package.
- [x] 2.2 Move the existing CLI implementation from `src/bin/palate.rs` into the CLI package and keep the binary target name `palate`.
- [x] 2.3 Move CLI-only dependencies such as argument parsing, directory walking, and terminal/output helpers into the CLI package manifest.
- [x] 2.4 Add or move CLI smoke tests that verify single-file and directory analysis still use core detection results.
- [x] 2.5 Update documentation to describe the CLI package install command and note the package-boundary migration.

## 3. C API Adapter Package

- [x] 3.1 Create `crates/palate-capi/` with a manifest that depends on the core `palate` package and builds `cdylib` and `staticlib` artifacts.
- [x] 3.2 Add a public `include/palate.h` header with C-compatible status codes, result pointer documentation, version function declarations, and detection function declarations.
- [x] 3.3 Implement guarded `extern "C"` detection functions that accept a UTF-8 path C string plus `(content pointer, content length)` bytes.
- [x] 3.4 Return canonical file type names as non-owned `const char *` values and avoid exposing Rust `FileType` discriminants or a generated language enum.
- [x] 3.5 Translate null arguments, invalid UTF-8 paths, no-match results, and caught Rust panics into documented C status codes.
- [x] 3.6 Keep all C ABI exports, header files, and C-specific helpers inside the adapter package rather than the core package.

## 4. Validation and Tests

- [x] 4.1 Add Rust tests for the C adapter covering successful detection, no-match try detection, fallback-to-text detection, embedded NUL content, null arguments, and invalid UTF-8 paths.
- [x] 4.2 Add a C smoke test that includes `palate.h`, compiles with a C compiler, links against the adapter artifact, and verifies a successful `main.rs` detection.
- [x] 4.3 Add manifest or source checks that confirm the core package does not define C ABI exports or depend on CLI/C adapter-only packages.
- [x] 4.4 Run package-specific tests for `palate`, `palate-cli`, `palate_polyglot_tokenizer`, and `palate-capi`.
- [x] 4.5 Run workspace-level formatting and tests to ensure the split does not regress existing behavior.

## 5. CI and Documentation

- [x] 5.1 Update CI to build and test all workspace members and to build the C API adapter artifacts.
- [x] 5.2 Update README and package documentation to describe the workspace layout, core crate, CLI crate, and C API adapter crate.
- [x] 5.3 Document C API usage with a minimal C example showing include path, link target, detection call, statuses, and string ownership.
- [x] 5.4 Document the intentional absence of a WASM adapter in this change and leave WASM as a future adapter crate if needed.
- [x] 5.5 Run `openspec status --change provide-capi-and-migrate-to-workspace` and confirm the change is apply-ready.
