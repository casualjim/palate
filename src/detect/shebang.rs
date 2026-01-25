use crate::FileType;

/// Detect file type from shebang line.
///
/// Parses `#!` lines from file content and maps to languages using interpreters map.
pub(crate) fn detect_from_shebang(content: &str) -> Option<FileType> {
    let first_line = content.lines().next()?;

    // Check for shebang
    if !first_line.starts_with("#!") {
        return None;
    }

    // Extract the interpreter part (everything after #!)
    let shebang_content = &first_line[2..];

    // Parse the shebang to get the interpreter name
    // Handle cases like:
    // - #!/bin/bash
    // - #!/usr/bin/env python3
    // - #!/usr/bin/perl -w
    let parts: Vec<&str> = shebang_content.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    // Get the interpreter path or name
    let interpreter = if parts[0].ends_with("env") && parts.len() > 1 {
        // #!/usr/bin/env python3 -> python3
        parts[1]
    } else {
        // #!/bin/bash -> bash
        // #!/usr/bin/perl -> perl
        parts[0].rsplit('/').next().unwrap_or(parts[0])
    };

    // Remove any version suffix for cleaner matching
    // e.g., "python3.11" -> "python", "ruby2.7" -> "ruby"
    let base_interpreter = interpreter.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');

    interpreter_to_filetype(base_interpreter)
}

/// Hardcoded interpreter mappings built from hyperpolyglot's languages.yml
/// TODO: Generate this from languages.yml in a build script
fn interpreter_to_filetype(interpreter: &str) -> Option<FileType> {
    Some(match interpreter {
        "bash" => FileType::Bash,
        "sh" => FileType::Sh,
        "zsh" => FileType::Zsh,
        "ksh" => FileType::Ksh,
        "csh" => FileType::Csh,
        "tcsh" => FileType::Tcsh,
        "fish" => FileType::Fish,
        "dash" => FileType::Sh,
        "perl" => FileType::Perl,
        "python" => FileType::Python,
        "ruby" => FileType::Ruby,
        "php" => FileType::Php,
        "node" => FileType::JavaScript,
        "ts-node" | "deno" | "bun" => FileType::TypeScript,
        "tclsh" | "wish" => FileType::Tcl,
        "lua" => FileType::Lua,
        "guile" => FileType::Lisp,
        "racket" => FileType::Racket,
        "scheme" => FileType::Scheme,
        "sbcl" | "clisp" => FileType::Lisp,
        "elixir" => FileType::Elixir,
        "erlang" | "escript" => FileType::Erlang,
        "groovy" => FileType::Groovy,
        "java" => FileType::Java,
        "kotlin" => FileType::Kotlin,
        "scala" => FileType::Scala,
        "clojure" => FileType::Clojure,
        "ocaml" | "ocamlrun" => FileType::OCaml,
        "swift" => FileType::Swift,
        "julia" => FileType::Julia,
        "R" | "rscript" => FileType::R,
        "matlab" => FileType::Matlab,
        "octave" => FileType::Octave,
        "awk" | "gawk" | "mawk" | "nawk" => FileType::Awk,
        "sed" => FileType::Sed,
        "make" | "gmake" => FileType::Make,
        "nasm" | "yasm" => FileType::Asm,
        "pike" => FileType::Pike,
        "bc" => FileType::Bc,
        "dc" => FileType::D,
        "icon" => FileType::Icon,
        "rexx" | "regina" => FileType::Rexx,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_shebang() {
        assert_eq!(
            Some(FileType::Bash),
            detect_from_shebang("#!/bin/bash\necho hello")
        );
        assert_eq!(
            Some(FileType::Python),
            detect_from_shebang("#!/usr/bin/python3\nprint('hello')")
        );
        assert_eq!(Some(FileType::Sh), detect_from_shebang("#!/bin/sh\n"));
    }

    #[test]
    fn test_env_shebang() {
        assert_eq!(
            Some(FileType::Python),
            detect_from_shebang("#!/usr/bin/env python3\n")
        );
        assert_eq!(
            Some(FileType::Perl),
            detect_from_shebang("#!/usr/bin/env perl\n")
        );
    }

    #[test]
    fn test_no_shebang() {
        assert_eq!(None, detect_from_shebang("echo hello"));
        assert_eq!(None, detect_from_shebang(""));
    }
}
