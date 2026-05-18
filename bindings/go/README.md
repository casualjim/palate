# Palate Go binding

Thin cgo wrapper around the existing `palate-capi` C ABI.

```go
fileType, err := palate.Detect("main.rs", []byte("fn main() {}"))
fileType, ok, err := palate.TryDetect("unknown.file", nil)
version := palate.Version()
```

`Detect` includes Palate's text fallback. `TryDetect` does not fall back and returns `("", false, nil)` when no type matches. Non-success C API statuses are returned as errors.

## Local build and test

Build the C API library first, then run Go tests with the debug library path available:

```sh
cargo build -p palate-capi
LD_LIBRARY_PATH="$PWD/target/debug:${LD_LIBRARY_PATH:-}" \
  DYLD_LIBRARY_PATH="$PWD/target/debug:${DYLD_LIBRARY_PATH:-}" \
  sh -c 'cd bindings/go && go test ./...'
```

The Go package does not read files or scan directories. Automated Go module proxy publishing is deferred.
