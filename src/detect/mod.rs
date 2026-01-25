use std::path::Path;

use aho_corasick::AhoCorasick;
use lazy_regex::{lazy_regex, regex, regex_is_match};
use once_cell::sync::Lazy;
use regex::Regex;

use self::{
    file_extension::FILE_EXTENSION, filename::FILENAME, path_suffix::PATH_SUFFIX, pattern::PATTERN,
    shebang::detect_from_shebang, util::get_lines,
};
use crate::FileType;

mod classifier;
mod file_extension;
mod filename;
mod heuristics;
mod path_suffix;
mod pattern;
mod shebang;
mod util;

/// Same as [`try_detect`] but automatically falling back to [`FileType::Text`] where
/// [`try_detect`] would return [`None`].
///
/// # Example
/// ```
/// use palate::FileType;
///
/// assert_eq!(FileType::Rust, palate::detect("main.rs", ""));
/// assert_eq!(FileType::Text, palate::detect("test.txt", ""));
/// assert_eq!(FileType::Text, palate::detect("unsupported.filetype", ""));
/// ```
pub fn detect(path: impl AsRef<Path>, content: &str) -> FileType {
    try_detect(path, content).unwrap_or(FileType::Text)
}

/// Try to detect a [`FileType`] given a file's path and content.
///
/// # Example
/// ```
/// use palate::FileType;
///
/// assert_eq!(Some(FileType::Rust), palate::try_detect("main.rs", ""));
/// assert_eq!(Some(FileType::Text), palate::try_detect("test.txt", ""));
/// assert_eq!(None, palate::try_detect("unsupported.filetype", ""));
/// ```
pub fn try_detect(path: impl AsRef<Path>, content: &str) -> Option<FileType> {
    let path = path.as_ref();

    // path suffix
    for (suffix, resolver) in PATH_SUFFIX {
        if path.ends_with(suffix) {
            if let Some(ft) = resolver.resolve(path, content) {
                return Some(ft);
            }
        }
    }

    // shebang detection (from hyperpolyglot)
    if let Some(ft) = detect_from_shebang(content) {
        return Some(ft);
    }

    // filename
    if let Some(resolver) = path
        .file_name()
        .and_then(|os_name| os_name.to_str())
        .and_then(|filename| FILENAME.get(filename))
    {
        if let Some(ft) = resolver.resolve(path, content) {
            return Some(ft);
        }
    }

    // patterns (non-negative priority)
    let mut negative_prio_start_index = 0;
    for (index, (match_full_path, regex, pat)) in PATTERN.iter().enumerate() {
        if pat.priority.is_some_and(|prio| prio < 0) {
            negative_prio_start_index = index;
            break;
        }
        if match match_full_path {
            true => path.to_str(),
            false => path.file_name().and_then(|os_name| os_name.to_str()),
        }
        .is_none_or(|haystack| !regex.is_match(haystack))
        {
            continue;
        }
        if let Some(ft) = pat.resolver.resolve(path, content) {
            return Some(ft);
        }
    }

    // file extension
    if let Some(resolver) = path
        .extension()
        .and_then(|os_ext| os_ext.to_str())
        .and_then(|ext| FILE_EXTENSION.get(ext))
    {
        if let Some(ft) = resolver.resolve(path, content) {
            return Some(ft);
        }
    }

    // patterns (negative priority)
    for (match_full_path, regex, pat) in PATTERN.iter().skip(negative_prio_start_index) {
        if match match_full_path {
            true => path.to_str(),
            false => path.file_name().and_then(|os_name| os_name.to_str()),
        }
        .is_none_or(|haystack| !regex.is_match(haystack))
        {
            continue;
        }
        if let Some(ft) = pat.resolver.resolve(path, content) {
            return Some(ft);
        }
    }

    // file contents - fallback to heuristics and classifier
    // Try heuristics for ambiguous extensions
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if let Some(ft) = heuristics::apply_heuristics(ext, path, content) {
            return Some(ft);
        }
    }

    // Final fallback: naive Bayes classifier
    if let Some(ft) = classifier::classify(content) {
        return Some(ft);
    }

    None
}

// Include all dynamic resolver functions from tft's detect.rs
// These are kept as module-private functions used by the PHF maps

pub(crate) fn asa(_path: &Path, _content: &str) -> Option<FileType> {
    // TODO: user defined preferred asa filetype
    Some(FileType::AspVbs)
}

pub(crate) fn asm(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred asm syntax
    match util::findany(
        content,
        10,
        true,
        [".title", ".ident", ".macro", ".subtitle", ".library"],
    ) {
        true => Some(FileType::Vmasm),
        false => Some(FileType::Asm),
    }
}

pub(crate) fn asp(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred asp filetype
    match util::find(content, 3, false, "perlscript") {
        true => Some(FileType::AspPerl),
        false => Some(FileType::AspVbs),
    }
}

pub(crate) fn bak(path: &Path, content: &str) -> Option<FileType> {
    // for files like `main.rs.bak` retry search without the `.bak` extension
    try_detect(path.with_extension(""), content)
}

