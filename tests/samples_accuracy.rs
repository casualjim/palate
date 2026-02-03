use std::{collections::HashMap, path::Path};

use palate::FileType;

#[derive(Debug, serde::Deserialize)]
struct LanguageEntry {
    aliases: Option<Vec<String>>,
    fs_name: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct GrammarsMappingEntry {
    effective_filetype: Option<String>,
    nvim_filetype: Option<String>,
    grammar: Option<String>,
    nvim_parser: Option<String>,
}

fn read_languages_yml() -> HashMap<String, LanguageEntry> {
    let content = std::fs::read_to_string("languages.yml").expect("read languages.yml");
    serde_norway::from_str(&content).expect("parse languages.yml")
}

fn read_filetype_to_parser_map() -> Option<HashMap<FileType, Vec<String>>> {
    use std::str::FromStr;

    let content = std::fs::read_to_string("target/grammars-mapping-enhanced.json").ok()?;
    let entries: Vec<GrammarsMappingEntry> = serde_json::from_str(&content).ok()?;

    let mut map: HashMap<FileType, Vec<String>> = HashMap::new();

    for entry in entries {
        let ft_name = entry
            .effective_filetype
            .as_deref()
            .unwrap_or("")
            .trim()
            .to_string();
        let ft_name = if ft_name.is_empty() {
            entry.nvim_filetype.as_deref().unwrap_or("").trim().to_string()
        } else {
            ft_name
        };
        if ft_name.is_empty() {
            continue;
        }

        let parser_id = entry
            .grammar
            .as_deref()
            .unwrap_or("")
            .trim()
            .to_string();
        let parser_id = if parser_id.is_empty() {
            entry.nvim_parser.as_deref().unwrap_or("").trim().to_string()
        } else {
            parser_id
        };
        if parser_id.is_empty() {
            continue;
        }

        let Ok(ft) = FileType::from_str(&ft_name) else {
            continue;
        };

        map.entry(ft).or_default().push(parser_id);
    }

    for parsers in map.values_mut() {
        parsers.sort();
        parsers.dedup();
    }

    Some(map)
}

fn language_key_for_samples_dir<'a>(
    samples_dir_name: &str,
    languages: &'a HashMap<String, LanguageEntry>,
    fs_name_to_key: &'a HashMap<String, String>,
) -> Option<&'a str> {
    if let Some(key) = languages
        .get_key_value(samples_dir_name)
        .map(|(k, _)| k.as_str())
    {
        return Some(key);
    }
    if let Some(key) = fs_name_to_key.get(samples_dir_name) {
        return Some(key);
    }
    None
}

fn expected_filetype_for_language(lang_key: &str, entry: &LanguageEntry) -> Option<FileType> {
    use std::str::FromStr;

    fn slugify(name: &str) -> String {
        name.trim()
            .to_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }

    let mut candidates: Vec<String> = Vec::new();

    // Symbol-heavy language keys that slugify poorly.
    match lang_key.trim().to_lowercase().as_str() {
        "c++" => candidates.push("cpp".to_string()),
        "c#" => candidates.push("csharp".to_string()),
        "f#" => candidates.push("fsharp".to_string()),
        "f*" => candidates.push("fstar".to_string()),
        "objective-c" => candidates.push("objc".to_string()),
        "objective-c++" => candidates.push("objcpp".to_string()),
        _ => {}
    }

    // A few Linguist keys don't round-trip cleanly through our FileType aliases.
    match lang_key {
        "Cabal Config" => candidates.push("cabalconfig".to_string()),
        "RMarkdown" => candidates.push("rmd".to_string()),
        _ => {}
    }

    if let Some(fs_name) = &entry.fs_name {
        candidates.push(fs_name.to_lowercase());
    }

    // Match the generator's slugging (e.g. "API Blueprint" -> "api-blueprint").
    candidates.push(slugify(lang_key));

    // A couple of fallback normalizations.
    candidates.push(lang_key.to_lowercase());
    candidates.push(lang_key.to_lowercase().replace([' ', '-', '_'], ""));

    // Linguist aliases can be non-canonical (e.g. JSON includes `jsonl`, `geojson`, ...),
    // so try them only after the language key itself.
    if let Some(aliases) = &entry.aliases {
        for a in aliases {
            let a = a.trim();
            if a.is_empty() {
                continue;
            }
            candidates.push(slugify(a));
            candidates.push(a.to_lowercase());
        }
    }

    for c in candidates {
        if let Ok(ft) = FileType::from_str(&c) {
            return Some(ft);
        }
    }

    None
}

