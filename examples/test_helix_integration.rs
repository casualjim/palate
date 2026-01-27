use palate::{detect, FileType};
use std::path::Path;

fn main() {
    // Test Helix file extensions
    let tests = vec![
        // Helix extensions (new from Helix)
        ("test.sv", FileType::SystemVerilog),  // SystemVerilog from Helix
        ("test.svh", FileType::SystemVerilog), // SystemVerilog header from Helix
        ("test.v", FileType::V),               // V language (has priority over Verilog)
        ("test.vh", FileType::Verilog),        // Verilog header from Helix

        // Helix filenames (new from Helix)
        ("APKBUILD", FileType::Apkbuild),      // Alpine package build from Helix (dedicated type)
        ("Justfile", FileType::Just),          // Just command runner from Helix
        ("Containerfile", FileType::Dockerfile), // Containerfile from Helix

        // Helix path suffixes (new from Helix)
        ("i3/config", FileType::Sh),           // i3 config from Helix
        ("debian/copyright", FileType::DebCopyright), // Debian copyright from Helix

        // Helix patterns (new from Helix)
        ("SConstruct", FileType::Python),      // SCons build file from Helix
        ("BUILD.bazel", FileType::Bzl),        // Bazel BUILD file from Helix
    ];

    println!("Testing Helix integration:");
    println!("========================");

    let mut passed = 0;
    let mut failed = 0;

    for (input, expected) in tests {
        let result = detect(Path::new(input), "");
        let status = if result == expected {
            passed += 1;
            "✓ PASS"
        } else {
            failed += 1;
            &format!("✗ FAIL: got {:?}", result)
        };
        println!("{:30} -> {:30} | {}", input, format!("{:?}", expected).as_str(), status);
    }

    println!();
    println!("Results: {} passed, {} failed", passed, failed);

    if failed > 0 {
        std::process::exit(1);
    }
}