pub(crate) fn bas(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred bas filetype
    // Most frequent FreeBASIC-specific keywords in distro files
    let fb_keywords = regex!(
        r"^\s*(extern|var|enum|private|scope|union|byref|operator|constructor|delete|namespace|public|property|with|destructor|using)\b(\s*[:=(])\@!"i
    );
    let fb_preproc = regex!(
        r"^\s*(#\s*\a+|option\s+(byval|dynamic|escape|(no)?gosub|nokeyword|private|static)\b|(''|rem)\s*\$lang\b|def(byte|longint|short|ubyte|uint|ulongint|ushort)\b)"i
    );

    let fb_comment = regex!(r"^\s*/'");
    // OPTION EXPLICIT, without the leading underscore, is common to many dialects
    let qb64_preproc = regex!(r"^\s*($\a+|option\s+(_explicit|_?explicitarray)\b)"i);

    for line in content.lines().take(100) {
        if util::findany(
            line,
            0,
            false,
            ["BEGIN VB.Form", "BEGIN VB.MDIForm", "BEGIN VB.UserControl"],
        ) {
            return Some(FileType::Vb);
        } else if fb_comment.is_match(line)
            || fb_preproc.is_match(line)
            || fb_keywords.is_match(line)
        {
            return Some(FileType::FreeBasic);
        } else if qb64_preproc.is_match(line) {
            return Some(FileType::Qb64);
        }
    }
    Some(FileType::Basic)
}

pub(crate) fn bindzone(content: &str, default: Option<FileType>) -> Option<FileType> {
    match regex_is_match!(
        r"^; <<>> DiG [0-9\.]+.* <<>>|\$ORIGIN|\$TTL|IN\s+SOA",
        get_lines(content, 4)
    ) {
        true => Some(FileType::Bindzone),
        false => default,
    }
}

pub(crate) fn btm(_path: &Path, _content: &str) -> Option<FileType> {
    // TODO: user defined dosbatch for btm
    Some(FileType::Btm)
}

pub(crate) fn cfg(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred cfg filetype
    match regex_is_match!(r"(eio|mmc|moc|proc|sio|sys):cfg"i, get_lines(content, 1)) {
        true => Some(FileType::Rapid),
        false => Some(FileType::Cfg),
    }
}

pub(crate) fn change(_path: &Path, content: &str) -> Option<FileType> {
    if regex_is_match!(r"^(#|!)", get_lines(content, 1)) {
        return Some(FileType::Ch);
    }
    for line in content.lines().take(10) {
        if line.starts_with('@') {
            return Some(FileType::Change);
        }
        if util::find(line, 0, true, "MODULE") {
            return Some(FileType::Chill);
        }
        if regex_is_match!(r"main\s*\(|#\s*include|//"i, line) {
            return Some(FileType::Ch);
        }
    }
    Some(FileType::Chill)
}

pub(crate) fn changelog(_path: &Path, content: &str) -> Option<FileType> {
    match util::find(content, 1, false, "; urgency=") {
        true => Some(FileType::DebChangelog),
        false => Some(FileType::Changelog),
    }
}

pub(crate) fn cls(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred cls filetype
    let first_line = get_lines(content, 1);
    if regex_is_match!(r"^[%\\]", first_line) {
        Some(FileType::Tex)
    } else if first_line.starts_with('#')
        && AhoCorasick::builder()
            .ascii_case_insensitive(true)
            .build(["rexx"])
            .unwrap()
            .is_match(first_line)
    {
        Some(FileType::Rexx)
    } else if first_line == "VERSION 1.0 CLASS" {
        Some(FileType::Vb)
    } else {
        Some(FileType::St)
    }
}

pub(crate) fn cmd(_path: &Path, content: &str) -> Option<FileType> {
    match content.starts_with("/*") {
        true => Some(FileType::Rexx),
        false => Some(FileType::DosBatch),
    }
}

pub(crate) fn control(_path: &Path, content: &str) -> Option<FileType> {
    match content.starts_with("Source:") {
        true => Some(FileType::DebControl),
        false => None,
    }
}

pub(crate) fn copyright(_path: &Path, content: &str) -> Option<FileType> {
    match content.starts_with("Format:") {
        true => Some(FileType::DebCopyright),
        false => None,
    }
}

pub(crate) fn cpp(_path: &Path, _content: &str) -> Option<FileType> {
    // TODO: user defined cynlib for cpp
    Some(FileType::Cpp)
}

pub(crate) fn cpy(_path: &Path, content: &str) -> Option<FileType> {
    match content.starts_with("##") {
        true => Some(FileType::Python),
        false => Some(FileType::Cobol),
    }
}

pub(crate) fn csh(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred csh filetype
    // TODO: user defined preferred shell filetype
    shell(content, FileType::Csh)
}

pub(crate) fn dat(path: &Path, content: &str) -> Option<FileType> {
    if path
        .file_name()
        .and_then(|os_name| os_name.to_str())
        .is_some_and(|name| regex_is_match!(r"^((.*\.)?upstream\.dat|upstream\..*\.dat)$"i, name))
    {
        return Some(FileType::UpstreamDat);
    }
    // TODO: user defined preferred dat filetype
    match util::next_non_blank(content, 0)
        .is_some_and(|line| regex_is_match!(r"^\s*(&\w+|defdat\b)"i, line))
    {
        true => Some(FileType::Krl),
        false => None,
    }
}

