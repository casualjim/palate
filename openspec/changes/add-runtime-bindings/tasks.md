## 1. Shared Contract Preparation

- [x] 1.1 Confirm the existing `palate::detect`, `palate::try_detect`, `FileType::to_string`, and `palate-capi` behavior for representative fixtures.
- [x] 1.2 Add a minimal core byte-detection helper only if Python/Node adapters would otherwise duplicate lossy UTF-8 conversion logic.
- [x] 1.3 Add or update Rust tests for any shared byte helper, including embedded NUL bytes and invalid UTF-8 bytes.

## 2. Python Binding

- [x] 2.1 Add a `crates/palate-py` workspace member configured for PyO3 extension-module builds.
- [x] 2.2 Add Python package metadata for maturin-based local builds.
- [x] 2.3 Implement `version()`, `detect(path, content)`, and `try_detect(path, content)` exports.
- [x] 2.4 Accept bytes as the primary content input and string content as a documented convenience.
- [x] 2.5 Return canonical strings for matches and `None` for try-detection no-match.
- [x] 2.6 Add Python smoke tests for version, Rust detection, no match, string content, and binary content with embedded NUL.

## 3. Node Binding

- [x] 3.1 Add a `crates/palate-napi` workspace member configured for napi-rs native addon builds.
- [x] 3.2 Add npm package metadata and generated TypeScript declarations for the Node package.
- [x] 3.3 Implement `version()`, `detect(path, content)`, and `tryDetect(path, content)` exports.
- [x] 3.4 Accept `Buffer` and `Uint8Array` content inputs with consistent byte conversion behavior.
- [x] 3.5 Choose and document the JavaScript no-match absence value for `tryDetect`.
- [x] 3.6 Add Node smoke tests for version, Rust detection, no match, Uint8Array input, and embedded NUL content.

## 4. Go Binding

- [x] 4.1 Add a Go module/package under `bindings/go` that uses cgo against `crates/palate-capi/include/palate.h`.
- [x] 4.2 Implement `Version() string`, `Detect(path string, content []byte) (string, error)`, and `TryDetect(path string, content []byte) (string, bool, error)`.
- [x] 4.3 Translate `palate_status_t` values into idiomatic Go success, no-match, and error results.
- [x] 4.4 Document the requirement to build/link the `palate-capi` library for local Go tests and consumers.
- [x] 4.5 Add Go smoke tests for version, Rust detection, no match, C API error propagation, and embedded NUL content.

## 5. WASM Compatibility

- [x] 5.1 Add a CI/local check that installs `wasm32-unknown-unknown` when needed.
- [x] 5.2 Ensure `cargo build -p palate --target wasm32-unknown-unknown` succeeds with detection enabled.
- [x] 5.3 Fix any target-incompatible core dependency or feature usage without adding a dedicated WASM adapter crate.

## 6. CI and Documentation

- [x] 6.1 Update CI to build and test the Python binding in a minimal local/maturin smoke-test flow.
- [x] 6.2 Update CI to build and test the Node binding in a minimal npm/napi-rs smoke-test flow.
- [x] 6.3 Update CI to build `palate-capi` before running Go binding tests.
- [x] 6.4 Add the WASM target build check to CI.
- [x] 6.5 Update README or binding documentation with the shared runtime API, package build commands, and non-goals such as no file scanning helpers.
- [ ] 6.6 Wire npm publishing through cargo-dist custom publish jobs using trusted publishing/provenance.
- [ ] 6.7 Wire PyPI publishing through cargo-dist custom publish jobs using trusted publishing/OIDC.

## 7. Verification

- [x] 7.1 Run `cargo fmt --all -- --check`.
- [x] 7.2 Run `cargo nextest run --workspace --all-features --no-tests pass`.
- [x] 7.3 Run Python binding smoke tests.
- [x] 7.4 Run Node binding smoke tests.
- [x] 7.5 Run Go binding smoke tests.
- [x] 7.6 Run the WASM target build check.
- [x] 7.7 Confirm all runtime bindings agree on canonical outputs for the shared fixtures.
