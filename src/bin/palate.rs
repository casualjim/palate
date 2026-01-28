use clap::{Arg, ArgAction, Command};
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};
use ignore::WalkBuilder;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use palate::{FileType, is_text_file, try_detect};

const MAX_CONTENT_SIZE_BYTES: usize = 51_200;

struct CLIOptions {
    color: bool,
    condensed_output: bool,
    filters: Option<Vec<Regex>>,
}

impl CLIOptions {
    fn matches_filter(&self, pattern: &str) -> bool {
        if let Some(filters) = &self.filters {
            filters.iter().any(|filter| filter.is_match(pattern))
        } else {
            true
        }
    }

    fn color_option(&self) -> ColorChoice {
        if self.color {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        }
    }
}

fn main() {
    let matches = get_cli().get_matches();
    let path = matches
        .get_one::<String>("PATH")
        .map(String::as_str)
        .unwrap_or(".");
    let root = Path::new(path);
    let root_is_dir = root.is_dir();

    let mut breakdown = get_language_breakdown(root);
    let mut language_count: Vec<(FileType, Vec<PathBuf>)> = breakdown.drain().collect();
    language_count.sort_by(|(_, a), (_, b)| b.len().cmp(&a.len()));
    for (_, files) in language_count.iter_mut() {
        files.sort();
    }

    if let Err(_) = print_language_split(&language_count) {
        std::process::exit(1);
    }

    let cli_options = CLIOptions {
        color: !matches.get_flag("no-color"),
        condensed_output: matches.get_flag("condensed"),
        filters: matches.get_many::<String>("filter").map(|filters| {
            filters
                .map(|f| Regex::new(f).unwrap_or_else(|_| {
                    eprintln!("Invalid filter: {}", f);
                    std::process::exit(1);
                }))
                .collect()
        }),
    };

    if matches.get_flag("file-breakdown") {
        writeln!(io::stdout(), "").unwrap_or_else(|_| std::process::exit(1));
        if let Err(_) = print_file_breakdown(&language_count, &cli_options, root, root_is_dir) {
            std::process::exit(1);
        }
    }
}

fn get_cli() -> Command {
    Command::new("palate")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Palate is a file type detector. It supports detecting the file type of a file or the file type makeup of a directory.")
        .arg(Arg::new("PATH").index(1).default_value("."))
        .arg(
            Arg::new("file-breakdown")
                .short('b')
                .long("breakdown")
                .help("prints the file type detected for each file visited")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("condensed")
                .short('c')
                .long("condensed")
                .help("condenses the output for the breakdowns to only show the headers")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("filter")
                .short('f')
                .long("filter")
                .help("a regex that is used to filter which sections get printed for the file and strategy breakdowns")
                .num_args(1)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("no-color")
                .short('n')
                .long("no-color")
                .help("don't color code the output (useful when piping)")
                .action(ArgAction::SetTrue),
        )
}

fn get_language_breakdown(root: &Path) -> HashMap<FileType, Vec<PathBuf>> {
    let mut breakdown: HashMap<FileType, Vec<PathBuf>> = HashMap::new();

    if root.is_file() {
        if let Some(file_type) = detect_path(root) {
            breakdown.entry(file_type).or_default().push(root.to_path_buf());
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
            breakdown
                .entry(file_type)
                .or_default()
                .push(path.to_path_buf());
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

fn print_language_split(language_counts: &Vec<(FileType, Vec<PathBuf>)>) -> Result<(), io::Error> {
    let total = language_counts
        .iter()
        .fold(0usize, |acc, (_, files)| acc + files.len()) as f64;
    if total == 0.0 {
        return Ok(());
    }

    for (language, files) in language_counts.iter() {
        let percentage = ((files.len() * 100) as f64) / total;
        writeln!(io::stdout(), "{:.2}% {}", percentage, file_type_name(*language))?;
    }

    Ok(())
}

fn print_file_breakdown(
    language_counts: &Vec<(FileType, Vec<PathBuf>)>,
    options: &CLIOptions,
    root: &Path,
    root_is_dir: bool,
) -> Result<(), io::Error> {
    let mut stdout = StandardStream::stdout(options.color_option());
    let mut title_color = ColorSpec::new();
    title_color.set_fg(Some(Color::Magenta));
    let default_color = ColorSpec::new();

    for (language, breakdowns) in language_counts.iter() {
        let language_name = file_type_name(*language);
        if options.matches_filter(language_name) {
            stdout.set_color(&title_color)?;
            write!(stdout, "{}", language_name)?;

            stdout.set_color(&default_color)?;
            writeln!(stdout, " ({})", breakdowns.len())?;
            if !options.condensed_output {
                for file in breakdowns.iter() {
                    let path = display_path(root, root_is_dir, file);
                    writeln!(stdout, "{}", path.display())?;
                }
                writeln!(stdout, "")?;
            }
        }
    }
    Ok(())
}

fn file_type_name(file_type: FileType) -> &'static str {
    file_type.into()
}

fn display_path<'a>(root: &Path, root_is_dir: bool, path: &'a Path) -> &'a Path {
    if root_is_dir {
        if let Ok(stripped) = path.strip_prefix(root) {
            return stripped;
        }
    }
    if let Ok(stripped) = path.strip_prefix(".") {
        return stripped;
    }
    path
}