pub(crate) fn decl(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(3) {
        if regex_is_match!(r"^<!sgml"i, line) {
            return Some(FileType::SgmlDecl);
        }
    }
    None
}

pub(crate) fn dep3patch(path: &Path, content: &str) -> Option<FileType> {
    let filename = path.file_name()?.to_str()?;
    if filename == "series" {
        return None;
    }
    for line in content.lines().take(100) {
        if util::starts_with_any(
            line,
            true,
            [
                "Description:",
                "Subject:",
                "Origin:",
                "Bug:",
                "Forwarded:",
                "Author:",
                "From:",
                "Reviewed-by:",
                "Acked-by:",
                "Last-Updated:",
                "Applied-Upstream:",
            ],
        ) {
            return Some(FileType::Dep3Patch);
        } else if line.starts_with("---") {
            // end of headers found. stop processing
            return None;
        }
    }
    None
}

pub(crate) fn dsl(_path: &Path, content: &str) -> Option<FileType> {
    match regex_is_match!(r"^\s*<!", get_lines(content, 1)) {
        true => Some(FileType::Dsl),
        false => Some(FileType::Structurizr),
    }
}

pub(crate) fn dtrace(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(100) {
        if regex_is_match!(r"^(module|import)\b"i, line) {
            return Some(FileType::D);
        } else if regex_is_match!(r"'^#!\S+dtrace|#pragma\s+D\s+option|:\S-:\S-:", line) {
            return Some(FileType::DTrace);
        }
    }
    Some(FileType::D)
}

pub(crate) fn e(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred euphoria filetype
    for line in content.lines().take(100) {
        if regex_is_match!(r"^\s*<'\s*$|^\s*'>\s*$", line) {
            return Some(FileType::SpecMan);
        }
    }
    Some(FileType::Eiffel)
}

pub(crate) fn edn(_path: &Path, content: &str) -> Option<FileType> {
    match regex_is_match!(r"^\s*\(\s*edif\b"i, get_lines(content, 1)) {
        true => Some(FileType::Edif),
        false => Some(FileType::Clojure),
    }
}

pub(crate) fn ent(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(5) {
        if regex_is_match!(r"^\s*[#{]", line) {
            return Some(FileType::Cl);
        } else if !line.trim_start().is_empty() {
            // not a blank line, not a comment, and not a block start,
            // so doesn't look like valid cl code
            break;
        }
    }
    Some(FileType::Dtd)
}

pub(crate) fn euphoria(_path: &Path, _content: &str) -> Option<FileType> {
    // TODO: user defined preferred euphoria filetype
    Some(FileType::Euphoria3)
}

pub(crate) fn ex(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred euphoria filetype
    for line in content.lines().take(100) {
        if regex_is_match!(r"^(--|ifdef\b|include\b)", line) {
            return Some(FileType::Euphoria3);
        }
    }
    Some(FileType::Elixir)
}

pub(crate) fn foam(_path: &Path, content: &str) -> Option<FileType> {
    let mut foam_file = false;
    for line in content.lines().take(15) {
        if line.contains("FoamFile") {
            foam_file = true;
        } else if foam_file && line.trim_start().starts_with("object") {
            return Some(FileType::Foam);
        }
    }
    None
}

pub(crate) fn frm(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred frm filetype
    match util::findany(content, 5, false, ["BEGIN VB.Form", "BEGIN VB.MDIForm"]) {
        true => Some(FileType::Vb),
        false => Some(FileType::Form),
    }
}

pub(crate) fn fs(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred fs filetype
    for line in content.lines().take(100) {
        if line.starts_with([':', '(', '\\']) {
            return Some(FileType::Forth);
        }
    }
    Some(FileType::FSharp)
}

pub(crate) fn fvwm(path: &Path, _content: &str) -> Option<FileType> {
    match path.extension().is_some_and(|ext| ext == "m4") {
        true => Some(FileType::Fvwm2M4),
        false => Some(FileType::Fvwm2),
    }
}

pub(crate) fn git(_path: &Path, content: &str) -> Option<FileType> {
    match regex_is_match!(r"^[a-fA-F0-9]{40,}\b|^ref: ", get_lines(content, 1)) {
        true => Some(FileType::Git),
        false => None,
    }
}

pub(crate) fn header(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(200) {
        if regex_is_match!(r"^@(interface|end|class)"i, line) {
            // TODO: allow setting C or C++
            return Some(FileType::ObjC);
        }
    }
    // TODO: user defined preferred header filetype
    Some(FileType::C)
}

pub(crate) fn hook(_path: &Path, content: &str) -> Option<FileType> {
    match get_lines(content, 1) == "[Trigger]" {
        true => Some(FileType::Conf),
        false => None,
    }
}

pub(crate) fn html(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(10) {
        if regex_is_match!(r"\bDTD\s+XHTML\s", line) {
            return Some(FileType::Xhtml);
        } else if regex_is_match!(r"\{%\s*(extends|block|load)\b|\{#\s+"i, line) {
            return Some(FileType::HtmlDjango);
        }
    }
    Some(FileType::Html)
}

