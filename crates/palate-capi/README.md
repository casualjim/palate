# palate-capi

C API adapter for Palate file type detection.

This package is the only workspace member that exports C ABI functions or ships C headers. It depends on the core Rust `palate` library and builds `staticlib` and `cdylib` artifacts named `palate_capi`.

```sh
cargo build -p palate-capi
```

The public header is available at `include/palate.h`. Release archives also include `lib/pkgconfig/palate-capi.pc` so C consumers can build with:

```sh
PKG_CONFIG_PATH=/path/to/palate-capi/lib/pkgconfig cc example.c $(pkg-config --cflags --libs palate-capi)
```

The `palate-capi.pc` file is relocatable relative to its own location.
