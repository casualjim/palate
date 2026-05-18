//! C API adapter for Palate.

use std::{
    collections::HashMap,
    ffi::{c_char, CStr, CString},
    panic::{catch_unwind, AssertUnwindSafe},
    path::Path,
    ptr,
    sync::{Mutex, OnceLock},
};

use palate::FileType;

/// Operation completed successfully.
pub const PALATE_STATUS_OK: i32 = 0;
/// No file type matched.
pub const PALATE_STATUS_NO_MATCH: i32 = 1;
/// A required argument was null or invalid.
pub const PALATE_STATUS_INVALID_ARGUMENT: i32 = 2;
/// The path argument was not valid UTF-8.
pub const PALATE_STATUS_INVALID_UTF8: i32 = 3;
/// A Rust panic was caught at the FFI boundary.
pub const PALATE_STATUS_PANIC: i32 = 4;

type PalateStatus = i32;

static NAME_CACHE: OnceLock<Mutex<HashMap<&'static str, &'static CStr>>> = OnceLock::new();

fn name_cache() -> &'static Mutex<HashMap<&'static str, &'static CStr>> {
    NAME_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn cached_name_ptr(file_type: FileType) -> *const c_char {
    let name: &'static str = file_type.into();
    let mut cache = name_cache().lock().expect("file type name cache poisoned");
    let c_name = cache.entry(name).or_insert_with(|| {
        Box::leak(
            CString::new(name)
                .expect("canonical file type name contains no nul bytes")
                .into_boxed_c_str(),
        )
    });
    c_name.as_ptr()
}

unsafe fn content_bytes<'a>(
    content: *const u8,
    content_len: usize,
) -> Result<&'a [u8], PalateStatus> {
    if content.is_null() {
        if content_len == 0 {
            Ok(&[])
        } else {
            Err(PALATE_STATUS_INVALID_ARGUMENT)
        }
    } else {
        // SAFETY: The caller provides a non-null pointer to `content_len` bytes.
        Ok(unsafe { std::slice::from_raw_parts(content, content_len) })
    }
}

unsafe fn path_str<'a>(path: *const c_char) -> Result<&'a str, PalateStatus> {
    if path.is_null() {
        return Err(PALATE_STATUS_INVALID_ARGUMENT);
    }

    // SAFETY: The caller provides a non-null, NUL-terminated C string pointer.
    unsafe { CStr::from_ptr(path) }
        .to_str()
        .map_err(|_| PALATE_STATUS_INVALID_UTF8)
}

unsafe fn detect_impl(
    path: *const c_char,
    content: *const u8,
    content_len: usize,
    out_file_type: *mut *const c_char,
    fallback: bool,
) -> PalateStatus {
    if out_file_type.is_null() {
        return PALATE_STATUS_INVALID_ARGUMENT;
    }

    // SAFETY: `out_file_type` was checked for null and is caller-owned output storage.
    unsafe { *out_file_type = ptr::null() };

    let path = match unsafe { path_str(path) } {
        Ok(path) => path,
        Err(status) => return status,
    };
    let content = match unsafe { content_bytes(content, content_len) } {
        Ok(content) => content,
        Err(status) => return status,
    };
    let result = if fallback {
        Some(palate::detect_bytes(Path::new(path), content))
    } else {
        palate::try_detect_bytes(Path::new(path), content)
    };

    match result {
        Some(file_type) => {
            // SAFETY: `out_file_type` was checked for null and receives a library-owned string.
            unsafe { *out_file_type = cached_name_ptr(file_type) };
            PALATE_STATUS_OK
        }
        None => PALATE_STATUS_NO_MATCH,
    }
}

/// Return the Palate adapter version as a library-owned static string.
#[no_mangle]
pub extern "C" fn palate_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr().cast()
}

/// Detect a file type with fallback to `text`.
#[no_mangle]
pub unsafe extern "C" fn palate_detect(
    path: *const c_char,
    content: *const u8,
    content_len: usize,
    out_file_type: *mut *const c_char,
) -> PalateStatus {
    catch_unwind(AssertUnwindSafe(|| unsafe {
        detect_impl(path, content, content_len, out_file_type, true)
    }))
    .unwrap_or(PALATE_STATUS_PANIC)
}

/// Try to detect a file type without fallback.
#[no_mangle]
pub unsafe extern "C" fn palate_try_detect(
    path: *const c_char,
    content: *const u8,
    content_len: usize,
    out_file_type: *mut *const c_char,
) -> PalateStatus {
    catch_unwind(AssertUnwindSafe(|| unsafe {
        detect_impl(path, content, content_len, out_file_type, false)
    }))
    .unwrap_or(PALATE_STATUS_PANIC)
}