pub(crate) fn hw(_path: &Path, content: &str) -> Option<FileType> {
    match util::find(content, 1, false, "<?php") {
        true => Some(FileType::Php),
        false => Some(FileType::Virata),
    }
}

pub(crate) fn idl(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(50) {
        if regex_is_match!(r#"^\s*import\s+"(unknwn|objidl)"\.idl"#i, line) {
            return Some(FileType::Msidl);
        }
    }
    Some(FileType::Idl)
}

pub(crate) fn in_(path: &Path, content: &str) -> Option<FileType> {
    if path.file_name().is_some_and(|name| name == "configure.in") {
        return bak(path, content);
    }
    None
}

static PASCAL_KEYWORDS: Lazy<Regex> =
    lazy_regex!(r"^\s*(program|unit|library|uses|begin|procedure|function|const|type|var)\b"i);
static PASCAL_COMMENTS: Lazy<Regex> = lazy_regex!(r"^\s*(\{|\(\*|//)");

pub(crate) fn inc(path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred inc filetype
    let lines = get_lines(content, 3);
    if util::find(lines, 0, false, "perlscript") {
        Some(FileType::AspPerl)
    } else if util::find(lines, 0, false, "<%") {
        Some(FileType::AspVbs)
    } else if util::find(lines, 0, false, "<?") {
        Some(FileType::Php)
    } else if regex_is_match!(r"^\s(\{|\(\*)"i, lines) || PASCAL_KEYWORDS.is_match(lines) {
        Some(FileType::Pascal)
    } else if regex_is_match!(
        // TODO: is this regex correct?
        r"^\s*(inherit|require|[A-Z][\w_:${}]*\s+\??[?:+]?=) "i,
        lines
    ) {
        Some(FileType::Bitbake)
    } else if let Some(ft) = asm(path, content) {
        match ft {
            FileType::Asm => Some(FileType::Pov),
            _ => Some(ft),
        }
    } else {
        Some(FileType::Pov)
    }
}

pub(crate) fn inp(_path: &Path, content: &str) -> Option<FileType> {
    if content.starts_with('*') {
        return Some(FileType::Abaqus);
    }
    for line in content.lines().take(500) {
        if util::starts_with_any(line, false, ["header surface data"]) {
            return Some(FileType::Trasys);
        }
    }
    None
}

pub(crate) fn install(_path: &Path, content: &str) -> Option<FileType> {
    match util::find(content, 1, false, "<?php") {
        true => Some(FileType::Php),
        false => sh(content, Some(FileType::Bash)),
    }
}

pub(crate) fn log(path: &Path, _content: &str) -> Option<FileType> {
    let path = path.to_str();
    if path.is_some_and(|path| regex_is_match!(r"upstream([.-].*)?\.log|.*\.upstream\.log"i, path))
    {
        Some(FileType::UpstreamLog)
    } else if path.is_some_and(|path| {
        regex_is_match!(
            r"upstreaminstall(\..*)?\.log|.*\.upstreaminstall\.log"i,
            path
        )
    }) {
        Some(FileType::UpstreamInstallLog)
    } else if path
        .is_some_and(|path| regex_is_match!(r"usserver(\..*)?\.log|.*\.usserver\.log"i, path))
    {
        Some(FileType::UsServerLog)
    } else if path
        .is_some_and(|path| regex_is_match!(r"usw2kagtlog(\..*)?\.log|.*\.usw2kagtlog\.log"i, path))
    {
        Some(FileType::Usw2KagtLog)
    } else {
        None
    }
}

pub(crate) fn lpc(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined allow lpc
    for line in content.lines().take(12) {
        if util::starts_with_any(
            line,
            true,
            [
                "//",
                "inherit",
                "private",
                "protected",
                "nosave",
                "string",
                "object",
                "mapping",
                "mixed",
            ],
        ) {
            return Some(FileType::Lpc);
        }
    }
    Some(FileType::C)
}

pub(crate) fn lsl(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred lsl filetype
    match util::next_non_blank(content, 0)
        .is_some_and(|line| regex_is_match!(r"^\s*%|:\s*trait\s*$", line))
    {
        true => Some(FileType::Larch),
        false => Some(FileType::Lsl),
    }
}

pub(crate) fn m(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred m filetype
    let octave_block_terminators = regex!(
        r"(^|;)\s*\bend(_try_catch|classdef|enumeration|events|methods|parfor|properties)\b"i
    );
    let objc_preprocessor =
        regex!(r"^\s*#\s*(import|include|define|if|ifn?def|undef|line|error|pragma)\b"i);

    let mut saw_comment = false;
    for line in content.lines().take(100) {
        let trimmed_line = line.trim_start();
        if trimmed_line.starts_with("/*") {
            // /* ... */ is a comment in Objective C and Murphi, so we can't conclude
            // it's either of them yet, but track this as a hint in case we don't see
            // anything more definitive.
            saw_comment = true;
        }
        if trimmed_line.starts_with("//")
            || util::starts_with_any(trimmed_line, false, ["@import"])
            || objc_preprocessor.is_match(line)
        {
            return Some(FileType::ObjC);
        } else if util::starts_with_any(trimmed_line, false, ["#", "%%!", "unwind_protect"])
            || octave_block_terminators.is_match(line)
        {
            return Some(FileType::Octave);
        } else if trimmed_line.starts_with("%%") {
            return Some(FileType::Matlab);
        } else if trimmed_line.starts_with("(*") {
            return Some(FileType::Mma);
        } else if regex_is_match!(r"^\s*((type|var)\b|--)"i, line) {
            return Some(FileType::Murphi);
        }
    }

    match saw_comment {
        // We didn't see anything definitive, but this looks like either Objective C
        // or Murphi based on the comment leader. Assume the former as it is more
        // common.
        true => Some(FileType::ObjC),
        // default is Matlab
        false => Some(FileType::Matlab),
    }
}

