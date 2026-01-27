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
- Shebang interpretation support
- Naive Bayes classifier fallback (with `classifier` feature)

## Usage

```rust
use palate::{detect, FileType};

// Detect file type with fallback to Text
let ft = detect("main.rs", "");
assert_eq!(FileType::Rust, ft);

// Try detection without fallback
let ft = palate::try_detect("unknown.xyz", "");
assert_eq!(None, ft);
```

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
