## ADDED Requirements

### Requirement: Shared Runtime Detection Contract
The runtime bindings SHALL expose a small detection contract consisting of version, fallback detection, and no-fallback try-detection functions over a path/name and caller-provided content.

#### Scenario: Fallback detection returns canonical file type
- **WHEN** a runtime caller detects `main.rs` with Rust source content
- **THEN** the binding returns the canonical file type string `rust`

#### Scenario: Try-detection returns no match without text fallback
- **WHEN** a runtime caller try-detects an unknown path and content that does not match a known file type
- **THEN** the binding reports no match using the runtime's documented absence value

#### Scenario: Version is exposed
- **WHEN** a runtime caller requests the Palate version
- **THEN** the binding returns a non-empty version string from the packaged Palate adapter

### Requirement: Python Binding
The system SHALL provide a Python binding backed by PyO3 that exposes idiomatic Python functions for the shared runtime detection contract.

#### Scenario: Python detects from bytes
- **WHEN** Python code calls `detect("main.rs", b"fn main() {}")`
- **THEN** the function returns `"rust"`

#### Scenario: Python try_detect returns None for no match
- **WHEN** Python code calls `try_detect("unknown.file", b"")` and Palate finds no match
- **THEN** the function returns `None`

#### Scenario: Python accepts text content convenience
- **WHEN** Python code calls `detect("main.rs", "fn main() {}")`
- **THEN** the function returns the same canonical file type as the bytes input

### Requirement: Node Binding
The system SHALL provide a Node.js binding backed by napi-rs that exposes idiomatic JavaScript functions for the shared runtime detection contract.

#### Scenario: Node detects from Buffer
- **WHEN** Node.js code calls `detect("main.rs", Buffer.from("fn main() {}"))`
- **THEN** the function returns `"rust"`

#### Scenario: Node detects from Uint8Array
- **WHEN** Node.js code calls `detect("main.rs", new Uint8Array(Buffer.from("fn main() {}")))`
- **THEN** the function returns `"rust"`

#### Scenario: Node tryDetect reports no match
- **WHEN** Node.js code calls `tryDetect("unknown.file", Buffer.from(""))` and Palate finds no match
- **THEN** the function returns the documented JavaScript absence value

### Requirement: Go Binding
The system SHALL provide a Go package that wraps the existing Palate C API through cgo for the shared runtime detection contract.

#### Scenario: Go detects from byte slice
- **WHEN** Go code calls `Detect("main.rs", []byte("fn main() {}"))`
- **THEN** the function returns `"rust"` and a nil error

#### Scenario: Go try-detection reports no match separately from errors
- **WHEN** Go code calls `TryDetect("unknown.file", []byte{})` and Palate finds no match
- **THEN** the function returns an empty file type, `false`, and a nil error

#### Scenario: Go propagates C API errors
- **WHEN** the underlying C API returns an invalid argument, invalid UTF-8, panic, or other error status
- **THEN** the Go package returns a non-nil error instead of treating the result as a successful detection

### Requirement: WASM Target Compatibility
The core `palate` crate SHALL build for the `wasm32-unknown-unknown` target with detection enabled without adding a separate WASM adapter crate.

#### Scenario: Core crate builds for WASM target
- **WHEN** CI installs the `wasm32-unknown-unknown` Rust target and runs a build for the core `palate` crate
- **THEN** the build completes successfully

#### Scenario: No WASM wrapper package is required
- **WHEN** this change is implemented
- **THEN** no dedicated `palate-wasm` crate or JavaScript wrapper package is required for completion

### Requirement: Runtime Binding Consistency
All runtime bindings SHALL return canonical file type names and SHALL keep byte-to-text conversion behavior consistent with the existing C adapter.

#### Scenario: Binary content does not truncate at embedded NUL
- **WHEN** a runtime binding receives content bytes containing an embedded NUL byte
- **THEN** detection receives the full byte buffer converted with the documented lossy UTF-8 behavior instead of truncating at the NUL byte

#### Scenario: Bindings do not expose Rust enum internals
- **WHEN** a runtime caller receives a detection result
- **THEN** the result is a canonical string or the documented absence value, not a Rust enum discriminant or generated language table entry

#### Scenario: File reading helpers are absent from initial bindings
- **WHEN** a runtime user inspects the initial binding API
- **THEN** the API does not include file-reading, directory-scanning, or CLI traversal helpers