pub(crate) fn m4_ext(path: &Path, _content: &str) -> Option<FileType> {
    match !path.to_str().is_some_and(|p| p.ends_with("html.m4"))
        && !path.to_str().is_some_and(|p| p.contains("fvwm2rc"))
    {
        true => Some(FileType::M4),
        false => None,
    }
}

pub(crate) fn mc(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(20) {
        let trimmed_line = line.trim_start();
        if util::starts_with_any(trimmed_line, false, ["#", "dnl"]) {
            return Some(FileType::M4);
        } else if trimmed_line.starts_with(';') {
            return Some(FileType::MsMessages);
        }
    }
    Some(FileType::M4)
}

pub(crate) fn me(path: &Path, _content: &str) -> Option<FileType> {
    match path.file_name().is_some_and(|name| {
        name.eq_ignore_ascii_case("read.me") || name.eq_ignore_ascii_case("click.me")
    }) {
        true => None,
        false => Some(FileType::Nroff),
    }
}

pub(crate) fn mm(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(20) {
        if regex_is_match!(r"^\s*(#\s*(include|import)\b|@import\b|/\*)"i, line) {
            return Some(FileType::ObjCpp);
        }
    }
    Some(FileType::Nroff)
}

pub(crate) fn mms(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(20) {
        let trimmed_line = line.trim_start();
        if util::starts_with_any(trimmed_line, true, ["%", "//", "*"]) {
            return Some(FileType::Mmix);
        } else if trimmed_line.starts_with('#') {
            return Some(FileType::Make);
        }
    }
    Some(FileType::Mmix)
}

pub(crate) fn is_lprolog(content: &str) -> bool {
    for line in content.lines().take(500) {
        let trimmed_line = line.trim_start();
        if !trimmed_line.is_empty() && !trimmed_line.starts_with('%') {
            return regex_is_match!(r"\bmodule\s+\w+\s*\.\s*(%|$)"i, line);
        }
    }
    false
}

pub(crate) fn is_rapid(content: &str) -> bool {
    util::next_non_blank(content, 0)
        .is_some_and(|line| regex_is_match!(r"^\s*(%{3}|module\s+\w+\s*(\(|$))"i, line))
}

pub(crate) fn mod_(path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred mod filetype
    if path
        .file_name()
        .is_some_and(|name| name.eq_ignore_ascii_case("go.mod"))
    {
        Some(FileType::GoMod)
    } else if is_lprolog(content) {
        Some(FileType::LambdaProlog)
    } else if util::next_non_blank(content, 0)
        .is_some_and(|line| regex_is_match!(r"(\bMODULE\s+\w+\s*;|^\s*\(\*)", line))
    {
        Some(FileType::Modula2)
    } else if is_rapid(content) {
        Some(FileType::Rapid)
    } else {
        Some(FileType::Modsim3)
    }
}

pub(crate) fn news(_path: &Path, content: &str) -> Option<FileType> {
    match util::find(content, 1, false, "; urgency=") {
        true => Some(FileType::DebChangelog),
        false => None,
    }
}

pub(crate) fn nroff(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(5) {
        if line.starts_with('.') {
            return Some(FileType::Nroff);
        }
    }
    None
}

pub(crate) fn patch(_path: &Path, content: &str) -> Option<FileType> {
    match regex_is_match!(
        r"^From [a-fA-F0-9]{40}+ Mon Sep 17 00:00:00 2001$",
        get_lines(content, 1)
    ) {
        true => Some(FileType::GitSendEmail),
        false => Some(FileType::Diff),
    }
}

pub(crate) fn perl(path: &Path, content: &str) -> Option<FileType> {
    match (path.extension().is_some_and(|ext| ext == "t")
        && path
            .parent()
            .and_then(|p| p.file_name())
            .is_some_and(|dir| dir == "t" || dir == "xt"))
        || (content.starts_with('#') && util::find(content, 1, false, "perl"))
        || content
            .lines()
            .take(30)
            .any(|line| util::starts_with_any(line.trim_start(), false, ["use"]))
    {
        true => Some(FileType::Perl),
        false => None,
    }
}

pub(crate) fn pl(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred pl filetype
    match util::next_non_blank(content, 0)
        .is_some_and(|line| regex_is_match!(r":-|\bprolog\b|^\s*(%+(\s|$)|/\*)"i, line))
    {
        true => Some(FileType::Prolog),
        false => Some(FileType::Perl),
    }
}

pub(crate) fn pm(_path: &Path, content: &str) -> Option<FileType> {
    let line = get_lines(content, 1);
    if line.contains("XPM2") {
        Some(FileType::Xpm2)
    } else if line.contains("XPM") {
        Some(FileType::Xpm)
    } else {
        Some(FileType::Perl)
    }
}