fn acceptable_filetypes(expected: FileType) -> Vec<FileType> {
    // Accept “more narrow” detections where this is effectively a subtype, and a few
    // equivalence classes where samples include overlapping/compatible syntaxes.

    const EQUIV_GROUPS: &[&[FileType]] = &[
        // Ini dialects are effectively the same family for parsing.
        &[FileType::ConfIni, FileType::DosIni],
        // Groff vs Nroff: accept either.
        &[FileType::Nroff, FileType::Groff],
    ];

    const SUBTYPES: &[(FileType, &[FileType])] = &[
        // Shell is a family; accept common dialects as “more specific”.
        (
            FileType::Sh,
            &[
                FileType::Bash,
                FileType::Zsh,
                FileType::Ksh,
                FileType::Csh,
                FileType::Tcsh,
                FileType::Fish,
                FileType::Rc,
            ],
        ),
        // HCL is effectively Terraform-family in most corpora.
        (FileType::Hcl, &[FileType::Terraform, FileType::TerraformVars]),
        // R documentation and related files are often detected as RHelp.
        (FileType::R, &[FileType::RHelp]),
        // JSONC is JSON with comments; accept plain JSON.
        (FileType::JsonC, &[FileType::Json]),
        // Sexplib is a Lisp-family S-expression syntax.
        (FileType::Lisp, &[FileType::Sexplib]),
        // Cabal config dir includes `.cabal` package files and `cabal.project`.
        (
            FileType::CabalConfig,
            &[FileType::Cabal, FileType::CabalProject],
        ),
        // Starlark/Bazel dialects.
        (FileType::Starlark, &[FileType::Bzl]),
        (FileType::Gn, &[FileType::Bzl]),
        // WebAssembly text formats.
        (FileType::Wast, &[FileType::Wat]),
        // SQL dialects.
        (
            FileType::Sql,
            &[FileType::Plsql, FileType::MySql, FileType::Cqlang],
        ),
        // GYP files are Python-ish build files (often parsed/treated as Python in corpora).
        (FileType::Python, &[FileType::Gyp]),
        // Registry: accept the more specific variant name.
        (FileType::WindowsRegistryEntries, &[FileType::Registry]),
        // ssh_config vs sshd_config.
        (FileType::SshConfig, &[FileType::SshdConfig]),
        // `tcsh` is a csh family dialect.
        (FileType::Tcsh, &[FileType::Csh]),
        // Scala ecosystem scripts / build files.
        (FileType::Scala, &[FileType::Sbt]),
        // NASM is a subtype of generic assembly.
        (FileType::Nasm, &[FileType::Asm]),
        // Arduino is a C++-family dialect; also accept C for header-only samples.
        (FileType::Cpp, &[FileType::Arduino, FileType::C]),
        // JSX is a JavaScript-family dialect.
        (FileType::JavaScript, &[FileType::Jsx]),
        // XHTML is still HTML-family.
        (FileType::Html, &[FileType::Xhtml]),
        // Godot shader language is GLSL-like.
        (FileType::Glsl, &[FileType::GdShader]),
        // Erlang lexer specs are commonly treated as Erlang-family.
        (FileType::Erlang, &[FileType::Leex]),
        // Forth dialect.
        (FileType::Forth, &[FileType::Reva]),
        // MQL dialect.
        (FileType::Mql5, &[FileType::Mql4]),
        // Quake / Modula-3 build tooling overlap in samples.
        (FileType::Quake, &[FileType::M3Build]),
        // Lockfiles are Ruby ecosystem artifacts.
        (FileType::Ruby, &[FileType::GemfileLock]),
        // SourcePawn vs Pawn: accept the narrower variant.
        (FileType::Sourcepawn, &[FileType::Pawn]),
        // Stata help is often SMCL.
        (FileType::Stata, &[FileType::Smcl]),
        // KiCad samples include non-KiCad schematics; accept other schematic types.
        (FileType::EeschemaSchematic, &[FileType::Eagle]),
    ];

    let mut v = vec![expected];

    for group in EQUIV_GROUPS {
        if group.contains(&expected) {
            v.extend_from_slice(group);
        }
    }

    for (base, subs) in SUBTYPES {
        if *base == expected {
            v.extend_from_slice(subs);
        }
    }

    v.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));
    v.dedup();
    v
}

