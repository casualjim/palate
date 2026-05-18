//! Python bindings for Palate runtime detection.

use pyo3::{
    exceptions::PyTypeError,
    prelude::*,
    types::{PyBytes, PyString, PyStringMethods},
};

fn content_bytes(content: &Bound<'_, PyAny>) -> PyResult<Vec<u8>> {
    if let Ok(bytes) = content.cast::<PyBytes>() {
        return Ok(bytes.as_bytes().to_vec());
    }

    if let Ok(text) = content.cast::<PyString>() {
        return Ok(text.to_cow()?.as_bytes().to_vec());
    }

    Err(PyTypeError::new_err("content must be bytes or str"))
}

/// Return the packaged Palate adapter version.
#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Detect a file type from a path/name and caller-provided content.
#[pyfunction]
fn detect(path: &str, content: &Bound<'_, PyAny>) -> PyResult<String> {
    let content = content_bytes(content)?;
    Ok(palate_core::detect_bytes(path, &content).to_string())
}

/// Try to detect a file type without falling back to text.
#[pyfunction]
fn try_detect(path: &str, content: &Bound<'_, PyAny>) -> PyResult<Option<String>> {
    let content = content_bytes(content)?;
    Ok(palate_core::try_detect_bytes(path, &content).map(|file_type| file_type.to_string()))
}

/// Python module initializer.
#[pymodule]
fn palate(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(version, module)?)?;
    module.add_function(wrap_pyfunction!(detect, module)?)?;
    module.add_function(wrap_pyfunction!(try_detect, module)?)?;
    Ok(())
}
