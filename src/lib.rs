#![doc = include_str!("../README.md")]
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

#[cfg(feature = "detect")]
use std::path::Path;

#[cfg(feature = "detect")]
pub use detect::*;
pub use list::FileType;

#[cfg(feature = "detect")]
mod detect;
mod list;

/// Internal resolver for file type detection.
#[derive(Clone)]
#[cfg(feature = "detect")]
enum FileTypeResolver {
    /// Static file type that is always returned.
    Static(FileType),
    /// Dynamic resolver function that examines file content.
    Dynamic(fn(&Path, &str) -> Option<FileType>),
}

#[cfg(feature = "detect")]
impl FileTypeResolver {
    /// Resolves the file type, potentially examining file content.
    fn resolve(&self, path: &Path, content: &str) -> Option<FileType> {
        match self {
            FileTypeResolver::Static(ft) => Some(*ft),
            FileTypeResolver::Dynamic(func) => func(path, content),
        }
    }
}

#[cfg(all(test, feature = "detect"))]
mod tests {
    use super::*;

    #[test]
    fn paths() {
        assert_eq!(FileType::Zsh, detect(Path::new("/etc/zprofile"), ""));
        assert_eq!(FileType::Toml, detect(Path::new(".cargo/config"), ""));
    }

    #[test]
    fn filenames() {
        assert_eq!(FileType::Json, detect(Path::new(".prettierrc"), ""));
        assert_eq!(FileType::CMake, detect(Path::new("CMakeLists.txt"), ""));
    }

    #[test]
    fn patterns() {
        assert_eq!(FileType::Scheme, detect(Path::new("highlights.scm"), ""));
        assert_eq!(
            FileType::TreeSitterQuery,
            detect(Path::new("a/b/c/queries/highlights.scm"), "")
        );
    }

    #[test]
    fn ts_extension_prefers_typescript() {
        let typescript = "export const x: number = 1;\nconsole.log(x)\n";
        assert_eq!(
            FileType::TypeScript,
            detect(Path::new("main.ts"), typescript)
        );
    }

    #[test]
    fn ts_extension_detects_qt_ts_as_xml() {
        let qt_ts = "<TS version=\"2.1\" language=\"en_US\"></TS>\n";
        assert_eq!(FileType::Xml, detect(Path::new("app.ts"), qt_ts));
    }

    #[test]
    fn test_variants_parse() {
        use std::str::FromStr;
        // Test various variants to find which ones fail
        assert!(FileType::from_str("text").is_ok(), "text should parse");
        assert!(FileType::from_str("8th").is_ok(), "8th should parse");
        assert!(FileType::from_str("a2ps").is_ok(), "a2ps should parse");
        assert!(FileType::from_str("ada").is_ok(), "ada should parse");
        assert!(FileType::from_str("agda").is_ok(), "agda should parse");
        assert!(FileType::from_str("bash").is_ok(), "bash should parse");
        assert!(FileType::from_str("rust").is_ok(), "rust should parse");
        assert!(FileType::from_str("python").is_ok(), "python should parse");
    }

    #[test]
    fn test_languages_toml_roundtrip() {
        use std::str::FromStr;

        // Parse the languages.toml fixture
        let content = std::fs::read_to_string("fixtures/languages.toml")
            .expect("Failed to read languages.toml fixture");

        let toml_data: toml::Value =
            toml::from_str(&content).expect("Failed to parse languages.toml");

        let languages = toml_data.get("languages").and_then(|l| l.as_array());

        if let Some(langs) = languages {
            let mut fromstr_passed = 0;
            let mut fromstr_failed = 0;
            #[cfg(feature = "serde")]
            let mut serde_passed = 0;
            #[cfg(feature = "serde")]
            let mut serde_failed = 0;
            let mut failures = Vec::new();
            let mut not_in_enum = Vec::new();

            for lang in langs {
                let name = lang
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("unknown");

                // Syntastica-style languages.toml uses `languages[].name` for the parser/language id,
                // and `languages[].file-types` for the editor filetypes we need to roundtrip.
                let file_types = lang
                    .get("file-types")
                    .and_then(|fts| fts.as_array())
                    .cloned()
                    .unwrap_or_default();

                for ft_value in file_types {
                    let ft_name = match ft_value.as_str() {
                        Some(v) => v,
                        None => {
                            fromstr_failed += 1;
                            failures
                                .push(format!("{name}: non-string file-type entry: {ft_value:?}"));
                            continue;
                        }
                    };

                    // Try to parse as FileType via FromStr
                    let result = FileType::from_str(ft_name);

                    if let Ok(ft) = result {
                        // Test FromStr roundtrip: ft_name -> FileType -> string -> FileType
                        let display = ft.to_string();
                        let reparsed = FileType::from_str(&display);

                        // `file-types` can contain aliases (e.g. "cs" and "csharp").
                        // `Display` is canonical, so don't require `display == ft_name`.
                        if reparsed.ok() == Some(ft) {
                            fromstr_passed += 1;
                        } else {
                            fromstr_failed += 1;
                            failures.push(format!(
                                "{name}: FromStr roundtrip failed (file-type={ft_name:?}, display={display:?})"
                            ));
                        }

                        // Test serde roundtrip: FileType -> json -> FileType
                        #[cfg(feature = "serde")]
                        {
                            let json = serde_json::to_string(&ft).expect("Failed to serialize");
                            let deser: Result<FileType, _> = serde_json::from_str(&json);

                            if deser.ok() == Some(ft) && json == format!("\"{}\"", display) {
                                serde_passed += 1;
                            } else {
                                serde_failed += 1;
                                failures.push(format!(
                                    "{name}: serde roundtrip failed (file-type={ft_name:?}, json={json:?})"
                                ));
                            }
                        }
                    } else {
                        // File-type not in our enum
                        fromstr_failed += 1;
                        not_in_enum.push(ft_name.to_string());
                        #[cfg(feature = "serde")]
                        {
                            serde_failed += 1;
                        }
                    }
                }
            }

            println!(
                "FromStr roundtrip: {} passed, {} failed",
                fromstr_passed, fromstr_failed
            );
            #[cfg(feature = "serde")]
            println!(
                "serde roundtrip: {} passed, {} failed",
                serde_passed, serde_failed
            );

            if !not_in_enum.is_empty() {
                not_in_enum.sort();
                not_in_enum.dedup();
                panic!(
                    "File-types not in FileType enum ({}):\n{}",
                    not_in_enum.len(),
                    not_in_enum
                        .iter()
                        .map(|l| format!("  - {}", l))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }

            if !failures.is_empty() {
                panic!(
                    "Roundtrip failures ({}):\n{}",
                    failures.len(),
                    failures
                        .iter()
                        .map(|f| format!("  {}", f))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
        }
    }
}
