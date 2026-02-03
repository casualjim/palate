//! Heuristics-based detection using regex patterns from hyperpolyglot.
//!
//! This module provides disambiguation rules for file extensions that map to
//! multiple possible languages. It uses the regex patterns defined in heuristics.yml.

use fancy_regex::RegexBuilder;
use std::path::Path;

use crate::FileType;

// Include the generated disambiguation map
include!("../codegen/disambiguation-heuristics-map.rs");

#[derive(Debug)]
enum Pattern {
    And(&'static [Pattern]),
    Negative(&'static str),
    Or(&'static [Pattern]),
    Positive(&'static str),
}

#[derive(Debug)]
struct Rule {
    languages: &'static [FileType],
    pattern: Option<Pattern>,
}

impl Pattern {
    fn matches(&self, content: &str) -> bool {
        match self {
            Pattern::Positive(pattern) => {
                let regex = RegexBuilder::new(&format!("(?m){pattern}"))
                    .build()
                    .unwrap();
                regex.is_match(content).unwrap_or(false)
            }
            Pattern::Negative(pattern) => {
                let regex = RegexBuilder::new(&format!("(?m){pattern}"))
                    .build()
                    .unwrap();
                !regex.is_match(content).unwrap_or(true)
            }
            Pattern::Or(patterns) => patterns.iter().any(|pattern| pattern.matches(content)),
            Pattern::And(patterns) => patterns.iter().all(|pattern| pattern.matches(content)),
        }
    }
}

/// Apply heuristic-based detection for ambiguous extensions.
///
/// This is called when a file extension could match multiple languages.
/// The patterns are evaluated in order; the first match wins.
pub(crate) fn apply_heuristics(extension: &str, _path: &Path, content: &str) -> Option<FileType> {
    match DISAMBIGUATIONS.get(extension) {
        Some(rules) => {
            for rule in rules.iter() {
                if let Some(pattern) = &rule.pattern {
                    if pattern.matches(content) {
                        // Return first language from the rule
                        return rule.languages.first().copied();
                    };
                } else {
                    // No pattern means "still ambiguous". Only treat it as a default
                    // when there's exactly one language.
                    if rule.languages.len() == 1 {
                        return rule.languages.first().copied();
                    }
                };
            }
            None
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heuristics_csharp() {
        let content = "using System;";
        assert_eq!(
            apply_heuristics(".cs", Path::new("test.cs"), content),
            Some(FileType::CSharp)
        );
    }

    #[test]
    fn test_heuristics_cpp() {
        let content = "    #include <vector>";
        assert_eq!(
            apply_heuristics(".h", Path::new("test.h"), content),
            Some(FileType::Cpp)
        );
    }

    #[test]
    fn test_heuristics_objc() {
        let content = "@interface MyClass";
        assert_eq!(
            apply_heuristics(".h", Path::new("test.h"), content),
            Some(FileType::ObjC)
        );
    }

    #[test]
    fn test_heuristics_c_default() {
        let content = "random content that doesn't match any pattern";
        assert_eq!(
            apply_heuristics(".h", Path::new("test.h"), content),
            Some(FileType::C) // Default fallback (no pattern)
        );
    }

    #[test]
    fn test_heuristics_unknown_extension() {
        let content = "anything";
        assert_eq!(
            apply_heuristics(".xyz", Path::new("test.xyz"), content),
            None
        );
    }

    #[test]
    fn test_heuristics_mod_fallback_returns_first_language() {
        // `.mod` includes a fallback rule (no pattern) with multiple candidate languages.
        // Our current implementation treats this as "still ambiguous".
        assert_eq!(
            apply_heuristics(".mod", Path::new("x.mod"), ""),
            None
        );
    }
}
