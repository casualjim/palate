# Palate

File type detection combining the best of `tft` and `hyperpolyglot`.

## Acknowledgments

This project is a reassembly of code from several excellent projects:

- **[tft](https://github.com/RubixDev/tft)** - Tree-sitter File Type, providing fast file type detection using tree-sitter grammars
- **[Neovim](https://github.com/neovim/neovim)** - The original source of filetype detection heuristics and patterns
- **[hyperpolyglot](https://github.com/monkslc/hyperpolyglot)** - Language detection library with comprehensive language patterns

This crate essentially combines and curates the detection logic from these sources into a unified, ergonomic API.

## License

GPL-3.0-or-later

This project is derived from [tft](https://github.com/RubixDev/tft) (GPL-3.0-or-later), which itself incorporates code from [Neovim](https://github.com/neovim/neovim) (Apache-2.0/Vim license). As a derivative of GPL-3.0 work, this project is licensed under GPL-3.0-or-later.

## Features

- Comprehensive file type detection
- Fast PHF-based lookups
- Content-based detection with heuristics
- Naive Bayes classifier fallback (with `classifier` feature)

## Workspace packages

This repository is a Cargo workspace with focused packages:

- `crates/palate` (`palate`): the Rust library and detection API.
- `crates/palate-cli` (`palate-cli`): the command-line application; installs the `palate` binary.
- `crates/palate-capi` (`palate-capi`): the C API adapter.
- `crates/palate_polyglot_tokenizer`: tokenizer helper used by code generation.

The CLI moved to the dedicated `palate-cli` package. To install the command-line tool from this repository while preserving the `palate` executable name, run:

```sh
cargo install --path crates/palate-cli
```

## Rust library usage

```rust
use palate::{detect, FileType};

// Detect file type with fallback to Text
let ft = detect("main.rs", "");
assert_eq!(FileType::Rust, ft);

// Try detection without fallback
let ft = palate::try_detect("unknown.xyz", "");
assert_eq!(None, ft);
```

## C API usage

Build the adapter and include the checked-in header:

```sh
cargo build -p palate-capi
cc example.c -I crates/palate-capi/include target/debug/libpalate_capi.a -o example
```

Release archives include a relocatable `lib/pkgconfig/palate-capi.pc`. After unpacking one, C consumers can use:

```sh
PKG_CONFIG_PATH=/path/to/palate-capi/lib/pkgconfig cc example.c $(pkg-config --cflags --libs palate-capi)
```

Minimal C caller:

```c
#include "palate.h"
#include <stdint.h>
#include <stdio.h>

int main(void) {
  const char *file_type = NULL;
  const uint8_t content[] = "fn main() {}\n";
  palate_status_t status =
      palate_try_detect("main.rs", content, sizeof(content) - 1, &file_type);

  if (status == PALATE_STATUS_OK) {
    /* file_type is owned by Palate and must not be freed. */
    printf("%s\n", file_type);
    return 0;
  }
  if (status == PALATE_STATUS_NO_MATCH) {
    return 1;
  }
  return 2;
}
```

`palate_detect` falls back to `text`; `palate_try_detect` returns `PALATE_STATUS_NO_MATCH` when nothing matches. Returned file type strings are canonical, null-terminated, library-owned names.

## Releases

GitHub releases are produced with `cargo-dist`, following the same release-cut flow used by sibling projects. Release artifacts include:

- `palate-cli-<target>` archives and shell/PowerShell/Homebrew installers for the `palate` binary.
- `palate-capi-<target>` archives containing `include/palate.h`, the C-linkable `palate_capi` libraries, and `lib/pkgconfig/palate-capi.pc`.

The C API archives are intended for C/C++ and other FFI consumers. Set `PKG_CONFIG_PATH` to the unpacked archive's `lib/pkgconfig` directory and use `pkg-config --cflags --libs palate-capi`.

## WASM adapter status

This workspace intentionally does not include a WASM adapter in this change. If WASM support is needed later, it should be added as a separate focused adapter crate rather than expanding the core `palate` library or the C API adapter.

## Detection Pipeline

1. Path suffix matching
2. Filename matching
3. Pattern matching (with priorities)
4. File extension matching (PHF map)
5. Negative priority patterns
6. Content-based detection (dynamic resolvers)

## Features

- `detect` (default): Enable file type detection
- `classifier`: Enable naive Bayes classifier
- `serde`: Enable serde serialization for FileType