pub(crate) fn pp(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred pp filetype
    match util::next_non_blank(content, 0)
        .is_some_and(|line| PASCAL_COMMENTS.is_match(line) || PASCAL_KEYWORDS.is_match(line))
    {
        true => Some(FileType::Pascal),
        false => Some(FileType::Puppet),
    }
}

pub(crate) fn prg(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred prg filetype
    match is_rapid(content) {
        true => Some(FileType::Rapid),
        false => Some(FileType::Clipper),
    }
}

pub(crate) fn progress_asm(path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred i filetype
    for line in content.lines().take(10) {
        let trimmed_line = line.trim_start();
        if trimmed_line.starts_with(';') {
            return asm(path, content);
        } else if trimmed_line.starts_with("/*") {
            break;
        }
    }
    Some(FileType::Progress)
}

pub(crate) fn progress_cweb(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred w filetype
    match util::starts_with_any(content, false, ["&analyze"])
        || content
            .lines()
            .take(3)
            .any(|line| util::starts_with_any(line, false, ["&global-define"]))
    {
        true => Some(FileType::Progress),
        false => Some(FileType::Cweb),
    }
}

pub(crate) fn progress_pascal(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred p filetype
    for line in content.lines().take(10) {
        if PASCAL_COMMENTS.is_match(line) || PASCAL_KEYWORDS.is_match(line) {
            return Some(FileType::Pascal);
        } else if line.trim_start().starts_with("/*") {
            break;
        }
    }
    Some(FileType::Progress)
}

pub(crate) fn proto(content: &str, default: FileType) -> Option<FileType> {
    // Cproto files have a comment in the first line and a function prototype in
    // the second line, it always ends in `;`. Indent files may also have
    // comments, thus we can't match comments to see the difference.
    // IDL files can have a single `;` in the second line, require at least one
    // character before the `;`.
    if regex_is_match!(r".;$", get_lines(content, 2)) {
        // second line ends with `;`
        return Some(FileType::Cpp);
    }
    // recognize Prolog by specific text in the first non-empty line;
    // require a blank after the `%` because Perl uses `%list` and `%translate`
    match util::next_non_blank(content, 0)
        .is_some_and(|line| regex_is_match!(r":-|\bprolog\b|^\s*(%+(\s|$)|/\*)"i, line))
    {
        true => Some(FileType::Prolog),
        false => Some(default),
    }
}

pub(crate) fn psf(_path: &Path, content: &str) -> Option<FileType> {
    let trimmed_line = get_lines(content, 1).trim();
    match [
        "distribution",
        "installed_software",
        "root",
        "bundle",
        "product",
    ]
    .into_iter()
    .any(|pat| trimmed_line.eq_ignore_ascii_case(pat))
    {
        true => Some(FileType::Psf),
        false => None,
    }
}

pub(crate) fn r(_path: &Path, content: &str) -> Option<FileType> {
    // Rebol is easy to recognize, check for that first
    if regex_is_match!(r"\brebol\b"i, get_lines(content, 50)) {
        return Some(FileType::Rebol);
    }

    for line in content.lines().take(50) {
        let trimmed_line = line.trim_start();
        // R has # comments
        if trimmed_line.starts_with('#') {
            return Some(FileType::R);
        }
        // Rexx has /* comments */
        if trimmed_line.starts_with("/*") {
            return Some(FileType::Rexx);
        }
    }
    // TODO: user defined preferred r filetype
    Some(FileType::R)
}

pub(crate) fn rc(path: &Path, _content: &str) -> Option<FileType> {
    match path
        .to_str()
        .is_some_and(|str| str.contains("/etc/Muttrc.d/"))
    {
        true => None,
        false => Some(FileType::Rc),
    }
}

pub(crate) fn redif(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(5) {
        if util::starts_with_any(line, false, ["template-type:"]) {
            return Some(FileType::Redif);
        }
    }
    None
}

pub(crate) fn reg(_path: &Path, content: &str) -> Option<FileType> {
    match regex_is_match!(
        r"^regedit[0-9]*\s*$|^windows registry editor version \d*\.\d*\s*$"i,
        get_lines(content, 1)
    ) {
        true => Some(FileType::Registry),
        false => None,
    }
}

pub(crate) fn rul(_path: &Path, content: &str) -> Option<FileType> {
    match util::find(content, 6, false, "installshield") {
        true => Some(FileType::InstallShield),
        false => Some(FileType::Diva),
    }
}

pub(crate) fn rules(path: &Path, _content: &str) -> Option<FileType> {
    let utf8_path = path.to_str();
    if utf8_path
        .is_some_and(|p| regex_is_match!(r"/(etc|(usr/)?lib)/udev/(rules\.d/)?.*\.rules$"i, p))
    {
        Some(FileType::UdevRules)
    } else if path.starts_with("/etc/ufw") {
        Some(FileType::Conf)
    } else if utf8_path.is_some_and(|p| regex_is_match!(r"/(etc|usr/share)/polkit-1/rules\.d/"i, p))
    {
        Some(FileType::JavaScript)
    } else {
        // TODO: maybe try to read udev.conf for other paths
        Some(FileType::Hog)
    }
}

