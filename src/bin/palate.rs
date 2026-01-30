use clap::{Arg, Command};
use ignore::WalkBuilder;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, Read, Write},
    path::{Path, PathBuf},
};

use palate::{FileType, is_text_file, try_detect};

const MAX_CONTENT_SIZE_BYTES: usize = 51_200;

#[derive(Default)]
struct LanguageStats {
    files: usize,
    lines: u64,
    blanks: u64,
    paths: Vec<PathBuf>,
}

fn main() {
    let matches = get_cli().get_matches();
    let path = matches
        .get_one::<String>("PATH")
        .map(String::as_str)
        .unwrap_or(".");
    let root = Path::new(path);

    let mut stats = scan_language_stats(root, false, true);
    let mut language_stats: Vec<(FileType, LanguageStats)> = stats.drain().collect();
    language_stats.sort_by(|(_, a), (_, b)| b.files.cmp(&a.files));
    for (_, data) in language_stats.iter_mut() {
        data.paths.sort();
    }

    if print_tokei_lite(&language_stats).is_err() {
        std::process::exit(1);
    }
}

fn get_cli() -> Command {
    Command::new("palate")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Palate is a file type detector. It supports detecting the file type of a file or the file type makeup of a directory.")
        .arg(Arg::new("PATH").index(1).default_value("."))
}

fn scan_language_stats(
    root: &Path,
    store_paths: bool,
    count_lines: bool,
) -> HashMap<FileType, LanguageStats> {
    let mut breakdown: HashMap<FileType, LanguageStats> = HashMap::new();

    if root.is_file() {
        if let Some(file_type) = detect_path(root) {
            let entry = breakdown.entry(file_type).or_default();
            entry.files += 1;
            if store_paths {
                entry.paths.push(root.to_path_buf());
            }
            if count_lines {
                if let Ok((lines, blanks)) = count_lines_and_blanks(root) {
                    entry.lines += lines;
                    entry.blanks += blanks;
                }
            }
        }
        return breakdown;
    }

    let walker = WalkBuilder::new(root).standard_filters(true).build();
    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            continue;
        }
        let path = entry.path();
        if let Some(file_type) = detect_path(path) {
            let entry = breakdown.entry(file_type).or_default();
            entry.files += 1;
            if store_paths {
                entry.paths.push(path.to_path_buf());
            }
            if count_lines {
                if let Ok((lines, blanks)) = count_lines_and_blanks(path) {
                    entry.lines += lines;
                    entry.blanks += blanks;
                }
            }
        }
    }

    breakdown
}

fn detect_path(path: &Path) -> Option<FileType> {
    if !is_text_file(path) {
        return None;
    }
    let content = read_file_content(path);
    try_detect(path, &content)
}

fn read_file_content(path: &Path) -> String {
    let mut buffer = Vec::new();
    if let Ok(file) = File::open(path) {
        let _ = file
            .take(MAX_CONTENT_SIZE_BYTES as u64)
            .read_to_end(&mut buffer);
    }
    String::from_utf8_lossy(&buffer).into_owned()
}

fn print_tokei_lite(language_stats: &[(FileType, LanguageStats)]) -> Result<(), io::Error> {
    let mut rows: Vec<(&'static str, usize, u64, u64)> = Vec::new(); // (lang, files, lines, blanks)
    let mut total_files = 0usize;
    let mut total_lines = 0u64;
    let mut total_blanks = 0u64;

    for (language, stats) in language_stats.iter() {
        let name = file_type_name(*language);
        rows.push((name, stats.files, stats.lines, stats.blanks));
        total_files += stats.files;
        total_lines += stats.lines;
        total_blanks += stats.blanks;
    }

    rows.sort_by(|a, b| b.2.cmp(&a.2)); // by lines desc

    let header = ("Language", "Files", "Lines", "Code", "Blanks");
    let mut w_lang = header.0.len();
    let mut w_files = header.1.len();
    let mut w_lines = header.2.len();
    let mut w_code = header.3.len();
    let mut w_blanks = header.4.len();

    for (lang, files, lines, blanks) in rows.iter() {
        w_lang = w_lang.max(lang.len());
        w_files = w_files.max(files.to_string().len());
        w_lines = w_lines.max(lines.to_string().len());
        w_code = w_code.max((lines.saturating_sub(*blanks)).to_string().len());
        w_blanks = w_blanks.max(blanks.to_string().len());
    }

    let total_code = total_lines.saturating_sub(total_blanks);
    w_files = w_files.max(total_files.to_string().len());
    w_lines = w_lines.max(total_lines.to_string().len());
    w_code = w_code.max(total_code.to_string().len());
    w_blanks = w_blanks.max(total_blanks.to_string().len());

    writeln!(
        io::stdout(),
        "{:<w_lang$} {:>w_files$} {:>w_lines$} {:>w_code$} {:>w_blanks$}",
        header.0,
        header.1,
        header.2,
        header.3,
        header.4,
        w_lang = w_lang,
        w_files = w_files,
        w_lines = w_lines,
        w_code = w_code,
        w_blanks = w_blanks
    )?;

    for (lang, files, lines, blanks) in rows {
        let code = lines.saturating_sub(blanks);
        writeln!(
            io::stdout(),
            "{:<w_lang$} {:>w_files$} {:>w_lines$} {:>w_code$} {:>w_blanks$}",
            lang,
            files,
            lines,
            code,
            blanks,
            w_lang = w_lang,
            w_files = w_files,
            w_lines = w_lines,
            w_code = w_code,
            w_blanks = w_blanks
        )?;
    }

    writeln!(
        io::stdout(),
        "{:<w_lang$} {:>w_files$} {:>w_lines$} {:>w_code$} {:>w_blanks$}",
        "Total",
        total_files,
        total_lines,
        total_code,
        total_blanks,
        w_lang = w_lang,
        w_files = w_files,
        w_lines = w_lines,
        w_code = w_code,
        w_blanks = w_blanks
    )?;

    Ok(())
}

fn file_type_name(file_type: FileType) -> &'static str {
    file_type.into()
}

fn count_lines_and_blanks(path: &Path) -> io::Result<(u64, u64)> {
    let file = File::open(path)?;
    let mut reader = io::BufReader::new(file);
    let mut buf = Vec::new();

    let mut lines = 0u64;
    let mut blanks = 0u64;

    loop {
        buf.clear();
        let n = reader.read_until(b'\n', &mut buf)?;
        if n == 0 {
            break;
        }
        lines += 1;
        if buf.iter().all(|b| b.is_ascii_whitespace()) {
            blanks += 1;
        }
    }

    Ok((lines, blanks))
}
