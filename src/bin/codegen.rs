use convert_case::{Case, Casing};
use pcre2::bytes::Regex as PCRERegex;
use phf_codegen::Map as PhfMap;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufWriter, Write},
    iter,
    path::Path,
};

type NamedPatterns = HashMap<String, MaybeMany<String>>;

#[derive(Deserialize)]
struct Heuristics {
    disambiguations: Vec<Disambiguation>,
    named_patterns: NamedPatterns,
}

#[derive(Deserialize)]
struct Disambiguation {
    extensions: Vec<String>,
    rules: Vec<RuleDTO>,
}

impl Disambiguation {
    fn to_domain_object_code(&self, named_patterns: &NamedPatterns) -> String {
        let mut rules = String::new();
        for rule in self.rules.iter() {
            let rule_code = rule.to_domain_object_code(named_patterns);
            // Skip empty rules (languages that don't exist in FileType)
            if !rule_code.is_empty() {
                rules.push_str(format!("{},", rule_code).as_str());
            }
        }
        format!("&[{}]", rules)
    }
}

#[derive(Deserialize)]
struct RuleDTO {
    language: MaybeMany<String>,
    #[serde(flatten)]
    pattern: Option<PatternDTO>,
}

impl RuleDTO {
    fn to_domain_object_code(&self, named_patterns: &NamedPatterns) -> String {
        let languages = match &self.language {
            MaybeMany::Many(values) => values.clone(),
            MaybeMany::One(value) => vec![value.clone()],
        };

        let pattern_code = match &self.pattern {
            Some(pattern) => format!("Some({})", pattern.to_domain_object_code(named_patterns)),
            None => String::from("None"),
        };

        // Convert language names to FileType::Variant format, filtering out languages
        // that don't exist in the FileType enum (to_pascal_case returns empty string)
        let language_types: Vec<String> = languages
            .iter()
            .filter_map(|lang| {
                let pascal = to_pascal_case(lang);
                if pascal.is_empty() {
                    None
                } else {
                    Some(format!("FileType::{}", pascal))
                }
            })
            .collect();

        // If all languages in this rule don't exist in FileType, skip the rule
        if language_types.is_empty() {
            return String::new();
        }

        format!(
            "Rule {{ languages: &[{}], pattern: {}}}",
            language_types.join(", "),
            pattern_code
        )
    }
}

#[derive(Clone, Deserialize)]
enum PatternDTO {
    #[serde(rename = "and")]
    And(Vec<PatternDTO>),
    #[serde(rename = "named_pattern")]
    Named(String),
    #[serde(rename = "negative_pattern")]
    Negative(String),
    #[serde(rename = "pattern")]
    Positive(MaybeMany<String>),
}

