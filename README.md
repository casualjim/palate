# Palate

File type detection combining the best of `tft` and `hyperpolyglot`.

## License

This crate is dual-licensed under:
- GPL-3.0-or-later (from tft)
- MIT OR Apache-2.0 (from hyperpolyglot)

You may choose either license for your use.

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
