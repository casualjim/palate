use std::{path::Path, process::Command};

#[test]
fn c_smoke_compiles_links_and_runs() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("workspace dir");
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let target_dir = workspace_dir.join("target").join(&profile);
    let lib = target_dir.join("libpalate_capi.a");

    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let build = Command::new(cargo)
        .args(["build", "-p", "palate-capi"])
        .current_dir(workspace_dir)
        .status()
        .expect("run cargo build for C API staticlib");
    assert!(build.success(), "cargo build -p palate-capi failed");
    assert!(lib.exists(), "missing static library: {}", lib.display());

    let exe = target_dir.join("palate-capi-c-smoke");
    let cc = std::env::var("CC").unwrap_or_else(|_| "cc".to_string());
    let compile = Command::new(cc)
        .arg(manifest_dir.join("tests/c_smoke.c"))
        .arg("-I")
        .arg(manifest_dir.join("include"))
        .arg(&lib)
        .arg("-o")
        .arg(&exe)
        .status()
        .expect("compile C smoke test");
    assert!(compile.success(), "C smoke test compilation/link failed");

    let run = Command::new(&exe).status().expect("run C smoke test");
    assert!(run.success(), "C smoke executable failed");
}
