//! Heuristics-based detection using PCRE2 regex patterns from hyperpolyglot.
//!
//! This module provides disambiguation rules for file extensions that map to
//! multiple possible languages. It uses the regex patterns defined in heuristics.yml.

use std::path::Path;

use crate::FileType;

/// Apply heuristic-based detection for ambiguous extensions.
///
/// This is called when a file extension could match multiple languages.
/// The patterns are evaluated in order; the first match wins.
pub(crate) fn apply_heuristics(extension: &str, _path: &Path, content: &str) -> Option<FileType> {
    // For now, return None - this indicates heuristics didn't match
    // The full implementation would parse heuristics.yml and apply the rules

    // Example heuristics for common ambiguous extensions:
    match extension {
        ".h" | ".hpp" | ".hxx" => disambiguate_header(content),
        ".m" => disambiguate_m(content),
        ".fs" => disambiguate_fs(content),
        ".pl" => disambiguate_pl(content),
        ".pm" => disambiguate_pm(content),
        _ => None,
    }
}

/// Disambiguate .h files (could be C, C++, Objective-C)
fn disambiguate_header(content: &str) -> Option<FileType> {
    // Check for Objective-C keywords
    for line in content.lines().take(200) {
        if line.contains("@interface") || line.contains("@end") || line.contains("@class") {
            return Some(FileType::ObjC);
        }
    }
    // Default to C
    Some(FileType::C)
}

/// Disambiguate .m files (could be MATLAB, Objective-C, Mathematica, etc.)
fn disambiguate_m(content: &str) -> Option<FileType> {
    // MATLAB comments start with %
    for line in content.lines().take(50) {
        let trimmed = line.trim_start();
        if trimmed.starts_with('%') {
            return Some(FileType::Matlab);
        }
        if trimmed.starts_with("//") || trimmed.contains("@interface") {
            return Some(FileType::ObjC);
        }
    }
    // Default to Objective-C as it's more common
    Some(FileType::ObjC)
}

/// Disambiguate .fs files (could be F#, Forth)
fn disambiguate_fs(content: &str) -> Option<FileType> {
    // Forth uses : ( colon ) for definitions
    for line in content.lines().take(50) {
        let trimmed = line.trim_start();
        if trimmed.starts_with(':') || trimmed.starts_with('\\') {
            return Some(FileType::Forth);
        }
    }
    // Default to F#
    Some(FileType::FSharp)
}

/// Disambiguate .pl files (could be Perl, Prolog)
fn disambiguate_pl(content: &str) -> Option<FileType> {
    let first_non_empty = content.lines().find(|l| !l.trim().is_empty())?;

    // Prolog facts/rules start with lowercase and contain :-
    if first_non_empty.contains(":-") {
        return Some(FileType::Prolog);
    }

    // Perl shebang
    if first_non_empty.starts_with("#!") && first_non_empty.contains("perl") {
        return Some(FileType::Perl);
    }

    // Default to Perl
    Some(FileType::Perl)
}

/// Disambiguate .pm files (could be Perl, XPM image)
fn disambiguate_pm(content: &str) -> Option<FileType> {
    // XPM files have a specific header
    let first_line = content.lines().next()?;
    if first_line.contains("XPM") {
        return Some(FileType::Xpm);
    }
    // Default to Perl
    Some(FileType::Perl)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disambiguate_header_objc() {
        let objc = "@interface MyClass\n@end";
        assert_eq!(Some(FileType::ObjC), disambiguate_header(objc));
    }

    #[test]
    fn test_disambiguate_header_c() {
        let c = "#ifndef HEADER_H\n#define HEADER_H\n#endif";
        assert_eq!(Some(FileType::C), disambiguate_header(c));
    }

    #[test]
    fn test_disambiguate_m_matlab() {
        let matlab = "% MATLAB comment\nx = 1;";
        assert_eq!(Some(FileType::Matlab), disambiguate_m(matlab));
    }

    #[test]
    fn test_disambiguate_pl_prolog() {
        let prolog = "parent(X, Y) :- ancestor(X, Y).";
        assert_eq!(Some(FileType::Prolog), disambiguate_pl(prolog));
    }

    #[test]
    fn test_disambiguate_pl_perl() {
        let perl = "use strict;\nmy $x = 1;";
        assert_eq!(Some(FileType::Perl), disambiguate_pl(perl));
    }
}
