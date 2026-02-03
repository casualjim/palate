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
        // Examples:
        // - #!/usr/bin/env python3 -> python3
        // - #!/usr/bin/env VAR=1 sh -x -> sh
        let mut it = parts.iter().skip(1);
        let mut candidate: Option<&str> = None;
        while let Some(tok) = it.next() {
            // Skip environment assignments and env flags.
            if tok.contains('=') || tok.starts_with('-') {
                continue;
            }
            candidate = Some(tok);
            break;
        }
        candidate?
    } else {
        // #!/bin/bash -> bash
        // #!/usr/bin/perl -> perl
        parts[0].rsplit('/').next().unwrap_or(parts[0])
    };

    // Handle interpreters where trailing digits are significant, not version suffixes.
    // E.g. `perl6` / `raku` are different languages than `perl`.
    let raw_lower = interpreter.to_ascii_lowercase();
    if matches!(raw_lower.as_str(), "perl6" | "raku" | "rakudo") {
        return Some(FileType::Raku);
    }

    // Remove any version suffix for cleaner matching.
    // e.g., "python3.11" -> "python", "ruby2.7" -> "ruby"
    let base_interpreter = interpreter.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');
    let base_interpreter = base_interpreter.to_ascii_lowercase();

    let ft = interpreter_to_filetype(&base_interpreter)?;

    // Special-case: shell wrapper scripts that exec into Scala (seen in the samples suite).
    if matches!(
        ft,
        FileType::Sh | FileType::Bash | FileType::Zsh | FileType::Ksh | FileType::Csh | FileType::Tcsh
    ) && content
        .lines()
        .take(10)
        .any(|l| l.trim_start().starts_with("exec ") && l.contains("scala"))
    {
        return Some(FileType::Scala);
    }

    Some(ft)
}

/// Hardcoded interpreter mappings built from hyperpolyglot's languages.yml
/// TODO: Generate this from languages.yml in a build script
fn interpreter_to_filetype(interpreter: &str) -> Option<FileType> {
    Some(match interpreter {
        // Compilers-as-interpreters (e.g. `#!/usr/bin/tcc -run`).
        "tcc" | "cc" | "gcc" | "clang" => FileType::C,

        "bash" => FileType::Bash,
        "sh" => FileType::Sh,
        "zsh" => FileType::Zsh,
        "ksh" => FileType::Ksh,
        "csh" => FileType::Csh,
        "tcsh" => FileType::Tcsh,
        "fish" => FileType::Fish,
        "dash" => FileType::Sh,
        "rc" => FileType::Rc,
        "perl" => FileType::Perl,
        "python" => FileType::Python,
        "ruby" => FileType::Ruby,
        "macruby" => FileType::Ruby,
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
        "r" | "rscript" => FileType::R,
        "matlab" => FileType::Matlab,
        "octave" => FileType::Octave,
        "nu" | "nush" => FileType::Nu,
        "apl" => FileType::Apl,
        "jconsole" => FileType::J,
        "hy" => FileType::Hy,
        "awk" | "gawk" | "mawk" | "nawk" => FileType::Awk,
        "sed" => FileType::Sed,
        "make" | "gmake" => FileType::Make,
        "nasm" | "yasm" => FileType::Asm,
        "openrc-run" => FileType::Openrc,
        "qmake" => FileType::Qmake,
        "swipl" => FileType::Prolog,
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
