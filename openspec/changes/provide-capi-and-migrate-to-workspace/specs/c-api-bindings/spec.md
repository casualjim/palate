## ADDED Requirements

### Requirement: Dedicated C API adapter package
The workspace SHALL provide a dedicated C API adapter package that depends on the core `palate` library and is the only package that exports C ABI functions.

#### Scenario: Adapter package builds linkable artifacts
- **WHEN** a developer builds the C API adapter package
- **THEN** Cargo produces C-linkable dynamic and static library artifacts for the adapter package

#### Scenario: Core library is not polluted by C exports
- **WHEN** a developer inspects the core `palate` library package
- **THEN** C ABI exports and C header files are absent from the core package and present only in the C API adapter package

### Requirement: Public C header defines the supported ABI
The C API adapter SHALL provide a public C header that declares all supported C types, status codes, and functions required by C consumers.

#### Scenario: C consumer includes the header
- **WHEN** a C source file includes the public Palate header
- **THEN** the source file compiles without requiring Rust headers or Rust-specific generated files

#### Scenario: Header is validated by smoke test
- **WHEN** the adapter package tests run
- **THEN** at least one C smoke test compiles against the public header and links against the adapter artifact

### Requirement: Detection functions use C-compatible inputs and outputs
The C API SHALL provide detection functions that accept a UTF-8 path C string and byte content with explicit length, and return detection results through C-compatible status codes and output pointers.

#### Scenario: Successful detection returns canonical name
- **WHEN** a C caller passes path `main.rs` and Rust source content to the try-detection function
- **THEN** the function returns success and writes the canonical file type name `rust` to the output

#### Scenario: Unknown file can report no match
- **WHEN** a C caller passes an unknown path and content to the try-detection function
- **THEN** the function reports a no-match status without falling back to `text`

#### Scenario: Fallback detection can return text
- **WHEN** a C caller passes an unknown path and content to the fallback detection function
- **THEN** the function returns success and writes the canonical file type name `text` to the output

#### Scenario: Content length handles embedded NUL bytes
- **WHEN** a C caller passes content containing embedded NUL bytes and an explicit content length
- **THEN** the adapter reads the provided length rather than truncating content at the first NUL byte

### Requirement: Rust FileType layout is not exposed to C
The C API SHALL expose canonical file type names rather than Rust `FileType` enum discriminants or Rust enum layout.

#### Scenario: Header avoids generated language enum
- **WHEN** a C consumer inspects the public header
- **THEN** the header does not require a C enum variant for every Rust `FileType`

#### Scenario: Returned names are stable strings
- **WHEN** detection succeeds through the C API
- **THEN** the returned file type value is the same canonical string that the Rust core library exposes for that file type

### Requirement: C API ownership is explicit
The C API SHALL document the lifetime and ownership of every pointer it accepts or returns, and initial detection result strings SHALL NOT require callers to free them.

#### Scenario: Caller receives static canonical name
- **WHEN** detection succeeds and writes a file type name pointer
- **THEN** the caller can read the string without calling a Palate free function for that pointer

#### Scenario: Null arguments are rejected
- **WHEN** a C caller passes a null required pointer
- **THEN** the function returns an invalid-argument status and does not dereference the null pointer

### Requirement: FFI boundary reports errors without unwinding
The C API SHALL convert invalid arguments, invalid UTF-8 paths, no-match results, and Rust panics into documented C status codes, and MUST NOT unwind Rust panics across C frames.

#### Scenario: Invalid UTF-8 path is rejected
- **WHEN** a C caller passes a path that is not valid UTF-8
- **THEN** the function returns an invalid-UTF-8 status without calling the core detection API with that path

#### Scenario: Panic is contained
- **WHEN** an exported C function encounters a Rust panic while serving a C call
- **THEN** the panic is caught and translated into a panic status code instead of unwinding into C
