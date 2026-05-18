# Palate Python binding

Thin PyO3 bindings for Palate runtime detection.

```python
import palate

palate.version()
palate.detect("main.rs", b"fn main() {}")      # "rust"
palate.try_detect("unknown.file", b"")         # None
```

`content` is primarily `bytes`; `str` is accepted as a UTF-8 convenience. The binding does not read files, scan directories, or publish wheels automatically in this change.

## Local build

```sh
mise exec -- env -u UV_PYTHON uv venv .venv
mise exec -- env -u UV_PYTHON uv pip install --python .venv maturin pytest
mise exec -- .venv/bin/python -m maturin develop --manifest-path crates/palate-py/Cargo.toml
mise exec -- .venv/bin/python -m pytest crates/palate-py/tests
```