pub(crate) fn sc(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(25) {
        if regex_is_match!(r"(class)?var\s<|\^this.*|\|\w+\||\+\s\w*\s\{|\*ar\s", line) {
            return Some(FileType::Supercollider);
        }
    }
    Some(FileType::Scala)
}

pub(crate) fn scd(_path: &Path, content: &str) -> Option<FileType> {
    match regex_is_match!(
        r#"^\S+\(\d[0-9A-Za-z]*\)(\s+"[^"]*"]){0,2}"#,
        get_lines(content, 1)
    ) {
        true => Some(FileType::Scdoc),
        false => Some(FileType::Supercollider),
    }
}

pub(crate) fn sgml(_path: &Path, content: &str) -> Option<FileType> {
    let lines = get_lines(content, 5);
    if lines.contains("linuxdoc") {
        Some(FileType::Smgllnx)
    } else if regex_is_match!(r"<!DOCTYPE.*DocBook", lines) {
        Some(FileType::DocBookSgml4)
    } else {
        Some(FileType::Sgml)
    }
}

pub(crate) fn sh(content: &str, dialect: Option<FileType>) -> Option<FileType> {
    let dialect = dialect.unwrap_or_else(|| {
        let first_line = get_lines(content, 1);
        // try to detect from shebang
        if !regex_is_match!(r"^\s*#!", first_line) {
            FileType::Sh
        } else if regex_is_match!(r"\bcsh\b"i, first_line) {
            FileType::Csh
        } else if regex_is_match!(r"\btcsh\b"i, first_line) {
            FileType::Tcsh
        } else if regex_is_match!(r"\bzsh\b"i, first_line) {
            FileType::Zsh
        } else if regex_is_match!(r"\bksh\b"i, first_line) {
            FileType::Ksh
        } else if regex_is_match!(r"\b(bash|bash2)\b"i, first_line) {
            FileType::Bash
        } else {
            FileType::Sh
        }
    });
    shell(content, dialect)
}

pub(crate) fn shell(content: &str, dialect: FileType) -> Option<FileType> {
    let mut prev_line = "";
    for (line_num, line) in content.lines().enumerate().take(1000) {
        // skip the first line
        if line_num == 0 {
            prev_line = line;
            continue;
        }

        if regex_is_match!(r"\s*exec\s+(\S*/)?(tclsh|wish)"i, line)
            && !regex_is_match!(r"^\s*#.*\\$"i, prev_line)
        {
            // found an "exec" line with `tclsh` or `wish` after a comment with continuation
            return Some(FileType::Tcl);
        }

        prev_line = line;
    }
    Some(dialect)
}

pub(crate) fn sig(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred sig filetype
    let line = util::next_non_blank(content, 0)?;
    if regex_is_match!(r"^\s*(/\*|%|sig\s+[a-zA-Z])", line) {
        Some(FileType::LambdaProlog)
    } else if regex_is_match!(r"^\s*(\(\*|(signature|structure)\s+[a-zA-Z])", line) {
        Some(FileType::Sml)
    } else {
        None
    }
}

pub(crate) fn sil(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(100) {
        let trimmed_line = line.trim_start();
        if trimmed_line.starts_with(['\\', '%']) {
            return Some(FileType::Sile);
        } else if !trimmed_line.is_empty() {
            return Some(FileType::Sil);
        }
    }
    Some(FileType::Sil)
}

pub(crate) fn smi(_path: &Path, content: &str) -> Option<FileType> {
    match regex_is_match!(r"\bsmil\b"i, get_lines(content, 1)) {
        true => Some(FileType::Smil),
        false => Some(FileType::Mib),
    }
}

pub(crate) fn smil(_path: &Path, content: &str) -> Option<FileType> {
    match regex_is_match!(r"<\?\s*xml.*\?>", get_lines(content, 1)) {
        true => Some(FileType::Xml),
        false => Some(FileType::Smil),
    }
}

pub(crate) fn sql(_path: &Path, _content: &str) -> Option<FileType> {
    // TODO: user defined preferred sql filetype
    Some(FileType::Sql)
}

pub(crate) fn src(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred src filetype
    match util::next_non_blank(content, 0)
        .is_some_and(|line| regex_is_match!(r"^\s*(&\w+|(global\s+)?def(fct)?\b)"i, line))
    {
        true => Some(FileType::Krl),
        false => None,
    }
}

pub(crate) fn sys(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred sys filetype
    match is_rapid(content) {
        true => Some(FileType::Rapid),
        false => Some(FileType::Bat),
    }
}