impl PatternDTO {
    fn to_domain_object_code(&self, named_patterns: &NamedPatterns) -> String {
        match self {
            PatternDTO::Positive(MaybeMany::One(pattern)) => {
                // Panic on invalid regex now so we can unwrap in lib
                if let Err(e) = PCRERegex::new(pattern) {
                    panic!("Invalid regex pattern: {}\n{}", pattern, e);
                }
                format!("Pattern::Positive({:?})", pattern)
            }
            PatternDTO::Negative(pattern) => {
                // Panic on invalid regex now so we can unwrap in lib
                if let Err(e) = PCRERegex::new(pattern) {
                    panic!("Invalid regex pattern: {}\n{}", pattern, e);
                }
                format!("Pattern::Negative({:?})", pattern)
            }
            PatternDTO::Positive(MaybeMany::Many(patterns)) => {
                let mut code = String::from("Pattern::Or(&[");
                for pattern in patterns.iter() {
                    let p = PatternDTO::Positive(MaybeMany::One(pattern.clone()));
                    code.push_str(format!("{},", p.to_domain_object_code(named_patterns)).as_str());
                }
                code.push_str("])");
                code
            }
            PatternDTO::And(patterns) => {
                let mut code = String::from("Pattern::And(&[");
                for pattern in patterns.iter() {
                    code.push_str(
                        format!("{},", pattern.to_domain_object_code(named_patterns)).as_str(),
                    );
                }
                code.push_str("])");
                code
            }
            PatternDTO::Named(pattern_name) => {
                if let Some(pattern) = named_patterns.get(pattern_name) {
                    // Assume that all named patterns are positive
                    let pattern = PatternDTO::Positive(pattern.clone());
                    return pattern.to_domain_object_code(named_patterns);
                } else {
                    panic!(
                        "Named pattern: {} not found in named pattern map",
                        pattern_name
                    );
                };
            }
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
enum MaybeMany<T> {
    Many(Vec<T>),
    One(T),
}

const DISAMBIGUATION_HEURISTICS_FILE: &str = "src/codegen/disambiguation-heuristics-map.rs";
const TOKEN_LOG_PROBABILITY_FILE: &str = "src/codegen/token-log-probabilities.rs";

const HEURISTICS_SOURCE_FILE: &str = "heuristics.yml";

const MAX_TOKEN_BYTES: usize = 32;

fn main() {
    let heuristics: Heuristics =
        serde_yaml::from_str(&fs::read_to_string(HEURISTICS_SOURCE_FILE).unwrap()[..]).unwrap();
    create_disambiguation_heuristics_map(heuristics);

    // Only train classifier if samples directory exists
    if Path::new("samples").exists() {
        train_classifier();
    } else {
        println!("Note: Skipping classifier training - 'samples' directory not found");
        println!("      Copy/link samples from hyperpolyglot to enable classifier training");
    }
}

fn create_disambiguation_heuristics_map(heuristics: Heuristics) {
    let mut file = BufWriter::new(File::create(DISAMBIGUATION_HEURISTICS_FILE).unwrap());

    let mut temp_map: HashMap<String, String> = HashMap::new();
    for mut dis in heuristics.disambiguations.into_iter() {
        for ext in dis.extensions.iter() {
            // Adding a rule to default to C for .h if the Objective C and C++ patterns don't match
            // The classifer was unreliable for distinguishing between C and C++ for .h
            if ext == ".h" {
                dis.rules.push(RuleDTO {
                    language: MaybeMany::One(String::from("C")),
                    pattern: None,
                });
            }
            let extension = ext.clone().to_ascii_lowercase();
            let key = extension;
            let value = dis.to_domain_object_code(&heuristics.named_patterns);
            temp_map.insert(key, value);
        }
    }

    let mut disambiguation_heuristic_map = PhfMap::new();
    for (key, value) in temp_map.iter() {
        disambiguation_heuristic_map.entry(&key[..], &value[..]);
    }

    writeln!(
        &mut file,
        "static DISAMBIGUATIONS: phf::Map<&'static str, &'static [Rule]> =\n{};\n",
        disambiguation_heuristic_map.build()
    )
    .unwrap();
}

fn train_classifier() {
    let mut temp_token_count: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut temp_total_tokens_count = HashMap::new();

    fs::read_dir("samples")
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().is_dir())
        .map(|language_dir| {
            let path = language_dir.path();
            let language = path.file_name().unwrap();
            let language = language.to_string_lossy().into_owned();
            let language = match &language[..] {
                "Fstar" => String::from("F*"),
                _ => language,
            };

            let file_paths = fs::read_dir(language_dir.path())
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.is_file());

            let language_iter = iter::repeat(language);
            file_paths.zip(language_iter)
        })
        .flatten()
        .for_each(|(entry, language)| {
            let content = fs::read(entry).unwrap();

            // When tokenizing an invalid utf8 string, just set it to ""
            // Add better error handling here in the future but unure of the best
            // way to handle it now
            let tokens =
                palate_polyglot_tokenizer::get_key_tokens(std::str::from_utf8(&content[..]).unwrap_or(""));

            for token in tokens {
                if token.len() <= MAX_TOKEN_BYTES {
                    let total_tokens = temp_total_tokens_count.entry(language.clone()).or_insert(0);
                    *total_tokens += 1;

                    let tokens_count = temp_token_count
                        .entry(language.clone())
                        .or_insert(HashMap::new());

                    let count = tokens_count.entry(String::from(token)).or_insert(0);
                    *count += 1;
                }
            }
        });

    // Write token log probabilities
    let mut file = BufWriter::new(File::create(TOKEN_LOG_PROBABILITY_FILE).unwrap());
    let mut language_token_log_probabilities = PhfMap::new();
    for (language, token_count_map) in temp_token_count.iter() {
        let total_tokens = *temp_total_tokens_count.get(language).unwrap() as f64;
        let mut token_log_probabilities = PhfMap::new();
        for (token, token_count) in token_count_map.iter() {
            let probability = (*token_count as f64) / (total_tokens);
            let log_probability = probability.ln();
            token_log_probabilities.entry(&token[..], &format!("{}f64", log_probability)[..]);
        }
        let codegen_log_prob_map = format!("{}", token_log_probabilities.build());
        language_token_log_probabilities.entry(&language[..], &codegen_log_prob_map[..]);
    }

    writeln!(
        &mut file,
        "static TOKEN_LOG_PROBABILITIES: phf::Map<&'static str, phf::Map<&'static str, f64>> =\n{};\n",
        language_token_log_probabilities.build()
    )
    .unwrap();
}

fn to_pascal_case(s: &str) -> String {
    // Special case mappings for languages with different names in heuristics.yml vs FileType enum
    match s.to_lowercase().as_str() {
        "c#" => return String::from("CSharp"),
        "c++" => return String::from("Cpp"),
        "f#" => return String::from("FSharp"),
        "8th" => return String::from("Eighth"),
        "objective-c" => return String::from("ObjC"),
        "objective-c++" => return String::from("ObjCpp"),
        "f*" => return String::from("Fstar"),
        "m" => return String::from("Mma"),                 // Mathematica/Wolfram Language -> Mma
        "wolfram language" => return String::from("Mma"),
        "standard ml" => return String::from("Sml"),
        "supercollider" => return String::from("Supercollider"),
        "star" => return String::from("Starlark"),        // Star (Q) -> Starlark
        "sqlpl" => return String::from("Plsql"),           // SQLPL -> Plsql
        "euphoria" => return String::from("Euphoria3"),    // Euphoria -> Euphoria3
        "cairo zero" => return String::from("Cairo"),   // Cairo Zero -> Cairo
        "common lisp" => return String::from("Lisp"),    // Common Lisp -> Lisp
        "roff manpage" => return String::from("Nroff"),  // Roff Manpage -> Nroff
        "roff" => return String::from("Nroff"),          // Roff -> Nroff
        "gnuplot" => return String::from("GnuPlot"),    // gnuplot -> GnuPlot
        "java properties" => return String::from("JProperties"),  // java properties -> JProperties
        "vim script" => return String::from("Vim"),     // vim script -> Vim
        "vim help file" => return String::from("VimHelp"), // vim help file -> VimHelp
        "hosts file" => return String::from("HostsAccess"),  // hosts file -> HostsAccess
        "tex" => return String::from("Tex"),            // tex -> Tex
        "plpgsql" => return String::from("Plsql"),     // plpgsql -> Plsql
        "tsql" => return String::from("Sql"),          // tsql -> Sql
        "hiveql" => return String::from("Sql"),         // hiveql -> Sql
        "glimmer ts" => return String::from("JavaScriptGlimmer"),  // glimmer ts -> JavaScriptGlimmer
        "nushell" => return String::from("Nu"),          // nushell -> Nu
        "ini" => return String::from("ConfIni"),       // ini -> ConfIni
        "stringtemplate" => return String::from("Template"),  // stringtemplate -> Template
        "oasv2-json" => return String::from("Json"),    // OAS v2 JSON -> Json
        "oasv2-yaml" => return String::from("Yaml"),    // OAS v2 YAML -> Yaml
        "oasv3-json" => return String::from("Json"),    // OAS v3 JSON -> Json
        "oasv3-yaml" => return String::from("Yaml"),    // OAS v3 YAML -> Yaml
        "java server pages" => return String::from("Jsp"),  // java server pages -> Jsp
        "go template" => return String::from("Gotmpl"),  // go template -> Gotmpl
        "makefile" => return String::from("Make"),  // makefile -> Make

        // Skip languages that don't exist in FileType enum and have no logical fallback
        "e" | "eclipse" | "ecl" => return String::new(),
        "smalltalk" | "frege" | "newlisp" | "miniyaml" | "reason" => return String::new(),
        "unity3d asset" | "g-code" => return String::new(),
        "object data instance notation" | "objectscript" | "opencl" | "openedge abl" | "openstep property list" => return String::new(),
        "pickle" | "picolisp" | "pod 6" | "pov-ray sdl" | "proguard" | "public key" => return String::new(),
        "q" | "q#" | "qmake" | "qt script" | "quickbasic" | "renderscript" | "rez" => return String::new(),
        "robotframework" | "rocq prover" | "ros interface" | "rpc" | "runoff" => return String::new(),
        "sourcepawn" | "stl" | "subrip text" | "survex data" | "tact" | "tl-verilog" => return String::new(),
        "tsplib data" | "turing" | "type language" | "typespec" | "unix assembly" => return String::new(),
        "vba" | "vcard" | "visual basic 6.0" | "win32 message file" | "world of warcraft addon data" => return String::new(),
        "xbase" | "xml property list" | "x pixmap" | "ren'py" | "omnet++ msg" | "omnet++ ned" => return String::new(),
        "actionscript" | "adblock filter list" | "ags script" | "algol" | "al" => return String::new(),
        "answer set programming" | "assembly" | "asymptote" | "b4x" | "beef" => return String::new(),
        "bibtex style" | "bikeshed" | "bitbake" | "blitzbasic" | "bluespec bh" => return String::new(),
        "brainfuck" | "brighterscript" | "buildstream" => return String::new(),
        "cool" | "cweb" | "directx 3d file" | "ecmarkup" | "filebench wml" => return String::new(),
        "filterscript" | "freemarker" | "game maker language" | "gap" => return String::new(),
        "gcc machine description" | "genie" | "gerber image" | "gosu" | "graph modeling language" => return String::new(),
        "gsc" | "hyphy" | "kcl" | "kframework" | "kusto" => return String::new(),
        "lambdapi" | "lean 4" | "limbo" | "linear programming" | "linker script" => return String::new(),
        "linux kernel module" | "livescript" | "logos" | "loomscript" => return String::new(),
        "ltspice symbol" | "m4sugar" | "maxscript" | "mdsvex" | "mercury" => return String::new(),
        "microsoft developer studio project" | "monkey c" | "motorola 68k assembly" => return String::new(),
        "muf" | "nasl" | "ncl" | "nemerle" | "nl" | "nmodl" | "noir" => return String::new(),

        _ => {}
    }

    // Use convert_case for the rest
    // Remove special characters that aren't valid in Rust identifiers
    s.to_case(Case::Pascal)
        .chars()
        .map(|c| match c {
            '#' | '+' | '\'' | '-' | '_' | ' ' | '.' => '_',
            c => c,
        })
        .collect()
}
