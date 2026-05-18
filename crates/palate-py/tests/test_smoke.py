import palate


def test_version_is_non_empty():
    assert palate.version()


def test_detects_rust_from_bytes():
    assert palate.detect("main.rs", b"fn main() {}\n") == "rust"


def test_try_detect_none_for_no_match():
    assert palate.try_detect("unknown.file", b"") is None


def test_accepts_string_content_convenience():
    assert palate.detect("main.rs", "fn main() {}\n") == "rust"


def test_binary_content_with_embedded_nul():
    assert palate.detect("main.rs", b"fn main() {\0}\n") == "rust"
