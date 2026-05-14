use std::path::Path;

#[test]
fn core_manifest_excludes_cli_and_c_adapter_dependencies() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = std::fs::read_to_string(manifest_dir.join("Cargo.toml")).expect("read manifest");
    let parsed: toml::Value = toml::from_str(&manifest).expect("parse manifest");
    let deps = parsed
        .get("dependencies")
        .and_then(toml::Value::as_table)
        .expect("dependencies table");

    for dep in ["clap", "ignore", "termcolor", "cc"] {
        assert!(
            !deps.contains_key(dep),
            "core palate package should not depend on {dep}"
        );
    }
}

#[test]
fn core_sources_exclude_c_abi_exports_and_headers() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");
    let mut stack = vec![src_dir];

    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("read source dir") {
            let entry = entry.expect("read source entry");
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                continue;
            }
            let source = std::fs::read_to_string(&path).expect("read source file");
            assert!(
                !source.contains("#[no_mangle]")
                    && !source.contains("pub extern \"C\" fn")
                    && !source.contains("pub unsafe extern \"C\" fn"),
                "core source contains C ABI export markers: {}",
                path.display()
            );
        }
    }

    assert!(
        !manifest_dir.join("include").exists(),
        "core package should not contain public C headers"
    );
}
