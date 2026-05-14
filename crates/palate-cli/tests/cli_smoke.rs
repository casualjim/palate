use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn tmp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("palate-cli-{name}-{nonce}"));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

#[test]
fn single_file_reports_core_detection_result() {
    let dir = tmp_dir("single");
    let file = dir.join("main.rs");
    fs::write(&file, "fn main() {}\n").expect("write rust file");

    let output = Command::new(env!("CARGO_BIN_EXE_palate"))
        .arg(&file)
        .output()
        .expect("run palate CLI");

    fs::remove_dir_all(&dir).ok();

    assert!(output.status.success(), "CLI failed: {output:?}");
    let stdout = String::from_utf8(output.stdout).expect("stdout is utf8");
    assert!(stdout.contains("main.rs"), "stdout: {stdout}");
    assert!(stdout.contains("rust"), "stdout: {stdout}");
}

#[test]
fn directory_summary_reports_core_detection_result() {
    let dir = tmp_dir("dir");
    fs::write(dir.join("main.rs"), "fn main() {}\n").expect("write rust file");

    let output = Command::new(env!("CARGO_BIN_EXE_palate"))
        .arg(&dir)
        .output()
        .expect("run palate CLI");

    fs::remove_dir_all(&dir).ok();

    assert!(output.status.success(), "CLI failed: {output:?}");
    let stdout = String::from_utf8(output.stdout).expect("stdout is utf8");
    assert!(stdout.contains("rust"), "stdout: {stdout}");
    assert!(stdout.contains("Total"), "stdout: {stdout}");
}