/// Accuracy test against `./samples/` (copied from hyperpolyglot).
#[test]
fn samples_accuracy() {
    let samples_dir = Path::new("samples");
    if !samples_dir.is_dir() {
        panic!("missing ./samples directory");
    }

    let languages = read_languages_yml();
    let mut fs_name_to_key: HashMap<String, String> = HashMap::new();
    for (k, v) in languages.iter() {
        if let Some(fs_name) = &v.fs_name {
            fs_name_to_key.insert(fs_name.clone(), k.clone());
        }
    }

    let mut checked = 0usize;
    let mut correct = 0usize;
    let mut incorrect = 0usize;
    let mut undetected = 0usize;
    let mut skipped_languages = 0usize;
    let mut skipped_unmappable = 0usize;

    let mut failures: Vec<String> = Vec::new();
    let mut unmappable_languages: Vec<String> = Vec::new();

    // Optional: when present, accept detections that share the same parser/grammar.
    // This reduces taxonomy churn while keeping correctness aligned with what we can actually parse.
    let filetype_to_parser = read_filetype_to_parser_map();

    for entry in std::fs::read_dir(samples_dir).expect("read samples dir") {
        let entry = entry.expect("read samples entry");
        if !entry.file_type().expect("samples entry filetype").is_dir() {
            continue;
        }

        let dir_name = entry.file_name().to_string_lossy().into_owned();
        let lang_key = match language_key_for_samples_dir(&dir_name, &languages, &fs_name_to_key) {
            Some(k) => k,
            None => {
                skipped_languages += 1;
                continue;
            }
        };
        let lang_entry = &languages[lang_key];
        let expected = match expected_filetype_for_language(lang_key, lang_entry) {
            Some(ft) => ft,
            None => {
                skipped_unmappable += 1;
                unmappable_languages.push(lang_key.to_string());
                continue;
            }
        };
        let acceptable = acceptable_filetypes(expected);
        let expected_parsers = filetype_to_parser
            .as_ref()
            .and_then(|m| m.get(&expected))
            .map(|s| s.as_slice())
            .unwrap_or_default();

        for file in walkdir::WalkDir::new(entry.path())
            .follow_links(false)
            .into_iter()
            .filter_map(Result::ok)
        {
            if !file.file_type().is_file() {
                continue;
            }
            checked += 1;
            let path = file.path();

            let bytes = std::fs::read(path).expect("read sample file");
            let content = String::from_utf8_lossy(&bytes);
            match palate::try_detect(path, &content) {
                Some(got)
                    if acceptable.contains(&got)
                        || (!expected_parsers.is_empty()
                            && filetype_to_parser
                                .as_ref()
                                .and_then(|m| m.get(&got))
                                .is_some_and(|got_parsers| {
                                    got_parsers.iter().any(|gp| expected_parsers.contains(&gp))
                                })) =>
                {
                    correct += 1
                }
                Some(got) => {
                    incorrect += 1;
                    let got_parsers = filetype_to_parser.as_ref().and_then(|m| m.get(&got));
                    failures.push(format!(
                        "{}: expected {:?} (acceptable {:?}), got {:?} (parsers: expected={:?} got={:?})",
                        path.display(),
                        expected,
                        acceptable,
                        got,
                        expected_parsers,
                        got_parsers,
                    ));
                }
                None => {
                    undetected += 1;
                    failures.push(format!(
                        "{}: expected {:?} (acceptable {:?}), got None",
                        path.display(),
                        expected,
                        acceptable
                    ));
                }
            }
        }
    }

    let accuracy = if checked == 0 {
        0.0
    } else {
        (correct as f64) / (checked as f64)
    };

    failures.sort();
    let show = failures.len();

    unmappable_languages.sort();
    unmappable_languages.dedup();

    if skipped_languages != 0 {
        panic!("samples_accuracy: {skipped_languages} sample language directories were not found in languages.yml (first 50 shown):\n{}",
            std::fs::read_dir(samples_dir)
                .ok()
                .into_iter()
                .flat_map(|it| it.filter_map(Result::ok))
                .filter(|e| e.file_type().ok().is_some_and(|t| t.is_dir()))
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|d| language_key_for_samples_dir(d, &languages, &fs_name_to_key).is_none())
                .take(50)
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    if skipped_unmappable != 0 {
        panic!(
            "samples_accuracy: {skipped_unmappable} sample languages can't be mapped to a FileType (missing aliases/variants). First 100:\n{}",
            unmappable_languages.iter().take(100).cloned().collect::<Vec<_>>().join("\n")
        );
    }

    const MIN_ACCURACY: f64 = 1.0;
    const MAX_UNDETECTED: usize = 0;

    if accuracy < MIN_ACCURACY || undetected > MAX_UNDETECTED {
        panic!(
            "samples_accuracy FAILED: checked={checked}, correct={correct}, incorrect={incorrect}, undetected={undetected}, accuracy={accuracy:.3}\n\nshowing {show} of {} failures:\n{}",
            failures.len(),
            failures[..show].join("\n")
        );
    }
}
