//! Node.js bindings for Palate runtime detection.

use napi::bindgen_prelude::Uint8Array;
use napi_derive::napi;

/// Return the packaged Palate adapter version.
#[napi]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Detect a file type from a path/name and caller-provided content bytes.
#[napi]
pub fn detect(path: String, content: Uint8Array) -> String {
    palate::detect_bytes(path, content.as_ref()).to_string()
}

/// Try to detect a file type without falling back to text.
///
/// JavaScript receives `null` when Palate finds no match.
#[napi(js_name = "tryDetect")]
pub fn try_detect(path: String, content: Uint8Array) -> Option<String> {
    palate::try_detect_bytes(path, content.as_ref()).map(|file_type| file_type.to_string())
}
