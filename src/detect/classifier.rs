//! Naive Bayes classifier for content-based language detection.
//!
//! This module provides a fallback classification method when other detection
//! methods fail. It's based on token-based analysis similar to hyperpolyglot's
//! classifier.

use std::collections::HashMap;

use crate::FileType;

/// Classify file content using naive Bayes token analysis.
///
/// This is a simplified version that uses keyword detection.
/// The full implementation would use trained log probabilities from
/// hyperpolyglot's token data.
pub(crate) fn classify(content: &str) -> Option<FileType> {
    // Tokenize content and count language-specific keywords
    let tokens = tokenize(content);

    // Score each language based on token matches
    let mut scores: HashMap<FileType, i32> = HashMap::new();

    for token in tokens {
        // Language-specific keyword scoring
        // This is a simplified version - the full classifier would use
        // pre-trained log probabilities

        // Rust indicators
        if token == "fn" || token == "let" || token == "mut" || token == "impl" {
            *scores.entry(FileType::Rust).or_insert(0) += 3;
        }

        // Python indicators
        if token == "def" || token == "class" || token == "import" || token == "from" {
            *scores.entry(FileType::Python).or_insert(0) += 2;
        }

        // JavaScript/TypeScript indicators
        if token == "const" || token == "let" || token == "=> " || token == "function" {
            *scores.entry(FileType::JavaScript).or_insert(0) += 2;
        }

        // Go indicators
        if token == "func" || token == "var" || token == "type" || token == "struct" {
            *scores.entry(FileType::Go).or_insert(0) += 2;
        }

        // Java indicators
        if token == "public" || token == "private" || token == "class" || token == "interface" {
            *scores.entry(FileType::Java).or_insert(0) += 2;
        }

        // C/C++ indicators
        if token == "printf" || token == "scanf" || token == "malloc" {
            *scores.entry(FileType::C).or_insert(0) += 2;
        }

        // Ruby indicators
        if token == "end" || token == "require" || token == "module" {
            *scores.entry(FileType::Ruby).or_insert(0) += 2;
        }
    }

    // Return the language with the highest score.
    //
    // Tie-break deterministically so results don't depend on HashMap iteration order.
    let mut best: Option<(FileType, i32)> = None;
    for (ft, score) in scores.into_iter() {
        match best {
            None => best = Some((ft, score)),
            Some((best_ft, best_score)) => {
                let ft_name: &'static str = ft.into();
                let best_name: &'static str = best_ft.into();
                let choose = score > best_score
                    || (score == best_score && ft_name < best_name);
                if choose {
                    best = Some((ft, score));
                }
            }
        }
    }
    best.map(|(ft, _)| ft)
}

/// Simple tokenizer that extracts identifiers and keywords.
fn tokenize(content: &str) -> Vec<&str> {
    // Split on whitespace and punctuation
    content
        .split(|c: char| {
            c.is_whitespace()
                || c == '('
                || c == ')'
                || c == '{'
                || c == '}'
                || c == '['
                || c == ']'
                || c == ';'
                || c == ','
                || c == '.'
                || c == '='
                || c == ':'
                || c == '"'
        })
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_rust() {
        let rust_code = "fn main() {\n    let x = 42;\n    println!(\"{}\");\n}";
        assert_eq!(Some(FileType::Rust), classify(rust_code));
    }

    #[test]
    fn test_classify_python() {
        let python_code = "def main():\n    import os\n    class MyClass:\n        pass";
        assert_eq!(Some(FileType::Python), classify(python_code));
    }

    #[test]
    fn test_classify_empty() {
        assert_eq!(None, classify(""));
    }
}
