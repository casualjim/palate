use std::{ffi::CStr, path::Path, ptr};

use palate_capi::{
    palate_detect, palate_try_detect, PALATE_STATUS_INVALID_ARGUMENT, PALATE_STATUS_INVALID_UTF8,
    PALATE_STATUS_NO_MATCH, PALATE_STATUS_OK,
};

unsafe fn name_from_ptr(ptr: *const std::ffi::c_char) -> String {
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .expect("file type name is utf8")
        .to_string()
}

#[test]
fn try_detect_returns_canonical_name() {
    let path = c"main.rs";
    let content = b"fn main() {}\n";
    let mut out = ptr::null();

    let status =
        unsafe { palate_try_detect(path.as_ptr(), content.as_ptr(), content.len(), &mut out) };

    assert_eq!(status, PALATE_STATUS_OK);
    assert_eq!(unsafe { name_from_ptr(out) }, "rust");
}

#[test]
fn try_detect_unknown_reports_no_match() {
    let path = c"unknown.unknownext";
    let content = b"???\n";
    let mut out = ptr::null();

    let status =
        unsafe { palate_try_detect(path.as_ptr(), content.as_ptr(), content.len(), &mut out) };

    assert_eq!(status, PALATE_STATUS_NO_MATCH);
    assert!(out.is_null());
}

#[test]
fn fallback_detect_returns_text_for_unknown() {
    let path = c"unknown.unknownext";
    let content = b"plain text\n";
    let mut out = ptr::null();

    let status = unsafe { palate_detect(path.as_ptr(), content.as_ptr(), content.len(), &mut out) };

    assert_eq!(status, PALATE_STATUS_OK);
    assert_eq!(unsafe { name_from_ptr(out) }, "text");
}

#[test]
fn content_length_allows_embedded_nul() {
    let path = c"main.rs";
    let content = b"fn main() {\0}\n";
    let mut out = ptr::null();

    let status =
        unsafe { palate_try_detect(path.as_ptr(), content.as_ptr(), content.len(), &mut out) };

    assert_eq!(status, PALATE_STATUS_OK);
    assert_eq!(unsafe { name_from_ptr(out) }, "rust");
}

#[test]
fn null_arguments_are_invalid() {
    let path = c"main.rs";
    let content = b"fn main() {}\n";
    let mut out = ptr::null();

    let null_path =
        unsafe { palate_try_detect(ptr::null(), content.as_ptr(), content.len(), &mut out) };
    assert_eq!(null_path, PALATE_STATUS_INVALID_ARGUMENT);

    let null_content_with_len =
        unsafe { palate_try_detect(path.as_ptr(), ptr::null(), 1, &mut out) };
    assert_eq!(null_content_with_len, PALATE_STATUS_INVALID_ARGUMENT);

    let null_out = unsafe {
        palate_try_detect(
            path.as_ptr(),
            content.as_ptr(),
            content.len(),
            ptr::null_mut(),
        )
    };
    assert_eq!(null_out, PALATE_STATUS_INVALID_ARGUMENT);
}

#[test]
fn invalid_utf8_path_is_rejected() {
    let path = b"\xff\0";
    let content = b"fn main() {}\n";
    let mut out = ptr::null();

    let status = unsafe {
        palate_try_detect(
            path.as_ptr().cast(),
            content.as_ptr(),
            content.len(),
            &mut out,
        )
    };

    assert_eq!(status, PALATE_STATUS_INVALID_UTF8);
    assert!(out.is_null());
}

#[test]
fn pkg_config_metadata_matches_package_version() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let pc = std::fs::read_to_string(manifest_dir.join("lib/pkgconfig/palate-capi.pc"))
        .expect("read pkg-config file");

    assert!(
        pc.contains(&format!("Version: {}", env!("CARGO_PKG_VERSION"))),
        "pkg-config version must match package version"
    );
    assert!(pc.contains("Libs: -L${libdir} -lpalate_capi"));
    assert!(pc.contains("Cflags: -I${includedir}"));
}