pub(crate) fn tex(path: &Path, content: &str) -> Option<FileType> {
    let first_line = get_lines(content, 1);
    if regex_is_match!(r"^%&\s*plain(tex)?", first_line) {
        Some(FileType::PlainTex)
    } else if regex_is_match!(r"^%&\s*context", first_line)
        || path
            .to_str()
            .is_some_and(|p| regex_is_match!(r"tex/context/.*/.*\.tex"i, p))
    {
        Some(FileType::Context)
    } else {
        let latex_regex =
            regex!(r"^\s*\\(documentclass\b|usepackage\b|begin\{|newcommand\b|renewcommand\b)"i);
        let context_regex = regex!(
            r"^\s*\\(start[a-zA-Z]+|setup[a-zA-Z]+|usemodule|enablemode|enableregime|setvariables|useencoding|usesymbols|stelle[a-zA-Z]+|verwende[a-zA-Z]+|stel[a-zA-Z]+|gebruik[a-zA-Z]+|usa[a-zA-Z]+|imposta[a-zA-Z]+|regle[a-zA-Z]+|utilisemodule\b)"i
        );

        for line in content
            .lines()
            .skip_while(|line| regex_is_match!(r"^\s*%\S", line))
            .take(1000)
        {
            if latex_regex.is_match(line) {
                return Some(FileType::Tex);
            } else if context_regex.is_match(line) {
                return Some(FileType::Context);
            }
        }

        Some(FileType::Tex)
    }
}

pub(crate) fn tf(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines() {
        let trimmed_line = line.trim_start();
        if !trimmed_line.is_empty() && !trimmed_line.starts_with([';', '/']) {
            return Some(FileType::Terraform);
        }
    }
    Some(FileType::Tf)
}

pub(crate) fn tmp(path: &Path, content: &str) -> Option<FileType> {
    // for files like `main.rs~` retry search without the `~` suffix
    path.file_name()
        .and_then(|os_str| os_str.to_str())
        .and_then(|name| try_detect(path.with_file_name(&name[..name.len() - 1]), content))
}

pub(crate) fn ts(_path: &Path, content: &str) -> Option<FileType> {
    match regex_is_match!(r"<\?\s*xml", get_lines(content, 1)) {
        true => Some(FileType::Xml),
        false => Some(FileType::Smil),
    }
}

pub(crate) fn ttl(_path: &Path, content: &str) -> Option<FileType> {
    match regex_is_match!(r"^@?(prefix|base)", get_lines(content, 1)) {
        true => Some(FileType::Turtle),
        false => Some(FileType::Teraterm),
    }
}

pub(crate) fn txt(_path: &Path, content: &str) -> Option<FileType> {
    // vim helpfiles match *.txt but should have a modeline as last line
    match regex_is_match!(
        r"vim:.*ft=help",
        content.lines().next_back().unwrap_or(content)
    ) {
        true => Some(FileType::VimHelp),
        false => Some(FileType::Text),
    }
}

pub(crate) fn typ(_path: &Path, content: &str) -> Option<FileType> {
    // TODO: user defined preferred typ filetype
    for line in content.lines().take(200) {
        if regex_is_match!(r"^(CASE\s*=\s*(SAME|LOWER|UPPER|OPPOSITE)$|TYPE\s)", line) {
            return Some(FileType::Sql);
        }
    }
    Some(FileType::Typst)
}

pub(crate) fn v(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(200) {
        if !line.trim_start().starts_with('/') {
            if regex_is_match!(r";\s*($|/)", line) {
                return Some(FileType::Verilog);
            } else if regex_is_match!(r"\.\s*($|\(\*)", line) {
                return Some(FileType::Coq);
            }
        }
    }
    Some(FileType::V)
}

pub(crate) fn web(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(5) {
        if line.starts_with('%') {
            return Some(FileType::Web);
        }
    }
    Some(FileType::WinBatch)
}

pub(crate) fn xfree86(_path: &Path, content: &str) -> Option<FileType> {
    match regex_is_match!(r"\bXConfigurator\b", get_lines(content, 1)) {
        true => Some(FileType::XF86Conf3),
        false => Some(FileType::XF86Conf),
    }
}

pub(crate) fn xml(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(100) {
        if regex_is_match!(r"<!DOCTYPE.*DocBook", line) {
            return Some(FileType::DocBookXml4);
        } else if util::find(line, 0, false, " xmlns=\"http://docbook.org/ns/docbook\"") {
            return Some(FileType::DocBookXml5);
        } else if util::find(line, 0, false, "xmlns:xbl=\"http://www.mozilla.org/xbl\"") {
            return Some(FileType::Xbl);
        }
    }
    Some(FileType::Xml)
}

pub(crate) fn xpm(_path: &Path, content: &str) -> Option<FileType> {
    match util::find(content, 1, true, "XPM2") {
        true => Some(FileType::Xpm2),
        false => Some(FileType::Xpm),
    }
}

pub(crate) fn y(_path: &Path, content: &str) -> Option<FileType> {
    for line in content.lines().take(100) {
        if line.trim_start().starts_with('%') {
            return Some(FileType::Yacc);
        } else if regex_is_match!(r"^\s*(#|class\b)"i, line)
            && !regex_is_match!(r"^\s*#\s*include"i, line)
        {
            return Some(FileType::Racc);
        }
    }
    Some(FileType::Yacc)
}

pub(crate) fn make(path: &Path, _content: &str) -> Option<FileType> {
    // Check filename for specific makefile types
    let filename = path.file_name()?.to_str()?;
    if filename == "BSDmakefile" {
        // TODO: set buffer variable for bsd flavor
        return Some(FileType::Make);
    } else if filename == "GNUmakefile" {
        // TODO: set buffer variable for gnu flavor
        return Some(FileType::Make);
    }
    Some(FileType::Make)
}
