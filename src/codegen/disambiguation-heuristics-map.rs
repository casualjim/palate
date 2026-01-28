static DISAMBIGUATIONS: phf::Map<&'static str, &'static [Rule]> = ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (16, 9),
        (2, 33),
        (2, 71),
        (0, 0),
        (0, 19),
        (0, 1),
        (0, 15),
        (37, 31),
        (0, 1),
        (1, 54),
        (0, 37),
        (0, 10),
        (1, 0),
        (0, 132),
        (92, 107),
        (0, 6),
        (0, 2),
        (0, 79),
        (0, 1),
        (11, 43),
        (0, 52),
        (0, 37),
        (1, 81),
        (0, 124),
        (47, 39),
        (0, 1),
        (42, 51),
        (6, 126),
        (0, 3),
        (25, 63),
    ],
    entries: &[
        (
            ".cairo",
            &[
                Rule {
                    languages: &[FileType::Cairo],
                    pattern: Some(Pattern::Positive(
                        "(^(\\s*)%lang(\\s+)([A-Za-z0-9_]+))|(^(\\s*)%builtins(\\s+)([A-Za-z0-9_]+\\s*)*$)|(^(\\s*)from(\\s+)starkware\\.(cairo|starknet)\\.([A-Za-z0-9_.\\s]+?)import)|(,\\s*ap\\+\\+;$)|(;\\s*ap\\+\\+$)",
                    )),
                },
                Rule {
                    languages: &[FileType::Cairo],
                    pattern: None,
                },
            ],
        ),
        (
            ".cs",
            &[Rule {
                languages: &[FileType::CSharp],
                pattern: Some(Pattern::Positive(
                    "^\\s*(using\\s+[A-Z][\\s\\w.]+;|namespace\\s*[\\w\\.]+\\s*(\\{|;)|\\/\\/)",
                )),
            }],
        ),
        (
            ".3in",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: None,
                },
            ],
        ),
        (
            ".5",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::Positive("^\\.(?:[A-Za-z]{2}(?:\\s|$)|\\\\\")")),
                },
                Rule {
                    languages: &[FileType::Text],
                    pattern: None,
                },
            ],
        ),
        (
            ".fr",
            &[
                Rule {
                    languages: &[FileType::Forth],
                    pattern: Some(Pattern::Positive("^(: |also |new-device|previous )")),
                },
                Rule {
                    languages: &[FileType::Text],
                    pattern: None,
                },
            ],
        ),
        (
            ".yml",
            &[
                Rule {
                    languages: &[FileType::Yaml],
                    pattern: Some(Pattern::Positive("swagger:\\s?'?\"?2.[0-9.]+'?\"?")),
                },
                Rule {
                    languages: &[FileType::Yaml],
                    pattern: Some(Pattern::Positive("openapi:\\s?'?\"?3.[0-9.]+'?\"?")),
                },
                Rule {
                    languages: &[FileType::Yaml],
                    pattern: None,
                },
            ],
        ),
        (
            ".mdoc",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: None,
                },
            ],
        ),
        (
            ".3p",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: None,
                },
            ],
        ),
        (
            ".ex",
            &[
                Rule {
                    languages: &[FileType::Elixir],
                    pattern: Some(Pattern::Or(&[
                        Pattern::Positive("^\\s*@moduledoc\\s"),
                        Pattern::Positive("^\\s*(?:cond|import|quote|unless)\\s"),
                        Pattern::Positive("^\\s*def(?:exception|impl|macro|module|protocol)[(\\s]"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Euphoria3],
                    pattern: Some(Pattern::Or(&[
                        Pattern::Positive("^\\s*namespace\\s"),
                        Pattern::Positive("^\\s*(?:public\\s+)?include\\s"),
                        Pattern::Positive(
                            "^\\s*(?:(?:public|export|global)\\s+)?(?:atom|constant|enum|function|integer|object|procedure|sequence|type)\\s",
                        ),
                    ])),
                },
            ],
        ),
        (
            ".re",
            &[Rule {
                languages: &[FileType::Cpp],
                pattern: Some(Pattern::Or(&[
                    Pattern::Positive(
                        "^\\s*#(?:(?:if|ifdef|define|pragma)\\s+\\w|\\s*include\\s+<[^>]+>)",
                    ),
                    Pattern::Positive("^\\s*template\\s*<"),
                ])),
            }],
        ),
        (
            ".al",
            &[Rule {
                languages: &[FileType::Perl],
                pattern: None,
            }],
        ),
        (
            ".6",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::Positive("^\\.(?:[A-Za-z]{2}(?:\\s|$)|\\\\\")")),
                },
                Rule {
                    languages: &[FileType::Text],
                    pattern: None,
                },
            ],
        ),
        (
            ".yy",
            &[
                Rule {
                    languages: &[FileType::Json],
                    pattern: Some(Pattern::Positive("\\A\\s*[{\\[]")),
                },
                Rule {
                    languages: &[FileType::Yacc],
                    pattern: None,
                },
            ],
        ),
        (
            ".st",
            &[Rule {
                languages: &[FileType::Template],
                pattern: Some(Pattern::Positive(
                    "\\$\\w+[($]|.!\\s*.+?\\s*!.|<!\\s*.+?\\s*!>|\\[!\\s*.+?\\s*!\\]|\\{!\\s*.+?\\s*!\\}",
                )),
            }],
        ),
        (
            ".lisp",
            &[Rule {
                languages: &[FileType::Lisp],
                pattern: Some(Pattern::Positive(
                    "^\\s*\\((?i:defun|in-package|defpackage) ",
                )),
            }],
        ),
        (
            ".star",
            &[
                Rule {
                    languages: &[FileType::Starlark],
                    pattern: Some(Pattern::Positive("^loop_\\s*$")),
                },
                Rule {
                    languages: &[FileType::Starlark],
                    pattern: None,
                },
            ],
        ),
        (
            ".pp",
            &[
                Rule {
                    languages: &[FileType::Pascal],
                    pattern: Some(Pattern::Positive("^\\s*end[.;]")),
                },
                Rule {
                    languages: &[FileType::Puppet],
                    pattern: Some(Pattern::Positive("^\\s+\\w+\\s+=>\\s")),
                },
            ],
        ),
        (".mask", &[]),
        (
            ".cl",
            &[Rule {
                languages: &[FileType::Lisp],
                pattern: Some(Pattern::Positive(
                    "^\\s*\\((?i:defun|in-package|defpackage) ",
                )),
            }],
        ),
        (".x", &[]),
        (
            ".tsx",
            &[
                Rule {
                    languages: &[FileType::Xml],
                    pattern: Some(Pattern::Positive("(?i:^\\s*<\\?xml\\s+version)")),
                },
                Rule {
                    languages: &[FileType::Tsx],
                    pattern: None,
                },
            ],
        ),
        (
            ".d",
            &[
                Rule {
                    languages: &[FileType::D],
                    pattern: Some(Pattern::Positive(
                        "^module\\s+[\\w.]*\\s*;|import\\s+[\\w\\s,.:]*;|\\w+\\s+\\w+\\s*\\(.*\\)(?:\\(.*\\))?\\s*\\{[^}]*\\}|unittest\\s*(?:\\(.*\\))?\\s*\\{[^}]*\\}",
                    )),
                },
                Rule {
                    languages: &[FileType::DTrace],
                    pattern: Some(Pattern::Positive(
                        "^(\\w+:\\w*:\\w*:\\w*|BEGIN|END|provider\\s+|(tick|profile)-\\w+\\s+\\{[^}]*\\}|#pragma\\s+D\\s+(option|attributes|depends_on)\\s|#pragma\\s+ident\\s)",
                    )),
                },
            ],
        ),
        (".plist", &[]),
        (".as", &[]),
        (
            ".pro",
            &[
                Rule {
                    languages: &[FileType::Prolog],
                    pattern: Some(Pattern::Positive("^[^\\[#]+:-")),
                },
                Rule {
                    languages: &[FileType::ConfIni],
                    pattern: Some(Pattern::Positive("last_client=")),
                },
                Rule {
                    languages: &[FileType::Idl],
                    pattern: Some(Pattern::Positive(
                        "^\\s*(?i:function|pro|compile_opt) \\w[ \\w,:]*$",
                    )),
                },
            ],
        ),
        (".tag", &[]),
        (
            ".txt",
            &[
                Rule {
                    languages: &[FileType::VimHelp],
                    pattern: Some(Pattern::Positive(
                        "(?:(?:^|[ \\t])(?:vi|Vi(?=m))(?:m[<=>]?[0-9]+|m)?|[ \\t]ex)(?=:(?=[ \\t]*set?[ \\t][^\\r\\n:]+:)|:(?![ \\t]*set?[ \\t]))(?:(?:[ \\t]*:[ \\t]*|[ \\t])\\w*(?:[ \\t]*=(?:[^\\\\\\s]|\\\\.)*)?)*[ \\t:](?:filetype|ft|syntax)[ \\t]*=(help)(?=$|\\s|:)",
                    )),
                },
                Rule {
                    languages: &[FileType::HostsAccess],
                    pattern: Some(Pattern::Positive(
                        "(?xi) ^\n\n# IPv4 address\n(?<ipv4>\n  (?!\\.)\n  (?:\\.?\n    (?: 25[0-5]  # 250-255\n    |   2[0-4]\\d # 200-249\n    |   1\\d\\d    # 100-199\n    |   [1-9]?\\d # 0-99\n    )\\b\n){4})\n\n# CIDR notation: /[0-32]\n(?<cidr>/(3[0-2]|[12]?\\d)\\b)?\n\n# Domains list\n(?<domains>\n  [ \\t]+\n  \\w[-\\w]* (?:\\.\\w[-\\w]*)*\n  (?<!-)\\b\n)*+\n\n(?:$|\\s)",
                    )),
                },
                Rule {
                    languages: &[FileType::Text],
                    pattern: None,
                },
            ],
        ),
        (
            ".hh",
            &[Rule {
                languages: &[FileType::Hack],
                pattern: Some(Pattern::Positive("<\\?hh")),
            }],
        ),
        (".asy", &[]),
        (
            ".vcf",
            &[Rule {
                languages: &[FileType::Tsv],
                pattern: Some(Pattern::Positive("\\A##fileformat=VCF")),
            }],
        ),
        (
            ".odin",
            &[Rule {
                languages: &[FileType::Odin],
                pattern: Some(Pattern::Positive(
                    "package\\s+\\w+|\\b(?:im|ex)port\\s*\"[\\w:./]+\"|\\w+\\s*::\\s*(?:proc|struct)\\s*\\(|^\\s*//\\s",
                )),
            }],
        ),
        (
            ".php",
            &[
                Rule {
                    languages: &[FileType::Hack],
                    pattern: Some(Pattern::Positive("<\\?hh")),
                },
                Rule {
                    languages: &[FileType::Php],
                    pattern: Some(Pattern::Positive("<\\?[^h]")),
                },
            ],
        ),
        (".gsc", &[]),
        (
            ".7",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::Positive("^\\.(?:[A-Za-z]{2}(?:\\s|$)|\\\\\")")),
                },
                Rule {
                    languages: &[FileType::Text],
                    pattern: None,
                },
            ],
        ),
        (
            ".bb",
            &[Rule {
                languages: &[FileType::Clojure],
                pattern: Some(Pattern::Positive("\\((def|defn|defmacro|let)\\s")),
            }],
        ),
        (".ecl", &[]),
        (".gsh", &[]),
        (
            ".mojo",
            &[
                Rule {
                    languages: &[FileType::Mojo],
                    pattern: Some(Pattern::Positive(
                        "^\\s*(alias|def|from|fn|import|struct|trait)\\s",
                    )),
                },
                Rule {
                    languages: &[FileType::Xml],
                    pattern: Some(Pattern::Positive("^\\s*<\\?xml")),
                },
            ],
        ),
        (
            ".mc",
            &[Rule {
                languages: &[FileType::M4],
                pattern: Some(Pattern::Positive(
                    "^dnl|^divert\\((?:-?\\d+)?\\)|^\\w+\\(`[^\\r\\n]*?'[),]",
                )),
            }],
        ),
        (
            ".1x",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: None,
                },
            ],
        ),
        (".bst", &[]),
        (".ls", &[]),
        (
            ".e",
            &[
                Rule {
                    languages: &[FileType::Eiffel],
                    pattern: Some(Pattern::Or(&[
                        Pattern::Positive("^\\s*\\w+\\s*(?:,\\s*\\w+)*[:]\\s*\\w+\\s"),
                        Pattern::Positive(
                            "^\\s*\\w+\\s*(?:\\(\\s*\\w+[:][^)]+\\))?(?:[:]\\s*\\w+)?(?:--.+\\s+)*\\s+(?:do|local)\\s",
                        ),
                        Pattern::Positive(
                            "^\\s*(?:across|deferred|elseif|ensure|feature|from|inherit|inspect|invariant|note|once|require|undefine|variant|when)\\s*$",
                        ),
                    ])),
                },
                Rule {
                    languages: &[FileType::Euphoria3],
                    pattern: Some(Pattern::Or(&[
                        Pattern::Positive("^\\s*namespace\\s"),
                        Pattern::Positive("^\\s*(?:public\\s+)?include\\s"),
                        Pattern::Positive(
                            "^\\s*(?:(?:public|export|global)\\s+)?(?:atom|constant|enum|function|integer|object|procedure|sequence|type)\\s",
                        ),
                    ])),
                },
            ],
        ),
        (
            ".scm",
            &[
                Rule {
                    languages: &[FileType::TreeSitterQuery],
                    pattern: Some(Pattern::Or(&[
                        Pattern::Positive("\\(#[\\w-]+[!\\?]"),
                        Pattern::Positive("(?:[\\)\\]]\\s*[\\*\\+\\?](?:\\s|$))"),
                        Pattern::Positive("(?:^\\s*\\w+:\\s*[\\(\\[\\\"])"),
                        Pattern::Positive("\\(#(?:set!|(?:not-)?(?:any-of|match)\\?)"),
                        Pattern::Positive("@[\\w.-]+(?:\\)\\s|$)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Scheme],
                    pattern: Some(Pattern::Or(&[
                        Pattern::Positive(
                            "(?:'[\\(\\*#]|\\w->\\w|\\.\\.\\.[\\s\\)]|\\([+\\-:<>\\/=~\\)]|~>|[#`]\\(|#:\\w)",
                        ),
                        Pattern::Positive("^\\s*\\((?:define\\*?|import|library|lambda)"),
                    ])),
                },
            ],
        ),
        (".tlv", &[]),
        (
            ".builds",
            &[Rule {
                languages: &[FileType::Xml],
                pattern: Some(Pattern::Positive(
                    "^(\\s*)(?i:<Project|<Import|<Property|<?xml|xmlns)",
                )),
            }],
        ),
        (
            ".pl",
            &[
                Rule {
                    languages: &[FileType::Prolog],
                    pattern: Some(Pattern::Positive("^[^#]*:-")),
                },
                Rule {
                    languages: &[FileType::Perl],
                    pattern: Some(Pattern::And(&[
                        Pattern::Negative("^\\s*use\\s+v6\\b"),
                        Pattern::Or(&[
                            Pattern::Positive("\\buse\\s+(?:strict\\b|v?5\\b)"),
                            Pattern::Positive("^\\s*use\\s+(?:constant|overload)\\b"),
                            Pattern::Positive("^\\s*(?:\\*|(?:our\\s*)?@)EXPORT\\s*="),
                            Pattern::Positive(
                                "^\\s*package\\s+[^\\W\\d]\\w*(?:::\\w+)*\\s*(?:[;{]|\\sv?\\d)",
                            ),
                            Pattern::Positive("[\\s$][^\\W\\d]\\w*(?::\\w+)*->[a-zA-Z_\\[({]"),
                        ]),
                    ])),
                },
                Rule {
                    languages: &[FileType::Raku],
                    pattern: Some(Pattern::Positive(
                        "^\\s*(?:use\\s+v6\\b|\\bmodule\\b|\\b(?:my\\s+)?class\\b)",
                    )),
                },
            ],
        ),
        (
            ".rs",
            &[
                Rule {
                    languages: &[FileType::Rust],
                    pattern: Some(Pattern::Positive(
                        "^(use |fn |mod |pub |macro_rules|impl|#!?\\[)",
                    )),
                },
                Rule {
                    languages: &[FileType::Xml],
                    pattern: Some(Pattern::Positive("^\\s*<\\?xml")),
                },
            ],
        ),
        (".srt", &[]),
        (
            ".plt",
            &[Rule {
                languages: &[FileType::Prolog],
                pattern: Some(Pattern::Positive("^\\s*:-")),
            }],
        ),
        (
            ".man",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: None,
                },
            ],
        ),
        (
            ".h",
            &[
                Rule {
                    languages: &[FileType::ObjC],
                    pattern: Some(Pattern::Positive(
                        "^\\s*(@(interface|class|protocol|property|end|synchronised|selector|implementation)\\b|#import\\s+.+\\.h[\">])",
                    )),
                },
                Rule {
                    languages: &[FileType::Cpp],
                    pattern: Some(Pattern::Or(&[
                        Pattern::Positive(
                            "^\\s*#\\s*include <(cstdint|string|vector|map|list|array|bitset|queue|stack|forward_list|unordered_map|unordered_set|(i|o|io)stream)>",
                        ),
                        Pattern::Positive("^\\s*template\\s*<"),
                        Pattern::Positive("^[ \\t]*(try|constexpr)"),
                        Pattern::Positive("^[ \\t]*catch\\s*\\("),
                        Pattern::Positive("^[ \\t]*(class|(using[ \\t]+)?namespace)\\s+\\w+"),
                        Pattern::Positive("^[ \\t]*(private|public|protected):$"),
                        Pattern::Positive("__has_cpp_attribute|__cplusplus >"),
                        Pattern::Positive("std::\\w+"),
                    ])),
                },
                Rule {
                    languages: &[FileType::C],
                    pattern: None,
                },
                Rule {
                    languages: &[FileType::C],
                    pattern: None,
                },
            ],
        ),
        (
            ".1m",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: None,
                },
            ],
        ),
        (
            ".1",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::Positive("^\\.(?:[A-Za-z]{2}(?:\\s|$)|\\\\\")")),
                },
                Rule {
                    languages: &[FileType::Text],
                    pattern: None,
                },
            ],
        ),
        (
            ".gs",
            &[Rule {
                languages: &[FileType::Glsl],
                pattern: Some(Pattern::Positive("^#version\\s+[0-9]+\\b")),
            }],
        ),
        (
            ".ftl",
            &[Rule {
                languages: &[FileType::Fluent],
                pattern: Some(Pattern::Positive(
                    "^-?[a-zA-Z][a-zA-Z0-9_-]* *=|\\{\\$-?[a-zA-Z][-\\w]*(?:\\.[a-zA-Z][-\\w]*)?\\}",
                )),
            }],
        ),
        (
            ".toc",
            &[Rule {
                languages: &[FileType::Tex],
                pattern: Some(Pattern::Positive(
                    "^\\\\(contentsline|defcounter|beamer|boolfalse)",
                )),
            }],
        ),
        (".tsp", &[]),
        (".alg", &[]),
        (
            ".inc",
            &[
                Rule {
                    languages: &[FileType::Php],
                    pattern: Some(Pattern::Positive("^<\\?(?:php)?")),
                },
                Rule {
                    languages: &[FileType::Pascal],
                    pattern: Some(Pattern::Or(&[
                        Pattern::Positive(
                            "(?i:^\\s*\\{\\$(?:mode|ifdef|undef|define)[ ]+[a-z0-9_]+\\})",
                        ),
                        Pattern::Positive("^\\s*end[.;]\\s*$"),
                    ])),
                },
            ],
        ),
        (
            ".v",
            &[
                Rule {
                    languages: &[FileType::Verilog],
                    pattern: Some(Pattern::Positive(
                        "^[ \\t]*module\\s+[^\\s()]+\\s*\\#?\\(|^[ \\t]*`(?:define|ifdef|ifndef|include|timescale|pragma)|^[ \\t]*always[ \\t]*@|^[ \\t]*initial[ \\t]*(begin|@)",
                    )),
                },
                Rule {
                    languages: &[FileType::V],
                    pattern: Some(Pattern::Positive(
                        "\\$(?:if|else)[ \\t]|^[ \\t]*fn\\s+[^\\s()]+\\(.*?\\).*?\\{|^[ \\t]*for\\s*\\{",
                    )),
                },
            ],
        ),
        (".asm", &[]),
        (
            ".lean",
            &[Rule {
                languages: &[FileType::Lean],
                pattern: Some(Pattern::Positive("^import [a-z]")),
            }],
        ),
        (
            ".app",
            &[Rule {
                languages: &[FileType::Erlang],
                pattern: Some(Pattern::Positive(
                    "^\\{\\s*(?:application|'application')\\s*,\\s*(?:[a-z]+[\\w@]*|'[^']+')\\s*,\\s*\\[(?:.|[\\r\\n])*\\]\\s*\\}\\.[ \\t]*$",
                )),
            }],
        ),
        (".svx", &[]),
        (
            ".frm",
            &[Rule {
                languages: &[FileType::ConfIni],
                pattern: Some(Pattern::Positive("\\ATYPE=VIEW")),
            }],
        ),
        (
            ".vba",
            &[Rule {
                languages: &[FileType::Vim],
                pattern: Some(Pattern::Positive("^UseVimball")),
            }],
        ),
        (
            ".dsp",
            &[Rule {
                languages: &[FileType::Faust],
                pattern: Some(Pattern::Positive(
                    "\\bprocess\\s*[(=]|\\b(library|import)\\s*\\(\\s*\"|\\bdeclare\\s+(name|version|author|copyright|license)\\s+\"",
                )),
            }],
        ),
        (".g", &[]),
        (
            ".rno",
            &[Rule {
                languages: &[FileType::Nroff],
                pattern: Some(Pattern::Positive("^\\.\\\\\" ")),
            }],
        ),
        (
            ".tpl",
            &[Rule {
                languages: &[FileType::Smarty],
                pattern: Some(Pattern::Positive("(?<!\\{)\\{(\\*\\s|\\$|\\/)?\\w*\\b")),
            }],
        ),
        (
            ".yaml",
            &[
                Rule {
                    languages: &[FileType::Yaml],
                    pattern: Some(Pattern::Positive("swagger:\\s?'?\"?2.[0-9.]+'?\"?")),
                },
                Rule {
                    languages: &[FileType::Yaml],
                    pattern: Some(Pattern::Positive("openapi:\\s?'?\"?3.[0-9.]+'?\"?")),
                },
                Rule {
                    languages: &[FileType::Yaml],
                    pattern: None,
                },
            ],
        ),
        (
            ".3pm",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: None,
                },
            ],
        ),
        (
            ".4",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::Positive("^\\.(?:[A-Za-z]{2}(?:\\s|$)|\\\\\")")),
                },
                Rule {
                    languages: &[FileType::Text],
                    pattern: None,
                },
            ],
        ),
        (
            ".typ",
            &[
                Rule {
                    languages: &[FileType::Typst],
                    pattern: Some(Pattern::Positive("^#(import|show|let|set)")),
                },
                Rule {
                    languages: &[FileType::Xml],
                    pattern: None,
                },
            ],
        ),
        (
            ".sol",
            &[Rule {
                languages: &[FileType::Solidity],
                pattern: Some(Pattern::Positive(
                    "\\bpragma\\s+solidity\\b|\\b(?:abstract\\s+)?contract\\s+[a-zA-Z$_][a-zA-Z0-9$_]*(?:\\s+is\\s+(?:[a-zA-Z0-9$_][^\\{]*?)?)?\\s*\\{",
                )),
            }],
        ),
        (
            ".3qt",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: None,
                },
            ],
        ),
        (
            ".l",
            &[
                Rule {
                    languages: &[FileType::Lisp],
                    pattern: Some(Pattern::Positive("\\(def(un|macro)\\s")),
                },
                Rule {
                    languages: &[FileType::Lex],
                    pattern: Some(Pattern::Positive("^(%[%{}]xs|<.*>)")),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::Positive("^\\.[A-Za-z]{2}(\\s|$)")),
                },
            ],
        ),
        (".cmp", &[]),
        (
            ".properties",
            &[
                Rule {
                    languages: &[FileType::ConfIni],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[^#!;][^=]*="),
                        Pattern::Positive("^[;\\[]"),
                    ])),
                },
                Rule {
                    languages: &[FileType::JProperties],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[^#!;][^=]*="),
                        Pattern::Positive("^[#!]"),
                    ])),
                },
                Rule {
                    languages: &[FileType::ConfIni],
                    pattern: Some(Pattern::Positive("^[^#!;][^=]*=")),
                },
                Rule {
                    languages: &[FileType::JProperties],
                    pattern: Some(Pattern::Positive("^[^#!][^:]*:")),
                },
            ],
        ),
        (
            ".2",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::Positive("^\\.(?:[A-Za-z]{2}(?:\\s|$)|\\\\\")")),
                },
                Rule {
                    languages: &[FileType::Text],
                    pattern: None,
                },
            ],
        ),
        (
            ".html",
            &[Rule {
                languages: &[FileType::Html],
                pattern: None,
            }],
        ),
        (
            ".sql",
            &[
                Rule {
                    languages: &[FileType::Plsql],
                    pattern: Some(Pattern::Positive(
                        "(?i:^\\\\i\\b|AS\\s+\\$\\$|LANGUAGE\\s+'?plpgsql'?|BEGIN(\\s+WORK)?\\s*;)",
                    )),
                },
                Rule {
                    languages: &[FileType::Plsql],
                    pattern: Some(Pattern::Positive(
                        "(?i:ALTER\\s+MODULE|MODE\\s+DB2SQL|\\bSYS(CAT|PROC)\\.|ASSOCIATE\\s+RESULT\\s+SET|\\bEND!\\s*$)",
                    )),
                },
                Rule {
                    languages: &[FileType::Plsql],
                    pattern: Some(Pattern::Positive(
                        "(?i:\\$\\$PLSQL_|XMLTYPE|systimestamp|\\.nextval|CONNECT\\s+BY|AUTHID\\s+(DEFINER|CURRENT_USER)|constructor\\W+function)",
                    )),
                },
                Rule {
                    languages: &[FileType::Sql],
                    pattern: Some(Pattern::Positive(
                        "(?i:^\\s*GO\\b|BEGIN(\\s+TRY|\\s+CATCH)|OUTPUT\\s+INSERTED|DECLARE\\s+@|\\[dbo\\])",
                    )),
                },
                Rule {
                    languages: &[FileType::Sql],
                    pattern: None,
                },
            ],
        ),
        (
            ".sc",
            &[
                Rule {
                    languages: &[FileType::Supercollider],
                    pattern: Some(Pattern::Positive(
                        "(?i:\\^(this|super)\\.|^\\s*~\\w+\\s*=\\.)",
                    )),
                },
                Rule {
                    languages: &[FileType::Scala],
                    pattern: Some(Pattern::Positive(
                        "(^\\s*import (scala|java)\\.|^\\s*class\\b)",
                    )),
                },
            ],
        ),
        (
            ".3",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::Positive("^\\.(?:[A-Za-z]{2}(?:\\s|$)|\\\\\")")),
                },
                Rule {
                    languages: &[FileType::Text],
                    pattern: None,
                },
            ],
        ),
        (".lp", &[]),
        (
            ".scd",
            &[
                Rule {
                    languages: &[FileType::Supercollider],
                    pattern: Some(Pattern::Positive(
                        "(?i:\\^(this|super)\\.|^\\s*(~\\w+\\s*=\\.|SynthDef\\b))",
                    )),
                },
                Rule {
                    languages: &[FileType::Markdown],
                    pattern: Some(Pattern::Positive("^#+\\s+(NAME|SYNOPSIS|DESCRIPTION)")),
                },
            ],
        ),
        (
            ".q",
            &[Rule {
                languages: &[FileType::Sql],
                pattern: Some(Pattern::Positive(
                    "(?i:SELECT\\s+[\\w*,]+\\s+FROM|(CREATE|ALTER|DROP)\\s(DATABASE|SCHEMA|TABLE))",
                )),
            }],
        ),
        (
            ".tl",
            &[Rule {
                languages: &[FileType::Teal],
                pattern: Some(Pattern::And(&[
                    Pattern::Positive("--.*"),
                    Pattern::Positive("\\b(local|function|end|record|interface|enum)\\b"),
                ])),
            }],
        ),
        (
            ".cls",
            &[Rule {
                languages: &[FileType::Tex],
                pattern: Some(Pattern::Positive(
                    "^\\s*\\\\(?:NeedsTeXFormat|ProvidesClass)\\{",
                )),
            }],
        ),
        (
            ".9",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::Positive("^\\.(?:[A-Za-z]{2}(?:\\s|$)|\\\\\")")),
                },
                Rule {
                    languages: &[FileType::Text],
                    pattern: None,
                },
            ],
        ),
        (
            ".ts",
            &[
                Rule {
                    languages: &[FileType::Xml],
                    pattern: Some(Pattern::Positive("<TS\\b")),
                },
                Rule {
                    languages: &[FileType::TypeScript],
                    pattern: None,
                },
            ],
        ),
        (".w", &[]),
        (".resource", &[]),
        (
            ".ice",
            &[
                Rule {
                    languages: &[FileType::Json],
                    pattern: Some(Pattern::Positive("\\A\\s*[{\\[]")),
                },
                Rule {
                    languages: &[FileType::Slice],
                    pattern: None,
                },
            ],
        ),
        (
            ".pod",
            &[Rule {
                languages: &[FileType::Pod],
                pattern: None,
            }],
        ),
        (
            ".ncl",
            &[
                Rule {
                    languages: &[FileType::Xml],
                    pattern: Some(Pattern::Positive("^\\s*<\\?xml\\s+version")),
                },
                Rule {
                    languages: &[FileType::Nickel],
                    pattern: Some(Pattern::Or(&[
                        Pattern::Positive("^let(?:\\srec)?(?:\\s[a-zA-Z_][a-zA-Z0-9_]*)?"),
                        Pattern::Positive("^import\\s\"[^\"]+\"\\s+as\\s"),
                        Pattern::Positive("std\\.[a-zA-Z_][a-zA-Z0-9_]*\\."),
                    ])),
                },
                Rule {
                    languages: &[FileType::Text],
                    pattern: Some(Pattern::Positive("THE_TITLE")),
                },
            ],
        ),
        (
            ".url",
            &[Rule {
                languages: &[FileType::ConfIni],
                pattern: Some(Pattern::Positive(
                    "^\\[InternetShortcut\\](?:\\r?\\n|\\r)([^\\s\\[][^\\r\\n]*(?:\\r?\\n|\\r)){0,20}URL=",
                )),
            }],
        ),
        (
            ".gml",
            &[Rule {
                languages: &[FileType::Xml],
                pattern: Some(Pattern::Positive("(?i:^\\s*(<\\?xml|xmlns))")),
            }],
        ),
        (
            ".ms",
            &[Rule {
                languages: &[FileType::Nroff],
                pattern: Some(Pattern::Positive("^[.'][A-Za-z]{2}(\\s|$)")),
            }],
        ),
        (
            ".sw",
            &[
                Rule {
                    languages: &[FileType::Sway],
                    pattern: Some(Pattern::Positive(
                        "^\\s*(?:(?:abi|dep|fn|impl|mod|pub|trait)\\s|#\\[)",
                    )),
                },
                Rule {
                    languages: &[FileType::Xml],
                    pattern: Some(Pattern::Positive("^\\s*<\\?xml\\s+version")),
                },
            ],
        ),
        (
            ".lsp",
            &[Rule {
                languages: &[FileType::Lisp],
                pattern: Some(Pattern::Positive(
                    "^\\s*\\((?i:defun|in-package|defpackage) ",
                )),
            }],
        ),
        (
            ".3m",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: None,
                },
            ],
        ),
        (
            ".i",
            &[Rule {
                languages: &[FileType::Swig],
                pattern: Some(Pattern::Positive("^[ \\t]*%[a-z_]+\\b|^%[{}]$")),
            }],
        ),
        (
            ".gts",
            &[Rule {
                languages: &[FileType::JavaScriptGlimmer],
                pattern: Some(Pattern::Negative("^G0.")),
            }],
        ),
        (".msg", &[]),
        (".qs", &[]),
        (
            ".1in",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: None,
                },
            ],
        ),
        (
            ".json",
            &[
                Rule {
                    languages: &[FileType::Json],
                    pattern: Some(Pattern::Positive("\"swagger\":\\s?\"2.[0-9.]+\"")),
                },
                Rule {
                    languages: &[FileType::Json],
                    pattern: Some(Pattern::Positive("\"openapi\":\\s?\"3.[0-9.]+\"")),
                },
                Rule {
                    languages: &[FileType::Json],
                    pattern: None,
                },
            ],
        ),
        (
            ".bi",
            &[
                Rule {
                    languages: &[FileType::FreeBasic],
                    pattern: Some(Pattern::Or(&[
                        Pattern::Positive(
                            "(?i)^[ \\t]*#(?:define|endif|endmacro|ifn?def|include|lang|macro|pragma)(?:$|\\s)",
                        ),
                        Pattern::Positive(
                            "(?i)^[ \\t]*dim( shared)? [a-z_][a-z0-9_]* as [a-z_][a-z0-9_]* ptr",
                        ),
                    ])),
                },
                Rule {
                    languages: &[FileType::FreeBasic],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("(?i)^[ \\t]*return "),
                        Pattern::Negative("(?i)[ \\t]*gosub "),
                    ])),
                },
            ],
        ),
        (
            ".m4",
            &[Rule {
                languages: &[FileType::M4],
                pattern: None,
            }],
        ),
        (
            ".f",
            &[
                Rule {
                    languages: &[FileType::Forth],
                    pattern: Some(Pattern::Positive("^: ")),
                },
                Rule {
                    languages: &[FileType::Fortran],
                    pattern: Some(Pattern::Positive(
                        "^(?i:[c*][^abd-z]|      (subroutine|program|end|data)\\s|\\s*!)",
                    )),
                },
            ],
        ),
        (
            ".pkl",
            &[Rule {
                languages: &[FileType::Pkl],
                pattern: Some(Pattern::Or(&[
                    Pattern::Positive(
                        "^\\s*(module|import|amends|extends|local|const|fixed|abstract|open|class|typealias|@\\w+)\\b",
                    ),
                    Pattern::Positive(
                        "^\\s*[a-zA-Z0-9_$]+\\s*(=|{|:)|^\\s*`[^`]+`\\s*(=|{|:)|for\\s*\\(|when\\s*\\(",
                    ),
                ])),
            }],
        ),
        (".action", &[]),
        (
            ".bas",
            &[
                Rule {
                    languages: &[FileType::FreeBasic],
                    pattern: Some(Pattern::Or(&[
                        Pattern::Positive(
                            "(?i)^[ \\t]*#(?:define|endif|endmacro|ifn?def|include|lang|macro|pragma)(?:$|\\s)",
                        ),
                        Pattern::Positive(
                            "(?i)^[ \\t]*dim( shared)? [a-z_][a-z0-9_]* as [a-z_][a-z0-9_]* ptr",
                        ),
                    ])),
                },
                Rule {
                    languages: &[FileType::FreeBasic],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("(?i)^[ \\t]*return "),
                        Pattern::Negative("(?i)[ \\t]*gosub "),
                    ])),
                },
                Rule {
                    languages: &[FileType::Basic],
                    pattern: Some(Pattern::Positive("\\A\\s*\\d")),
                },
            ],
        ),
        (
            ".tst",
            &[Rule {
                languages: &[FileType::Scilab],
                pattern: None,
            }],
        ),
        (
            ".csl",
            &[Rule {
                languages: &[FileType::Xml],
                pattern: Some(Pattern::Positive("(?i:^\\s*(<\\?xml|xmlns))")),
            }],
        ),
        (
            ".md",
            &[
                Rule {
                    languages: &[FileType::Markdown],
                    pattern: Some(Pattern::Or(&[
                        Pattern::Positive("(^[-A-Za-z0-9=#!\\*\\[|>])|<\\/"),
                        Pattern::Positive("\\A\\z"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Markdown],
                    pattern: None,
                },
            ],
        ),
        (".csc", &[]),
        (
            ".rpy",
            &[Rule {
                languages: &[FileType::Python],
                pattern: Some(Pattern::Positive("^(import|from|class|def)\\s")),
            }],
        ),
        (
            ".mod",
            &[
                Rule {
                    languages: &[FileType::Xml],
                    pattern: Some(Pattern::Positive("<!ENTITY ")),
                },
                Rule {
                    languages: &[FileType::Modula2],
                    pattern: Some(Pattern::Positive("^\\s*(?i:MODULE|END) [\\w\\.]+;")),
                },
                Rule {
                    languages: &[FileType::Ampl],
                    pattern: None,
                },
            ],
        ),
        (
            ".ml",
            &[
                Rule {
                    languages: &[FileType::OCaml],
                    pattern: Some(Pattern::Positive(
                        "(^\\s*module)|let rec |match\\s+(\\S+\\s)+with",
                    )),
                },
                Rule {
                    languages: &[FileType::Sml],
                    pattern: Some(Pattern::Positive("=> |case\\s+(\\S+\\s)+of")),
                },
            ],
        ),
        (
            ".pm",
            &[
                Rule {
                    languages: &[FileType::Perl],
                    pattern: Some(Pattern::And(&[
                        Pattern::Negative("^\\s*use\\s+v6\\b"),
                        Pattern::Or(&[
                            Pattern::Positive("\\buse\\s+(?:strict\\b|v?5\\b)"),
                            Pattern::Positive("^\\s*use\\s+(?:constant|overload)\\b"),
                            Pattern::Positive("^\\s*(?:\\*|(?:our\\s*)?@)EXPORT\\s*="),
                            Pattern::Positive(
                                "^\\s*package\\s+[^\\W\\d]\\w*(?:::\\w+)*\\s*(?:[;{]|\\sv?\\d)",
                            ),
                            Pattern::Positive("[\\s$][^\\W\\d]\\w*(?::\\w+)*->[a-zA-Z_\\[({]"),
                        ]),
                    ])),
                },
                Rule {
                    languages: &[FileType::Raku],
                    pattern: Some(Pattern::Positive(
                        "^\\s*(?:use\\s+v6\\b|\\bmodule\\b|\\b(?:my\\s+)?class\\b)",
                    )),
                },
            ],
        ),
        (
            ".t",
            &[
                Rule {
                    languages: &[FileType::Perl],
                    pattern: Some(Pattern::And(&[
                        Pattern::Negative("^\\s*use\\s+v6\\b"),
                        Pattern::Or(&[
                            Pattern::Positive("\\buse\\s+(?:strict\\b|v?5\\b)"),
                            Pattern::Positive("^\\s*use\\s+(?:constant|overload)\\b"),
                            Pattern::Positive("^\\s*(?:\\*|(?:our\\s*)?@)EXPORT\\s*="),
                            Pattern::Positive(
                                "^\\s*package\\s+[^\\W\\d]\\w*(?:::\\w+)*\\s*(?:[;{]|\\sv?\\d)",
                            ),
                            Pattern::Positive("[\\s$][^\\W\\d]\\w*(?::\\w+)*->[a-zA-Z_\\[({]"),
                        ]),
                    ])),
                },
                Rule {
                    languages: &[FileType::Raku],
                    pattern: Some(Pattern::Positive(
                        "^\\s*(?:use\\s+v6\\b|\\bmodule\\b|\\bmy\\s+class\\b)",
                    )),
                },
            ],
        ),
        (
            ".es",
            &[
                Rule {
                    languages: &[FileType::Erlang],
                    pattern: Some(Pattern::Positive("^\\s*(?:%%|main\\s*\\(.*?\\)\\s*->)")),
                },
                Rule {
                    languages: &[FileType::JavaScript],
                    pattern: Some(Pattern::Positive(
                        "\\/\\/|[\"']use strict[\"']|export\\s+default\\s|\\/\\*(?:.|[\\r\\n])*?\\*\\/",
                    )),
                },
            ],
        ),
        (".k", &[]),
        (".stl", &[]),
        (
            ".3x",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: None,
                },
            ],
        ),
        (".nl", &[]),
        (
            ".res",
            &[Rule {
                languages: &[FileType::ReScript],
                pattern: Some(Pattern::Or(&[
                    Pattern::Positive("^\\s*(let|module|type)\\s+\\w*\\s+=\\s+"),
                    Pattern::Positive("^\\s*(?:include|open)\\s+\\w+\\s*$"),
                ])),
            }],
        ),
        (".bf", &[]),
        (
            ".asc",
            &[Rule {
                languages: &[FileType::AsciiDoc],
                pattern: Some(Pattern::Positive("^[=-]+\\s|\\{\\{[A-Za-z]")),
            }],
        ),
        (
            ".tact",
            &[Rule {
                languages: &[FileType::Json],
                pattern: Some(Pattern::Positive("\\A\\s*\\{\\\"")),
            }],
        ),
        (".ch", &[]),
        (".s", &[]),
        (
            ".r",
            &[
                Rule {
                    languages: &[FileType::Rebol],
                    pattern: Some(Pattern::Positive("(?i:\\bRebol\\b)")),
                },
                Rule {
                    languages: &[FileType::R],
                    pattern: Some(Pattern::Positive("<-|^\\s*#")),
                },
            ],
        ),
        (
            ".n",
            &[Rule {
                languages: &[FileType::Nroff],
                pattern: Some(Pattern::Positive("^[.']")),
            }],
        ),
        (
            ".gd",
            &[Rule {
                languages: &[FileType::GdScript],
                pattern: Some(Pattern::Positive(
                    "\\s*(extends|var|const|enum|func|class|signal|tool|yield|assert|onready)",
                )),
            }],
        ),
        (
            ".nr",
            &[Rule {
                languages: &[FileType::Nroff],
                pattern: Some(Pattern::Positive("^\\.")),
            }],
        ),
        (".bs", &[]),
        (".srv", &[]),
        (
            ".p",
            &[Rule {
                languages: &[FileType::GnuPlot],
                pattern: Some(Pattern::Or(&[
                    Pattern::Positive("^s?plot\\b"),
                    Pattern::Positive(
                        "^set\\s+(term|terminal|out|output|[xy]tics|[xy]label|[xy]range|style)\\b",
                    ),
                ])),
            }],
        ),
        (
            ".fs",
            &[
                Rule {
                    languages: &[FileType::Forth],
                    pattern: Some(Pattern::Positive("^(: |new-device)")),
                },
                Rule {
                    languages: &[FileType::FSharp],
                    pattern: Some(Pattern::Positive(
                        "^\\s*(#light|import|let|module|namespace|open|type)",
                    )),
                },
                Rule {
                    languages: &[FileType::Glsl],
                    pattern: Some(Pattern::Positive(
                        "^\\s*(#version|precision|uniform|varying|vec[234])",
                    )),
                },
            ],
        ),
        (
            ".nu",
            &[
                Rule {
                    languages: &[FileType::Nu],
                    pattern: Some(Pattern::Positive(
                        "^\\s*(import|export|module|def|let|let-env) ",
                    )),
                },
                Rule {
                    languages: &[FileType::Nu],
                    pattern: None,
                },
            ],
        ),
        (
            ".8",
            &[
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")"),
                        Pattern::Positive(
                            "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive(
                            "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
                        ),
                        Pattern::Positive("^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Nroff],
                    pattern: Some(Pattern::Positive("^\\.(?:[A-Za-z]{2}(?:\\s|$)|\\\\\")")),
                },
                Rule {
                    languages: &[FileType::Text],
                    pattern: None,
                },
            ],
        ),
        (
            ".m",
            &[
                Rule {
                    languages: &[FileType::ObjC],
                    pattern: Some(Pattern::Positive(
                        "^\\s*(@(interface|class|protocol|property|end|synchronised|selector|implementation)\\b|#import\\s+.+\\.h[\">])",
                    )),
                },
                Rule {
                    languages: &[FileType::Mma],
                    pattern: Some(Pattern::Positive("^\\s*;")),
                },
                Rule {
                    languages: &[FileType::Mma],
                    pattern: Some(Pattern::And(&[
                        Pattern::Positive("\\(\\*"),
                        Pattern::Positive("\\*\\)$"),
                    ])),
                },
                Rule {
                    languages: &[FileType::Matlab],
                    pattern: Some(Pattern::Positive("^\\s*%")),
                },
            ],
        ),
        (
            ".for",
            &[
                Rule {
                    languages: &[FileType::Forth],
                    pattern: Some(Pattern::Positive("^: ")),
                },
                Rule {
                    languages: &[FileType::Fortran],
                    pattern: Some(Pattern::Positive(
                        "^(?i:[c*][^abd-z]|      (subroutine|program|end|data)\\s|\\s*!)",
                    )),
                },
            ],
        ),
    ],
};
