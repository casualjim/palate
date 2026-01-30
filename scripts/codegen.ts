#!/usr/bin/env -S bun
/**
 * Convert Neovim's filetype.lua to Rust code for palate
 * This script reads from Neovim's runtime/lua/vim/filetype.lua
 * and generates Rust files in the base_dir
 */

import { TFT_ONLY_PATTERNS } from "./tft-only-patterns";


const neovimFile = "/home/ivan/github/neovim/neovim/runtime/lua/vim/filetype.lua";
const neovimDetectFile = "/home/ivan/github/neovim/neovim/runtime/lua/vim/filetype/detect.lua";
const baseDir = "/home/ivan/github/casualjim/palate";

// Ground truth: grammars.json from breeze-tree-sitter-parsers
const grammarsJsonPath = "/home/ivan/github/casualjim/breeze-tree-sitter-parsers/grammars.json";
const grammarsMappingPath = "/home/ivan/github/casualjim/palate/target/grammars-mapping-enhanced.json";

// Reference mapping extracted from rubixdev/tft/src/list.rs
// This ensures consistency with the existing codebase
const REFERENCE_MAPPING: Record<string, string> = {
  "8th": "Eighth",
  "a2ps": "A2ps",
  "a65": "A65",
  "aap": "Aap",
  "abap": "Abap",
  "abaqus": "Abaqus",
  "abc": "Abc",
  "abel": "Abel",
  "acedb": "Acedb",
  "ada": "Ada",
  "ahdl": "Ahdl",
  "aidl": "Aidl",
  "alsaconf": "AlsaConf",
  "aml": "Aml",
  "ampl": "Ampl",
  "ant": "Ant",
  "apache": "Apache",
  "apachestyle": "ApacheStyle",
  "applescript": "AppleScript",
  "aptconf": "AptConf",
  "arch": "Arch",
  "arduino": "Arduino",
  "art": "Art",
  "asciidoc": "AsciiDoc",
  "asm": "Asm",
  "asn": "Asn",
  "aspperl": "AspPerl",
  "aspvbs": "AspVbs",
  "asterisk": "Asterisk",
  "asteriskvm": "AsteriskVoiceMail",
  "astro": "Astro",
  "atlas": "Atlas",
  "autohotkey": "AutoHotKey",
  "autoit": "AutoIt",
  "automake": "Automake",
  "ave": "Ave",
  "awk": "Awk",
  "bash": "Bash",
  "basic": "Basic",
  "bass": "Bass",
  "bat": "Bat",
  "bc": "Bc",
  "bdf": "Bdf",
  "beancount": "BeanCount",
  "bib": "Bib",
  "bicep": "Bicep",
  "bindzone": "Bindzone",
  "bitbake": "Bitbake",
  "blank": "Blank",
  "blueprint": "Blueprint",
  "bsdl": "Bsdl",
  "bst": "Bst",
  "btm": "Btm",
  "bzl": "Bzl",
  "bzr": "Bzr",
  "c": "C",
  "cabal": "Cabal",
  "cabalconfig": "CabalConfig",
  "cabalproject": "CabalProject",
  "cairo": "Cairo",
  "calendar": "Calendar",
  "capnp": "Capnp",
  "catalog": "Catalog",
  "cdc": "Cdc",
  "cdl": "Cdl",
  "cdrdaoconf": "CdrdaoConf",
  "cdrtoc": "Cdrtoc",
  "cf": "Cf",
  "cfengine": "CfEngine",
  "cfg": "Cfg",
  "ch": "Ch",
  "chaiscript": "ChaiScript",
  "change": "Change",
  "changelog": "Changelog",
  "chaskell": "Chaskell",
  "chatito": "Chatito",
  "chill": "Chill",
  "chordpro": "Chordpro",
  "cl": "Cl",
  "clean": "Clean",
  "clipper": "Clipper",
  "clojure": "Clojure",
  "cmake": "CMake",
  "cmakecache": "CMakeCache",
  "cmod": "CMod",
  "cmusrc": "Cmusrc",
  "cobol": "Cobol",
  "coco": "Coco",
  "conaryrecipe": "ConaryRecipe",
  "conf": "Conf",
  "config": "Config",
  "confini": "ConfIni",
  "ini": "ConfIni",
  "context": "Context",
  "cook": "Cook",
  "coq": "Coq",
  "corn": "Corn",
  "cpon": "Cpon",
  "cqlang": "Cqlang",
  "crm": "Crm",
  "crontab": "Crontab",
  "crystal": "Crystal",
  "csc": "Csc",
  "csdl": "Csdl",
  "csh": "Csh",
  "csharp": "CSharp",
  "dsp": "Faust",
  "csp": "Csp",
  "css": "Css",
  "csv": "Csv",
  "cterm": "CTerm",
  "cucumber": "Cucumber",
  "cuda": "Cuda",
  "cue": "Cue",
  "cupl": "Cupl",
  "cuplsim": "Cuplsim",
  "cvs": "Cvs",
  "cvsrc": "Cvsrc",
  "cweb": "Cweb",
  "cynpp": "Cynpp",
  "d": "D",
  "dart": "Dart",
  "datascript": "DataScript",
  "dcd": "Dcd",
  "dcl": "Dcl",
  "deb822sources": "DebSources",
  "debchangelog": "DebChangelog",
  "debcontrol": "DebControl",
  "debcopyright": "DebCopyright",
  "debsources": "DebSources",
  "def": "Def",
  "denyhosts": "DenyHosts",
  "dep3patch": "Dep3Patch",
  "desc": "Desc",
  "desktop": "Desktop",
  "dhall": "Dhall",
  "dictconf": "DictConf",
  "dictdconf": "DictdConf",
  "diff": "Diff",
  "dircolors": "DirColors",
  "diva": "Diva",
  "dnsmasq": "DnsMasq",
  "docbk-sgml-4": "DocBookSgml4",
  "docbk-xml-4": "DocBookXml4",
  "docbk-xml-5": "DocBookXml5",
  "dockerfile": "Dockerfile",
  "dosbatch": "DosBatch",
  "dosini": "DosIni",
  "dot": "Dot",
  "dracula": "Dracula",
  "dsl": "Dsl",
  "dtd": "Dtd",
  "dtrace": "DTrace",
  "dts": "Dts",
  "dune": "Dune",
  "dylan": "Dylan",
  "dylanintr": "DylanIntr",
  "dylanlid": "DylanLid",
  "ebnf": "Ebnf",
  "ecd": "Ecd",
  "edif": "Edif",
  "editorconfig": "EditorConfig",
  "eelixir": "EElixir",
  "eiffel": "Eiffel",
  "eighth": "Eighth",
  "ejavascript": "EJavaScript",
  "elf": "Elf",
  "elinks": "ELinks",
  "elixir": "Elixir",
  "elm": "Elm",
  "elmfilt": "ElmFilt",
  "elsa": "Elsa",
  "elvish": "Elvish",
  "epuppet": "EPuppet",
  "erlang": "Erlang",
  "eruby": "ERuby",
  "esdl": "Esdl",
  "esmtprc": "Esmtprc",
  "esqlc": "Esqlc",
  "esterel": "Esterel",
  "eterm": "Eterm",
  "euphoria3": "Euphoria3",
  "exim": "Exim",
  "expect": "Expect",
  "exports": "Exports",
  "factor": "Factor",
  "falcon": "Falcon",
  "fan": "Fan",
  "fennel": "Fennel",
  "fetchmail": "FetchMail",
  "fgl": "Fgl",
  "firrtl": "Firrtl",
  "fish": "Fish",
  "foam": "Foam",
  "focexec": "FocExec",
  "form": "Form",
  "forth": "Forth",
  "fortran": "Fortran",
  "fpcmake": "FpcMake",
  "framescript": "FrameScript",
  "freebasic": "FreeBasic",
  "fsh": "Fsh",
  "fsharp": "FSharp",
  "fstab": "FsTab",
  "func": "Func",
  "fusion": "Fusion",
  "fvwm": "Fvwm",
  "fvwm-1": "Fvwm1",
  "fvwm-2": "Fvwm2",
  "fvwm2m4": "Fvwm2M4",
  "gdb": "Gdb",
  "gdmo": "Gdmo",
  "gdresource": "GdResource",
  "gdscript": "GdScript",
  "gdshader": "GdShader",
  "gedcom": "Gedcom",
  "gemtext": "GemText",
  "git": "Git",
  "gitattributes": "GitAttributes",
  "gitcommit": "GitCommit",
  "gitconfig": "GitConfig",
  "gitignore": "GitIgnore",
  "gitolite": "GitOlite",
  "gitrebase": "GitRebase",
  "gitsendemail": "GitSendEmail",
  "gkrellmrc": "Gkrellmrc",
  "gleam": "Gleam",
  "glsl": "Glsl",
  "gnash": "Gnash",
  "gnuplot": "GnuPlot",
  "go": "Go",
  "gomod": "GoMod",
  "gosum": "GoSum",
  "gowork": "GoWork",
  "gp": "Gp",
  "gpg": "Gpg",
  "grads": "Grads",
  "graphql": "GraphQl",
  "gretl": "Gretl",
  "groovy": "Groovy",
  "group": "Group",
  "grub": "Grub",
  "gsp": "Gsp",
  "gtkrc": "Gtkrc",
  "gyp": "Gyp",
  "hack": "Hack",
  "haml": "Haml",
  "hamster": "Hamster",
  "handlebars": "Handlebars",
  "hare": "Hare",
  "haskell": "Haskell",
  "haste": "Haste",
  "hastepreproc": "HastePreProc",
  "hb": "Hb",
  "hcl": "Hcl",
  "heex": "Heex",
  "hercules": "Hercules",
  "hex": "Hex",
  "hexdump": "HexDump",
  "hgcommit": "Hgcommit",
  "hjson": "HJson",
  "hlsplaylist": "HlsPlaylist",
  "hog": "Hog",
  "hollywood": "Hollywood",
  "hoon": "Hoon",
  "hostconf": "HostConf",
  "hostsaccess": "HostsAccess",
  "html": "Html",
  "htmldjango": "HtmlDjango",
  "htmlm4": "HtmlM4",
  "httest": "HtTest",
  "i3config": "I3Config",
  "ibasic": "IBasic",
  "icemenu": "IceMenu",
  "icon": "Icon",
  "idl": "Idl",
  "idlang": "Idlang",
  "indent": "Indent",
  "inform": "Inform",
  "initng": "Initng",
  "inittab": "InitTab",
  "installshield": "InstallShield",
  "ipfilter": "IpFilter",
  "iss": "Iss",
  "ist": "Ist",
  "j": "J",
  "jal": "Jal",
  "jam": "Jam",
  "java": "Java",
  "javacc": "JavaCc",
  "javascript": "JavaScript",
  // Neovim uses "javascriptreact" for JSX filetype; canonicalize to "jsx" (Jsx variant).
  "javascriptreact": "Jsx",
  "javascript.glimmer": "JavaScriptGlimmer",
  "jess": "Jess",
  "jgraph": "JGraph",
  "jovial": "Jovial",
  "jproperties": "JProperties",
  "jq": "Jq",
  "json": "Json",
  "json5": "Json5",
  "jsonc": "JsonC",
  "jsonl": "JsonL",
  "jsonnet": "Jsonnet",
  "jsp": "Jsp",
  "jsx": "Jsx",
  "julia": "Julia",
  "kconfig": "KConfig",
  "kdl": "Kdl",
  "kivy": "Kivy",
  "kix": "Kix",
  "kotlin": "Kotlin",
  "krl": "Krl",
  "kscript": "KScript",
  "ksh": "Ksh",
  "kwt": "Kwt",
  "lace": "Lace",
  "lambdaprolog": "LambdaProlog",
  "larch": "Larch",
  "latte": "Latte",
  "ld": "Ld",
  "ldif": "Ldif",
  "lean": "Lean",
  "ledger": "Ledger",
  "less": "Less",
  "lex": "Lex",
  "lftp": "Lftp",
  "lhaskell": "LHaskell",
  "libao": "Libao",
  "lilo": "Lilo",
  "lilypond": "Lilypond",
  "limits": "Limits",
  "liquid": "Liquid",
  "lisp": "Lisp",
  "lite": "Lite",
  "litestep": "LiteStep",
  "livebook": "Livebook",
  "llvm": "Llvm",
  "logcheck": "LogCheck",
  "loginaccess": "LoginAccess",
  "logindefs": "LoginDefs",
  "logtalk": "Logtalk",
  "lotos": "Lotos",
  "lout": "Lout",
  "lpc": "Lpc",
  "lsl": "Lsl",
  "lss": "Lss",
  "lua": "Lua",
  "luau": "Luau",
  "lynx": "Lynx",
  "lyrics": "Lyrics",
  "m3build": "M3Build",
  "m3quake": "M3Quake",
  "m4": "M4",
  "mail": "Mail",
  "mailaliases": "MailAliases",
  "mailcap": "MailCap",
  "make": "Make",
  "mallard": "Mallard",
  "manconf": "ManConf",
  "map": "Map",
  "maple": "Maple",
  "markdown": "Markdown",
  "mason": "Mason",
  "master": "Master",
  "matlab": "Matlab",
  "maxima": "Maxima",
  "mel": "Mel",
  "mermaid": "Mermaid",
  "meson": "Meson",
  "messages": "Messages",
  "mf": "Mf",
  "mgl": "Mgl",
  "mgp": "Mgp",
  "mib": "Mib",
  "mix": "Mix",
  "mma": "Mma",
  "mmix": "Mmix",
  "mmp": "Mmp",
  "modconf": "ModConf",
  "modsim3": "Modsim3",
  "modula2": "Modula2",
  "modula3": "Modula3",
  "monk": "Monk",
  "moo": "Moo",
  "moonscript": "MoonScript",
  "move": "Move",
  "mp": "Mp",
  "mp-metafun": "MpMetafun",
  "mplayerconf": "MPlayerConf",
  "mrxvtrc": "Mrxvtrc",
  "msidl": "Msidl",
  "msmessages": "MsMessages",
  "msql": "Msql",
  "mupad": "Mupad",
  "murphi": "Murphi",
  "mush": "Mush",
  "muttrc": "Muttrc",
  "mysql": "MySql",
  "n1ql": "N1ql",
  "named": "Named",
  "nanorc": "Nanorc",
  "natural": "Natural",
  "ncf": "Ncf",
  "neomuttrc": "Neomuttrc",
  "netrc": "Netrc",
  "nginx": "Nginx",
  "nim": "Nim",
  "ninja": "Ninja",
  "nix": "Nix",
  "nqc": "Nqc",
  "nroff": "Nroff",
  "nsis": "Nsis",
  "nu": "Nu",
  "objc": "ObjC",
  "objcpp": "ObjCpp",
  "obj": "Obj",
  "obse": "Obse",
  "ocaml": "OCaml",
  "ocamlinterface": "OCamlInterface",
  "occam": "Occam",
  "octave": "Octave",
  "odin": "Odin",
  "omnimark": "OmniMark",
  "opam": "Opam",
  "openroad": "OpenRoad",
  "openscad": "OpenScad",
  "openvpn": "OpenVpn",
  "opl": "Opl",
  "ora": "Ora",
  "org": "Org",
  "pamconf": "PamConf",
  "pamenv": "PamEnv",
  "papp": "Papp",
  "pascal": "Pascal",
  "passwd": "Passwd",
  "pbtxt": "Pbtxt",
  "pccts": "Pccts",
  "pcmk": "Pcmk",
  "pdf": "Pdf",
  "pem": "Pem",
  "perl": "Perl",
  "pf": "Pf",
  "pfmain": "PfMain",
  "php": "Php",
  "pike": "Pike",
  "pilrc": "Pilrc",
  "pine": "Pine",
  "pinfo": "PInfo",
  "plaintex": "PlainTex",
  "pli": "Pli",
  "plm": "Plm",
  "plp": "Plp",
  "plsql": "Plsql",
  "po": "Po",
  "pod": "Pod",
  "poefilter": "PoeFilter",
  "poke": "Poke",
  "pony": "Pony",
  "postscr": "Postscr",
  "pov": "Pov",
  "povini": "PovIni",
  "ppd": "Ppd",
  "ppwiz": "Ppwiz",
  "prisma": "Prisma",
  "privoxy": "Privoxy",
  "proc": "Proc",
  "procmail": "ProcMail",
  "progress": "Progress",
  "prolog": "Prolog",
  "promela": "Promela",
  "proto": "Proto",
  "protocols": "Protocols",
  "prql": "Prql",
  "ps1": "Ps1",
  "ps1xml": "Ps1Xml",
  "psf": "Psf",
  "psl": "Psl",
  "ptcap-print": "PtcapPrint",
  "ptcap-term": "PtcapTerm",
  "pug": "Pug",
  "puppet": "Puppet",
  "pyret": "Pyret",
  "pyrex": "Pyrex",
  "python": "Python",
  "qb64": "Qb64",
  "ql": "Ql",
  "qmldir": "QmlDir",
  "quake": "Quake",
  "quarto": "Quarto",
  "r": "R",
  "racc": "Racc",
  "racket": "Racket",
  "radiance": "Radiance",
  "raku": "Raku",
  "raml": "Raml",
  "rapid": "Rapid",
  "ratpoison": "RatPoison",
  "rbs": "Rbs",
  "rc": "Rc",
  "rcs": "Rcs",
  "readline": "Readline",
  "rebol": "Rebol",
  "redif": "Redif",
  "registry": "Registry",
  "rego": "Rego",
  "remind": "Remind",
  "rescript": "ReScript",
  "resolv": "Resolv",
  "reva": "Reva",
  "rexx": "Rexx",
  "rhelp": "RHelp",
  "rib": "Rib",
  "rmd": "Rmd",
  "rnc": "Rnc",
  "rng": "Rng",
  "rnoweb": "Rnoweb",
  "robot": "Robot",
  "robots": "Robots",
  "ron": "Ron",
  "routeros": "RouterOs",
  "rpcgen": "Rpcgen",
  "rpgle": "Rpgle",
  "rpl": "Rpl",
  "rrst": "Rrst",
  "rst": "Rst",
  "rtf": "Rtf",
  "ruby": "Ruby",
  "rush": "Rush",
  "rust": "Rust",
  "sage": "Sage",
  "samba": "Samba",
  "sas": "Sas",
  "sass": "Sass",
  "sather": "Sather",
  "sbt": "Sbt",
  "scala": "Scala",
  "scdoc": "Scdoc",
  "scheme": "Scheme",
  "scilab": "Scilab",
  "screen": "Screen",
  "scss": "Scss",
  "sd": "Sd",
  "sdc": "Sdc",
  "sdl": "Sdl",
  "sed": "Sed",
  "sensors": "Sensors",
  "services": "Services",
  "setserial": "SetSerial",
  "sexplib": "Sexplib",
  "sgml": "Sgml",
  "sgmldecl": "SgmlDecl",
  "sh": "Sh",
  "sieve": "Sieve",
  "sil": "Sil",
  "sile": "Sile",
  "simula": "Simula",
  "sinda": "Sinda",
  "sisu": "Sisu",
  "skill": "Skill",
  "slang": "Slang",
  "slice": "Slice",
  "slpconf": "SlpConf",
  "slpreg": "SlpReg",
  "slpspi": "SlpSpi",
  "slrnrc": "Slrnrc",
  "slrnsc": "Slrnsc",
  "sm": "Sm",
  "smali": "Smali",
  "smarty": "Smarty",
  "smcl": "Smcl",
  "smgllnx": "Smgllnx",
  "smil": "Smil",
  "smith": "Smith",
  "smithy": "Smithy",
  "sml": "Sml",
  "snobol4": "Snobol4",
  "solidity": "Solidity",
  "solution": "Solution",
  "sparql": "Sparql",
  "spec": "Spec",
  "specman": "SpecMan",
  "spice": "Spice",
  "spup": "Spup",
  "spyce": "Spyce",
  "sql": "Sql",
  "sqlj": "Sqlj",
  "sqr": "Sqr",
  "squid": "Squid",
  "squirrel": "Squirrel",
  "srec": "Srec",
  "srt": "Srt",
  "ssa": "Ssa",
  "sshconfig": "SshConfig",
  "sshdconfig": "SshdConfig",
  "st": "St",
  "starlark": "Starlark",
  "stata": "Stata",
  "stp": "Stp",
  "structurizr": "Structurizr",
  "sudoers": "Sudoers",
  "supercollider": "Supercollider",
  "surface": "Surface",
  "svelte": "Svelte",
  "svg": "Svg",
  "svn": "Svn",
  "swayconfig": "SwayConfig",
  "swift": "Swift",
  "swiftgyb": "SwiftGyb",
  "sysctl": "Sysctl",
  "systemd": "Systemd",
  "systemverilog": "SystemVerilog",
  "tablegen": "Tablegen",
  "tads": "Tads",
  "tags": "Tags",
  "tak": "Tak",
  "tal": "Tal",
  "taskdata": "TaskData",
  "taskedit": "TaskEdit",
  "tcl": "Tcl",
  "tcsh": "Tcsh",
  "teal": "Teal",
  "template": "Template",
  "teraterm": "Teraterm",
  "terminfo": "Terminfo",
  "terraform": "Terraform",
  "terraform-vars": "TerraformVars",
  "tex": "Tex",
  "texinfo": "TexInfo",
  "texmf": "TexMF",
  "tf": "Tf",
  "thrift": "Thrift",
  "tidy": "Tidy",
  "tilde": "Tilde",
  "tla": "Tla",
  "tli": "Tli",
  "tmux": "Tmux",
  "toml": "Toml",
  "tpp": "Tpp",
  "trace32": "Trace32",
  "trasys": "Trasys",
  "tsalt": "Tsalt",
  "tsscl": "Tsscl",
  "tssgm": "Tssgm",
  "tssop": "Tssop",
  "tsv": "Tsv",
  "tsx": "Tsx",
  "turtle": "Turtle",
  "tutor": "Tutor",
  "twig": "Twig",
  "typescript": "TypeScript",
  "typescript.glimmer": "TypeScriptGlimmer",
  "typst": "Typst",
  "uc": "Uc",
  "udevconf": "UdevConf",
  "udevperm": "UdevPerm",
  "udevrules": "UdevRules",
  "uil": "Uil",
  "ungrammar": "Ungrammar",
  "updatedb": "UpdateDb",
  "upstart": "Upstart",
  "upstreamdat": "UpstreamDat",
  "upstreaminstalllog": "UpstreamInstallLog",
  "upstreamlog": "UpstreamLog",
  "urlshortcut": "UrlShortcut",
  "ursa": "Ursa",
  "usd": "Usd",
  "usserverlog": "UsServerLog",
  "usw2kagtlog": "Usw2KagtLog",
  "v": "V",
  "vala": "Vala",
  "vb": "Vb",
  "vdf": "Vdf",
  "vdmpp": "Vdmpp",
  "vdmrt": "Vdmrt",
  "vdmsl": "Vdmsl",
  "vera": "Vera",
  "verilog": "SystemVerilog",
  "verilogams": "Verilogams",
  "vgrindefs": "Vgrindefs",
  "vhdl": "Vhdl",
  "vhs": "Vhs",
  "vim": "Vim",
  "help": "VimHelp",
  "viminfo": "VimInfo",
  "virata": "Virata",
  "vmasm": "Vmasm",
  "voscm": "Voscm",
  "vrml": "Vrml",
  "vroom": "Vroom",
  "vto": "Vento",
  "vue": "Vue",
  "wast": "Wast",
  "wat": "Wat",
  "wdl": "Wdl",
  "web": "Web",
  "webmacro": "WebMacro",
  "wget": "Wget",
  "wget2": "Wget2",
  "winbatch": "WinBatch",
  "wit": "Wit",
  "wml": "Wml",
  "wsh": "Wsh",
  "wsml": "Wsml",
  "wvdial": "WvDial",
  "xbl": "Xbl",
  "xdefaults": "XDefaults",
  "xf86conf": "XF86Conf",
  "xf86conf-3": "XF86Conf3",
  "xf86conf-4": "XF86Conf4",
  "xhtml": "Xhtml",
  "xinetd": "Xinetd",
  "xmath": "XMath",
  "xml": "Xml",
  "xmodmap": "XModMap",
  "xpm": "Xpm",
  "xpm2": "Xpm2",
  "xquery": "XQuery",
  "xs": "Xs",
  // Tree-sitter query files
  "query": "TreeSitterQuery",
  "xsd": "Xsd",
  "xslt": "Xslt",
  "yacc": "Yacc",
  "yaml": "Yaml",
  "yang": "Yang",
  "yuck": "Yuck",
  "z8a": "Z8a",
  "zig": "Zig",
  "zimbu": "Zimbu",
  "zimbutempl": "ZimbuTempl",
  "zir": "Zir",
  "zserio": "Zserio",
  "zsh": "Zsh",
};

/**
 * Convert a string to PascalCase following our naming conventions
 */
function toPascalCase(name: string): string {
  // Handle special cases that differ from standard conversion
  const specialCases: Record<string, string> = {
    "c#": "CSharp",
    "c++": "Cpp",
    "f#": "FSharp",
    "8th": "Eighth",
    "objective-c": "ObjC",
    "objective-c++": "ObjCpp",
    "f*": "Fstar",
    "m": "Mma",
    "wolfram language": "Mma",
    "standard ml": "Sml",
    "supercollider": "Supercollider",
    "star": "Starlark",
    "sqlpl": "Plsql",
    "euphoria": "Euphoria3",
    "cairo zero": "Cairo",
    "cs": "CSharp",
    "commonlisp": "Lisp",
    "roff manpage": "Nroff",
    "roff": "Nroff",
    "gnuplot": "GnuPlot",
    "java properties": "JProperties",
    "vim script": "Vim",
    "vim help file": "VimHelp",
    "hosts file": "HostsAccess",
    "tex": "Tex",
    "plpgsql": "Plsql",
    "tsql": "Sql",
    "hiveql": "Sql",
    "glimmer ts": "JavaScriptGlimmer",
    "nushell": "Nu",
    "ini": "ConfIni",
    "confini": "ConfIni",
    "stringtemplate": "Template",
    "javascript.glimmer": "JavaScriptGlimmer",
    "typescript.glimmer": "TypeScriptGlimmer",
    "htmlangular": "Angular",
    "tla": "Tla",
    "ps1": "Ps1",
  };

  const lower = name.toLowerCase();
  if (specialCases[lower]) {
    return specialCases[lower];
  }

  // Default conversion: split on non-alphanumeric and capitalize each part
  return name
    .replace(/[-#]/g, "_")
    .split("_")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join("");
}

type GrammarInfo = {
  variant: string;
  nvimFiletype: string | null;
};

type ParsedValue =
  | { kind: "static"; filetype: string }
  | { kind: "detect"; functionName: string }
  | { kind: "closure"; expr: string }
  | { kind: "starsetf_static"; filetype: string }
  | { kind: "starsetf_detect"; functionName: string }
  | { kind: "inline_function" }
  | { kind: "unknown" };

type ParseOverrides = {
  customClosureByKey: Record<string, string>;
  inlineFunctionDetectByKey: Record<string, string>;
  staticFallbackByKey: Record<string, string>;
  // Per-key remapping for literal string filetypes like `mli = 'ocaml'`.
  // This is intentionally tiny and only used where we want to be *more specific*
  // than Neovim's table without turning the whole generator into an overrides zoo.
  staticValueOverrideByKey: Record<string, Record<string, string>>;
};

function buildGrammarInfo(
  grammarsData: any,
  grammarMappingByName: Map<string, any>,
) {
  const grammarInfoByName: Record<string, GrammarInfo> = {};
  const variantByFiletype: Record<string, string> = {};
  const filetypesFromGrammars = new Set<string>();

  for (const grammar of grammarsData.grammars) {
    const mapping = grammarMappingByName.get(grammar.name);
    const nvimFiletype = mapping?.nvim_filetype || mapping?.effective_filetype || null;
    const variant = toPascalCase(grammar.name);

    grammarInfoByName[grammar.name] = {
      variant,
      nvimFiletype,
    };

    // Variant resolution should work for both the grammar name and the mapped filetype.
    variantByFiletype[grammar.name] = variant;
    if (nvimFiletype) {
      variantByFiletype[nvimFiletype] = variant;
      filetypesFromGrammars.add(nvimFiletype);
    } else {
      filetypesFromGrammars.add(grammar.name);
    }
  }

  return { grammarInfoByName, variantByFiletype, filetypesFromGrammars };
}

// Read the grammars.json and mapping (ground truth)
const grammarsData = JSON.parse(await Bun.file(grammarsJsonPath).text());
const grammarsMapping = JSON.parse(await Bun.file(grammarsMappingPath).text());

console.log(`📦 Loaded ${grammarsData.grammars.length} grammars from grammars.json`);

const grammarMappingByName = new Map<string, any>();
for (const mapping of grammarsMapping) {
  grammarMappingByName.set(mapping.grammar, mapping);
}

const {
  grammarInfoByName,
  variantByFiletype,
  filetypesFromGrammars,
} = buildGrammarInfo(grammarsData, grammarMappingByName);

console.log(`📋 Built info map for ${Object.keys(grammarInfoByName).length} grammars`);

// Read the Neovim filetype.lua file
const content = await Bun.file(neovimFile).text();

// NOTE: We no longer parse nvim-treesitter parsers.lua here because:
// 1. The regex-based parsing was buggy and caused corruption
// 2. We now use the grammar info derived from grammars.json + grammars-mapping.json as the ground truth
// 3. The REFERENCE_MAPPING provides additional legacy mappings for compatibility

// Extract sections using the -- BEGIN/-- END markers that exist in the file
const extMatch = content.match(/-- BEGIN EXTENSION\n(.*)\n  -- END EXTENSION/s);
const filenameMatch = content.match(/-- BEGIN FILENAME\n(.*)\n  -- END FILENAME/s);
const patternMatch = content.match(/-- BEGIN PATTERN\n(.*)\n  -- END PATTERN/s);

if (!extMatch || !filenameMatch || !patternMatch) {
  console.error("Cannot extract table content from filetype.lua");
  process.exit(1);
}

const extContent = extMatch[1];
const filenameContent = filenameMatch[1];
const patternContent = patternMatch[1];

// Read detect.lua to find detect function signatures
const detectContent = await Bun.file(neovimDetectFile).text();

// Extract all detect function names from Neovim's detect.lua
const detectFunctions = new Set<string>();
for (const match of detectContent.matchAll(/function M\.(\w+)\(/g)) {
  detectFunctions.add(match[1]);
}

// Also add some detect functions that are defined inline in filetype.lua
const inlineDetect = new Set(["line1", "noext", "rc", "seq"]);
for (const name of inlineDetect) {
  detectFunctions.add(name);
}

// NOTE: detectFunctions is currently collected for parity/debugging; emission is driven by
// availableDetectFunctions to match the Rust implementation signatures.

// List of detect functions that actually exist in src/detect/mod.rs
// Only generate dynamic entries for these functions to avoid compilation errors
// Exclude functions with non-standard signatures (they take additional parameters)
const availableDetectFunctions = new Set([
  "asa", "asm", "asp", "bak", "bas", "btm", "cfg", "change", "changelog",
  "cls", "cmd", "control", "copyright", "cpp", "cpy", "csh", "dat", "decl", "dep3patch",
  "dsl", "dtrace", "e", "edn", "ent", "euphoria", "ex", "foam", "frm", "fs", "fvwm",
  "git", "header", "hook", "html", "hw", "idl", "in_", "inc", "inp", "install",
  "lpc", "lsl", "m", "m4_ext", "make", "markdown", "mc", "me", "mm", "mms", "mod_", "mp", "news", "nroff",
  "patch", "perl", "pl", "pm", "pp", "prg", "progress_asm", "progress_cweb", "progress_pascal", "psf", "r", "rc", "redif",
  "reg", "rul", "rules", "sc", "scd", "sgml", "sig", "sil", "smi",
  "smil", "sql", "src", "sys", "tex", "tf", "tmp", "ts", "ttl", "txt", "typ",
  "v", "web", "xfree86", "xml", "xpm", "y",
  // Helper functions used by detect functions (not for direct use)
  "is_lprolog", "is_rapid",
  // NOTE: Excluded functions with non-standard signatures:
  // - sh, shell: take (content: &str, dialect: Option/FileType) instead of (path, content)
  // - bindzone, proto: take (content: &str, default: Option/FileType) instead of (path, content)
]);

// Mapping from Neovim detect function names to Rust detect function names
// This handles cases where the function name differs or doesn't exist
const detectFunctionMapping: Record<string, string> = {
  // Shell functions - sh has a different signature, skip for now
  "bash": "SKIP_me",
  "tcsh": "SKIP_me",
  "zsh": "SKIP_me",
  "ksh": "SKIP_me",
  // XFree86 versions
  "xfree86_v3": "xfree86",
  "xfree86_v4": "xfree86",
  // Rust keyword mappings
  "mod": "mod_",
  "in": "in_",
  // Other function name mappings
  "m4": "m4_ext",
  "i": "progress_asm",     // detect.i -> progress_asm
  "dep3patch": "dep3patch",
  "fvwm_v1": "fvwm",
  "fvwm_v2": "fvwm",
  // detect_noext strips extension and re-detects -> use bak function
  "noext": "bak",
  // These functions don't exist yet - skip
  "app": "SKIP_me",
  "def": "SKIP_me",
  "line1": "SKIP_me",
  "f": "SKIP_me",
  "uci": "SKIP_me",
};

/**
 * Get the actual Rust function name for a Neovim detect function name
 */
function getDetectFunctionName(name: string): string {
  if (detectFunctionMapping[name]) {
    return detectFunctionMapping[name];
  }
  return name;
}

/**
 * Check if a detect function is available in the Rust code
 */
function isDetectFunctionAvailable(name: string): boolean {
  // Skip functions marked with SKIP_me prefix
  if (name.startsWith("SKIP_")) {
    return false;
  }
  return availableDetectFunctions.has(name);
}

/**
 * Manual extension overrides from the reference implementation
 * These entries exist in the reference but not in current Neovim
 * Format: "extension": ["static"|"dynamic", "variant_or_function"]
 */
const MANUAL_OVERRIDES: Record<string, ["static" | "dynamic", string]> = {
  // Entries from reference that aren't in current Neovim but should be preserved
  "ebnf": ["static", "Ebnf"],
  "ejs": ["static", "EJavaScript"],
  "hexdump": ["static", "HexDump"],
  "llvm": ["static", "Llvm"],
  "m2": ["static", "Modula2"],
  "mi": ["static", "Modula2"],
  "rpmnew": ["dynamic", "bak"],
  "rush": ["static", "Rush"],
  "ursa": ["static", "Ursa"],
  // Treat Verilog as SystemVerilog (single parser/grammar).
  "verilog": ["static", "SystemVerilog"],
  "vh": ["static", "SystemVerilog"],
  "vlg": ["static", "SystemVerilog"],
  "zir": ["static", "Zir"],
};

/**
 * Manual filename overrides from the reference implementation.
 *
 * These are entries that exist in tft but are not present in Neovim's filetype.lua
 * filename table (or need a different resolver type).
 *
 * Format: "filename": ["static"|"detect"|"closure", filetype_or_expr]
 */
const MANUAL_FILENAME_OVERRIDES: Record<string, ["static" | "detect" | "closure", string]> = {
  ".gnuplot": ["static", "gnuplot"],
  "config.nu": ["static", "nu"],
  "env.nu": ["static", "nu"],
  ".env": ["closure", "|_, content| detect::sh(content, None)"],
  "printcap": ["static", "ptcap-print"],
  "termcap": ["static", "ptcap-term"],
  "xorg.conf": ["static", "xf86conf-4"],
  "xorg.conf-4": ["static", "xf86conf-4"],
};

/**
 * Manual path suffix overrides from the reference implementation.
 *
 * These are entries that exist in tft but are not present in Neovim's filetype.lua
 * path table. They are location-based (so low ambiguity) and match the same
 * resulting filetype we would return for the corresponding filename anyway.
 */
const MANUAL_PATH_SUFFIX_OVERRIDES: Record<string, ["static", string]> = {
  "etc/pacman.conf": ["static", "confini"],
  "etc/zsh/zprofile": ["static", "zsh"],
};

/**
 * Manual pattern overrides from the reference implementation.
 *
 * Format: [rust_pattern, "static"|"dynamic"|"closure", variant_or_function, priority?, match_full_path?]
 * - `variant_or_function`:
 *   - static: Rust enum variant name (e.g. "Apache", "XDefaults")
 *   - dynamic: detect function name (e.g. "dep3patch")
 *   - closure: full Rust closure expression (e.g. "|_, content| detect::sh(content, None)")
 * - priority:
 *   - "starsetf" to generate Pattern::starsetf(..., None)
 *   - number to generate Pattern::new(..., Some(number)) (negative numbers force post-extension phase)
 *   - undefined to generate Pattern::new(..., None)
 * - match_full_path: overrides haystack selection; if omitted we use a best-effort heuristic.
 *
 * NOTE: `rust_pattern` values are already Rust regex format, not Lua pattern format.
 */
export type ManualPatternEntry = [
  string,
  "static" | "dynamic" | "closure",
  string,
  ("starsetf" | number)?,
  boolean?,
];

const MANUAL_PATTERNS: ManualPatternEntry[] = [
  // Patterns from reference that aren't in current Neovim
  ["^.*\\.git/.*$", "dynamic", "git", -1, true],
  ["^.*\\.[Ll][Oo][Gg]$", "dynamic", "log", undefined, true],
  ["^.+~$", "dynamic", "tmp", undefined, false],
  ...TFT_ONLY_PATTERNS,
];

// Track all filetypes for the enum.
// Sources are added in stages: manual overrides, grammars, Neovim tables, Helix tables,
// detect return types, and reference mappings.
const filetypes = new Set<string>();

const extEntries: Array<[string, string, string]> = [];
const filenameEntries: Array<[string, string, string]> = [];

// Track seen keys to avoid duplicates from the source data
const seenExtensions = new Set<string>();
const seenFilenames = new Set<string>();

// Add manual override filetypes to ensure they're included in the enum
for (const [key, [type, value]] of Object.entries(MANUAL_OVERRIDES)) {
  if (type === "static") {
    filetypes.add(key);
  }
  // Track manual overrides to avoid duplicates from source data
  seenExtensions.add(key);
}

// Add all grammars from grammars.json (ground truth)
// This ensures all 348 grammars are included in the generated list.rs
for (const filetype of filetypesFromGrammars) {
  filetypes.add(filetype);
}

console.log(`Added ${Object.keys(grammarInfoByName).length} grammars to filetypes set`);

// Add effective_filetypes from grammarsMapping to ensure languages like "cs" and "tsx" are included
for (const mapping of grammarsMapping) {
  if (mapping.effective_filetype && !filetypes.has(mapping.effective_filetype)) {
    filetypes.add(mapping.effective_filetype);
  }
}

/**
 * Convert filetype name to Rust enum variant name (PascalCase)
 * Uses the reference mapping from rubixdev/tft for consistency.
 * Precedence: reference mapping -> grammar mapping -> fallback PascalCase.
 */
function toFtVariant(str: string): string {
  // First, check the reference mapping
  if (REFERENCE_MAPPING[str]) {
    return REFERENCE_MAPPING[str];
  }

  const variantFromGrammar = variantByFiletype[str];
  if (variantFromGrammar) {
    return variantFromGrammar;
  }

  // Fallback: simple PascalCase conversion for unknown filetypes
  // Replace dots, hyphens, and # with underscores for processing
  let result = str.replace(/[.%\-#]/g, "_");

  // Split by underscore, capitalize each part, then join
  const parts = result.split("_").filter((p) => p.length > 0);
  result = parts.map((part) => part.charAt(0).toUpperCase() + part.slice(1)).join("");

  return result;
}

/**
 * Convert Lua pattern to Rust regex
 */
function luaToRustPattern(pattern: string): string {
  let result = "";

  // Handle leading / - in Neovim patterns this means full path match.
  // We strip it here; callers decide whether to anchor with ^.*.
  if (pattern.startsWith("/")) {
    pattern = pattern.slice(1);
  }

  let i = 0;

  while (i < pattern.length) {
    const char = pattern[i];
    const nextChar = pattern[i + 1];

    // Check for Lua wildcard: .* (any char, 0+ times) -> same in Rust
    if (char === "." && nextChar === "*") {
      result += ".*";
      i += 2;
      continue;
    }

    if (char === "%" && nextChar) {
      switch (nextChar) {
        case "%":
          result += "%%";
          i += 2;
          break;
        case "d":
          result += "[0-9]";
          i += 2;
          break;
        case "s":
          result += "\\s";
          i += 2;
          break;
        case "w":
          result += "[0-9a-zA-Z_]";
          i += 2;
          break;
        case "a":
          result += "[a-zA-Z]";
          i += 2;
          break;
        case "l":
          result += "[a-z]";
          i += 2;
          break;
        case "u":
          result += "[A-Z]";
          i += 2;
          break;
        case "-":
          // Skip Lua's single-char match
          i += 2;
          break;
        default:
          // For other % escapes (like %.), convert to Rust \\
          result += "\\" + nextChar;
          i += 2;
          break;
      }
    } else {
      // Regular character - check if it needs escaping for Rust
      // Note: $ and ^ are anchors, don't escape them
      // . in Lua is "any character", same as Rust, so don't escape
      // * in Lua (when not after a char) is literal *, need to escape
      // IMPORTANT: Lua `[...]` is a character class; keep `[` and `]` unescaped so it stays a regex class.
      // Literal brackets should be written as `%[` / `%]` in Lua, which we translate to `\[` / `\]` above.
      const rustMagic = ["(", ")", "{", "}", "|", "+", "?", "*"];
      if (rustMagic.includes(char)) {
        result += "\\" + char;
      } else {
        result += char;
      }
      i += 1;
    }
  }

  return result;
}

function finalizeRustPatternForNvimMatch(rustPattern: string): string {
  // tft-style: treat patterns as matching anywhere in the chosen haystack.
  // If the pattern already has a start anchor (`^`), keep it.
  if (rustPattern.startsWith("^")) {
    return rustPattern;
  }
  return `^.*${rustPattern}`;
}

/**
 * Mapping of inline function keys to their corresponding detect functions
 * These are entries in Neovim's filetype.lua that use inline functions
 * but have corresponding detect functions implemented in Rust
 */
const INLINE_FUNCTION_DETECT_BY_KEY: Record<string, string> = {
  "asa": "asa",
  "btm": "btm",
  "hook": "hook",
  "in": "in_",
};

/**
 * Mapping of detect.* functions that don't exist in Rust but have static fallback values
 * Format: "key": "fallback_filetype"
 * The keys are the table keys from Neovim (mostly extensions).
 */
const STATIC_FALLBACK_BY_KEY: Record<string, string> = {
  // Local inline functions that don't exist as detect.*
  "rc": "rc",  // detect_rc is a local inline function
  "rch": "rc", // detect_rc is a local inline function

  // detect.vba returns 'vim' or 'vb' - use 'vim' as fallback (matching reference)
  "vba": "vim",
  // detect.def returns various things - use 'def' as fallback
  "def": "def",

  // detect.cl returns 'opencl' or 'lisp' - use 'lisp' as fallback
  "cl": "lisp",

  // Detect functions that should use static mappings (matching reference)
  "ll": "llvm",       // reference uses Static(Llvm) not Dynamic(lsl)
  "mak": "make",      // reference uses Static(Make) not Dynamic(make)
  "sa": "sather",     // reference uses Static(Sather) not Dynamic(asa)
  "markdown": "markdown",  // reference uses Static(Markdown) not Dynamic(markdown)
  "md": "markdown",
  "mdown": "markdown",
  "mdwn": "markdown",
  "mkdn": "markdown",
  "mkd": "markdown",
  "mk": "make",
  // mp* extensions all resolve to MpMetafun - use static (matching reference)
  "mpiv": "mp-metafun",
  "mpvi": "mp-metafun",
  "mpxl": "mp-metafun",

  // Detect functions that don't exist in Rust yet
  "class": "stata",
  "dsp": "make",
  "f": "fortran",
};

/**
 * Mapping of keys that need custom closures
 * These are for detect functions with non-standard signatures or custom fallbacks.
 */
const CUSTOM_CLOSURE_BY_KEY: Record<string, string> = {
  // bindzone has signature: fn(content: &str, default: Option<FileType>) -> Option<FileType>
  "com": '|_, content| detect::bindzone(content, Some(FileType::Dcl))',
  "db": '|_, content| detect::bindzone(content, None)',

  // Disambiguate ambiguous extensions using content sniffing.
  // Neovim maps these to a single filetype, but the extensions are overloaded in the wild.
  // We prefer correctness via cheap content checks over hardcoding a single winner.
  "comp": "detect::comp",
  "lib": "detect::lib",

  // decl has fallback to Clean
  "dcl": '|path, content| detect::decl(path, content).or(Some(FileType::Clean))',

  // nroff with fallback to XMath
  "ms": '|path, content| detect::nroff(path, content).or(Some(FileType::XMath))',

  // sh has signature: fn(content: &str, dialect: Option<FileType>) -> Option<FileType>
  "bash": '|_, content| detect::sh(content, Some(FileType::Bash))',
  "ebuild": '|_, content| detect::sh(content, Some(FileType::Bash))',
  "eclass": '|_, content| detect::sh(content, Some(FileType::Bash))',
  "env": '|_, content| detect::sh(content, None)',
  "ksh": '|_, content| detect::sh(content, Some(FileType::Ksh))',
  "sh": '|_, content| detect::sh(content, None)',

  // proto has signature: fn(content: &str, default: FileType) -> Option<FileType>
  "pro": '|_, content| detect::proto(content, FileType::Idlang)',

  // shell has signature: fn(content: &str, dialect: FileType) -> Option<FileType>
  "tcsh": '|_, content| detect::shell(content, FileType::Tcsh)',

  // t with complex fallback chain
  "t": '|path, content| detect::nroff(path, content).or_else(|| detect::perl(path, content)).or(Some(FileType::Tads))',
};

const PARSE_OVERRIDES: ParseOverrides = {
  customClosureByKey: CUSTOM_CLOSURE_BY_KEY,
  inlineFunctionDetectByKey: INLINE_FUNCTION_DETECT_BY_KEY,
  staticFallbackByKey: STATIC_FALLBACK_BY_KEY,
  staticValueOverrideByKey: {
    // Neovim sometimes uses broader filetypes for specific extensions. We keep a
    // very small set of "more specific" refinements here (and ONLY here) so we
    // can regenerate consistently without manual edits to generated Rust files.
    "mli": { "ocaml": "ocamlinterface" },
    "wast": { "wat": "wast" },
    "at": { "config": "m4" },
    "mom": { "groff": "nroff" },
  },
};

/**
 * Parse a value from the table
 */
function parseValue(value: string, key: string, overrides: ParseOverrides): ParsedValue {
  // Custom closures by key (used for non-standard detect signatures).
  if (key in overrides.customClosureByKey) {
    return { kind: "closure", expr: overrides.customClosureByKey[key] };
  }

  // Check for inline function - some have corresponding detect functions
  if (value.match(/^function\s*\(/)) {
    if (key in overrides.inlineFunctionDetectByKey) {
      const detectFunc = overrides.inlineFunctionDetectByKey[key];
      if (isDetectFunctionAvailable(detectFunc)) {
        return { kind: "detect", functionName: detectFunc };
      }
    }
    return { kind: "inline_function" };
  }

  // Check for detect_line1 - prefer detect function if available, otherwise use fallback as static
  // detect_line1(pat, a, b) returns 'a' if line 1 matches pat, otherwise 'b'
  const line1Match = value.match(/^detect_line1\s*\(/);
  if (line1Match) {
    // Check if there's a detect function for this key
    const rustFuncName = getDetectFunctionName(key);
    if (isDetectFunctionAvailable(rustFuncName)) {
      return { kind: "detect", functionName: rustFuncName };
    }
    // Otherwise, use the fallback value as static mapping
    const fallback = value.match(/['"]([^'"]+)['"]\s*\)\s*$/);
    if (fallback) {
      return { kind: "static", filetype: fallback[1] };
    }
    return { kind: "static", filetype: "text" };
  }

  // Check for detect_seq
  const seqMatch = value.match(/^detect_seq\s*\(/);
  if (seqMatch) {
    const fallback = value.match(/['"]([^'"]+)['"]\s*\)\s*$/);
    if (fallback) {
      return { kind: "static", filetype: fallback[1] };
    }
    return { kind: "static", filetype: "text" };
  }

  // Check for starsetf
  const starsetfMatch = value.match(/^starsetf\s*\(/);
  if (starsetfMatch) {
    // Try quoted string first: starsetf('filetype')
    const inner = value.match(/starsetf\s*\(\s*['"]([^'"]+)['"]\s*\)/);
    if (inner) {
      if (inner[1].match(/^detect\.(\w+)/)) {
        const funcName = inner[1].match(/^detect\.(\w+)/);
        if (funcName) {
          return { kind: "starsetf_detect", functionName: getDetectFunctionName(funcName[1]) };
        }
      }
      return { kind: "starsetf_static", filetype: inner[1] };
    }
    // Try unquoted detect reference: starsetf(detect.uci)
    const detectRef = value.match(/starsetf\s*\(\s*detect\.(\w+)\s*\)/);
    if (detectRef) {
      return { kind: "starsetf_detect", functionName: getDetectFunctionName(detectRef[1]) };
    }
    // Catch other unquoted references: starsetf(apachestyle)
    const plainRef = value.match(/starsetf\s*\(\s*(\w+)\s*\)/);
    if (plainRef) {
      return { kind: "starsetf_static", filetype: plainRef[1] };
    }
    // Inline function - skip this entry
    return { kind: "inline_function" };
  }

  // Check for bare detect function references (detect_noext, detect_rc, etc.)
  if (value.match(/^detect_[a-z_]+$/)) {
    const funcName = value.match(/^detect_([a-z_]+)$/);
    if (funcName) {
      // Try to use the detect function first
      const rustFuncName = getDetectFunctionName(funcName[1]);
      if (isDetectFunctionAvailable(rustFuncName)) {
        return { kind: "detect", functionName: rustFuncName };
      }
      // Only fall back to static if function doesn't exist
      if (key in overrides.staticFallbackByKey) {
        return { kind: "static", filetype: overrides.staticFallbackByKey[key] };
      }
      // Otherwise skip
      return { kind: "inline_function" };
    }
  }

  // Check for detect.* functions
  if (value.match(/^detect\.(\w+)/)) {
    const funcName = value.match(/^detect\.(\w+)/);
    if (funcName) {
      const detectFuncName = funcName[1]; // e.g., "bash" from "detect.bash"
      // Check if this function has a custom closure
      if (detectFuncName in overrides.customClosureByKey) {
        return { kind: "closure", expr: overrides.customClosureByKey[detectFuncName] };
      }
      const rustFuncName = getDetectFunctionName(detectFuncName);
      // Check if this function has a static fallback
      if (key in overrides.staticFallbackByKey) {
        return { kind: "static", filetype: overrides.staticFallbackByKey[key] };
      }
      return { kind: "detect", functionName: rustFuncName };
    }
  }

  // Extract string value from quotes: '8th' or "8th"
  const strValue = value.match(/['"]([^'"]+)['"]/);
  if (strValue) {
    const raw = strValue[1];
    const overridden = overrides.staticValueOverrideByKey[key]?.[raw];
    return { kind: "static", filetype: overridden ?? raw };
  }

  return { kind: "unknown" };
}

// Detect function return values mapping
// Based on the actual detect function implementations in src/detect/mod.rs
const detectReturns: Record<string, Record<string, boolean>> = {
  // Existing mappings
  asm: { asm: true, masm: true, tiasm: true, vmasm: true, nasm: true, yasm: true },
  bas: { basic: true, freebasic: true, qb64: true, vb: true },
  bash: { sh: true },
  ksh: { sh: true },
  sh: { sh: true, csh: true, tcsh: true, zsh: true, ksh: true },
  shell: { sh: true, csh: true, tcsh: true, zsh: true, ksh: true },
  line1: {},
  perl: { perl: true },
  d: { d: true },
  asp: { aspvbs: true, aspperl: true },
  vba: { vb: true },
  ps1: { ps1: true, ps1xml: true },
  updaterlog: { upstreamdat: true },
  sgmldecl: { sgmldecl: true },
  specman: { specman: true },

  // Added mappings from src/detect/mod.rs implementations
  asa: { aspvbs: true },
  bindzone: { bindzone: true },
  btm: { btm: true },
  cfg: { cfg: true, rapid: true },
  change: { change: true, ch: true, chill: true },
  changelog: { changelog: true, debchangelog: true },
  cls: { tex: true, rexx: true, vb: true, st: true },
  cmd: { rexx: true, dosbatch: true },
  cpy: { python: true, cobol: true },
  csh: { csh: true },
  dsl: { dsl: true, structurizr: true },
  dtrace: { d: true, dtrace: true },
  e: { eiffel: true, specman: true },
  edn: { edif: true, clojure: true },
  ent: { cl: true, dtd: true },
  foam: { foam: true },
  frm: { vb: true, form: true },
  fs: { forth: true, fsharp: true },
  fvwm: { fvwm2: true, fvwm2m4: true },
  git: { git: true },
  header: { c: true, objc: true },
  hook: { conf: true },
  html: { html: true, xhtml: true, htmldjango: true },
  hw: { php: true, virata: true },
  idl: { idl: true, msidl: true },
  inc: { aspperl: true, aspvbs: true, php: true, pascal: true, bitbake: true, pov: true },
  inp: { abaqus: true, trasys: true },
  install: { php: true, bash: true },
  lpc: { lpc: true, c: true },
  lsl: { larch: true, lsl: true },
  m: { matlab: true, octave: true, objc: true, mma: true, murphi: true },
  m4_ext: { m4: true },
  mc: { m4: true, msmessages: true },
  me: { nroff: true },
  mm: { objcpp: true, nroff: true },
  mms: { mmix: true, make: true },
  mod_: { gomod: true, lambdaprolog: true, modula2: true, rapid: true, modsim3: true },
  news: { debchangelog: true },
  nroff: { nroff: true },
  patch: { gitsendemail: true, diff: true },
  pl: { prolog: true, perl: true },
  pm: { xpm2: true, xpm: true, perl: true },
  pp: { pascal: true, puppet: true },
  prg: { rapid: true, clipper: true },
  proto: { cpp: true, prolog: true },
  psf: { psf: true },
  r: { r: true, rebol: true, rexx: true },
  rc: { rc: true },
  redif: { redif: true },
  reg: { registry: true },
  rul: { installshield: true, diva: true },
  rules: { udevrules: true, conf: true, javascript: true, hog: true },
  sc: { supercollider: true, scala: true },
  scd: { scdoc: true, supercollider: true },
  sgml: { sgml: true, smgllnx: true, docbookxml4: true },
  sig: { lambdaprolog: true, sml: true },
  sil: { sile: true, sil: true },
  smi: { smil: true, mib: true },
  smil: { xml: true, smil: true },
  sql: { sql: true },
  src: { krl: true },
  sys: { rapid: true, bat: true },
  tf: { terraform: true, tf: true },
  tmp: { text: true },
  ts: { xml: true, smil: true },
  ttl: { turtle: true, teraterm: true },
  txt: { vimhelp: true, text: true },
  typ: { sql: true, typst: true },
  v: { v: true, systemverilog: true, coq: true },
  web: { web: true, winbatch: true },
  xfree86: { xf86conf3: true, xf86conf: true },
  xml: { xml: true, docbookxml4: true, docbookxml5: true, xbl: true },
  xpm: { xpm2: true, xpm: true },
  y: { yacc: true, racc: true },
};

type EntryKind = "static" | "detect" | "closure" | "starsetf_static" | "starsetf_detect";

function addDetectReturnFiletypes(filetypes: Set<string>, detectFunc: string): void {
  const returns = detectReturns[detectFunc];
  if (!returns) {
    return;
  }
  for (const ft of Object.keys(returns)) {
    filetypes.add(ft);
  }
}

function collectFiletypesFromParsedValue(filetypes: Set<string>, parsed: ParsedValue): void {
  switch (parsed.kind) {
    case "static":
    case "starsetf_static":
      filetypes.add(parsed.filetype);
      break;
    case "detect":
    case "starsetf_detect":
      addDetectReturnFiletypes(filetypes, parsed.functionName);
      break;
    default:
      break;
  }
}

function recordEntry(
  entries: Array<[string, EntryKind, string]>,
  key: string,
  parsed: ParsedValue,
  options: {
    seen?: Set<string>;
    allowedKinds?: ReadonlySet<EntryKind>;
    filetypes?: Set<string>;
  },
): void {
  if (parsed.kind === "inline_function" || parsed.kind === "unknown") {
    return;
  }

  const entryKind = parsed.kind as EntryKind;
  if (options.allowedKinds && !options.allowedKinds.has(entryKind)) {
    return;
  }

  if (options.seen && options.seen.has(key)) {
    return;
  }

  switch (parsed.kind) {
    case "static":
      entries.push([key, "static", parsed.filetype]);
      break;
    case "detect":
      entries.push([key, "detect", parsed.functionName]);
      break;
    case "closure":
      entries.push([key, "closure", parsed.expr]);
      break;
    case "starsetf_static":
      entries.push([key, "starsetf_static", parsed.filetype]);
      break;
    case "starsetf_detect":
      entries.push([key, "starsetf_detect", parsed.functionName]);
      break;
  }

  if (options.seen) {
    options.seen.add(key);
  }
  if (options.filetypes) {
    collectFiletypesFromParsedValue(options.filetypes, parsed);
  }
}

// ============================================================================
// Parse extension table
// ============================================================================
for (const line of extContent.split(/\r?\n/)) {
  const trimmed = line.trim();
  if (trimmed === "" || trimmed.startsWith("--")) continue;

  // Match ['ext'] = 'filetype' or ['ext'] = detect.func or ['ext'] = starsetf(...)
  let key: string | null, rest: string | null;

  const match = trimmed.match(/^\['([^']+)'\]\s*=\s*(.+)$/);
  if (match) {
    key = match[1];
    rest = match[2];
  } else {
    const match2 = trimmed.match(/^([\w_]+)\s*=\s*(.+)$/);
    if (match2) {
      key = match2[1];
      rest = match2[2];
    }
  }

  if (key && rest) {
    // Skip if we've already seen this extension (duplicate in source data)
    if (seenExtensions.has(key)) {
      continue;
    }

    // Strip trailing comma
    rest = rest.replace(/,\s*$/, "");
    const parsed = parseValue(rest, key, PARSE_OVERRIDES);
    recordEntry(extEntries, key, parsed, { seen: seenExtensions, filetypes });
  }
}

// ============================================================================
// Helix arrays (will be populated after all Neovim data is parsed)
// ============================================================================
const helixExtensions: Array<[string, string, string]> = [];
const helixFilenames: Array<[string, string, string]> = [];
const helixPathSuffixes: Array<[string, string, string]> = [];
// Helix "file-types" globs can include path separators and wildcard/meta syntax.
// Wildcards are NOT compatible with our PATH_SUFFIX detector (Path::ends_with),
// so any glob-like entry must become a PATTERN instead.
const helixPatterns: Array<[string, string, string, boolean]> = [];

// ============================================================================
// Parse filename table
// ============================================================================
const pathSuffixEntries: Array<[string, string, string]> = [];
const PATH_SUFFIX_ALLOWED_KINDS: ReadonlySet<EntryKind> = new Set([
  "static",
  "detect",
  "closure",
]);

for (const line of filenameContent.split(/\r?\n/)) {
  const trimmed = line.trim();
  if (trimmed === "" || trimmed.startsWith("--")) continue;

  let key: string | null, rest: string | null;

  // Try bracketed pattern first: ['key'] = value
  const match = trimmed.match(/^\['([^']+)'\]\s*=\s*(.+)$/);
  if (match) {
    key = match[1];
    rest = match[2];
  } else {
    // Try bare identifier pattern: key = value
    // This needs to handle keys with dots like bash.bashrc, .env, etc.
    const match2 = trimmed.match(/^([^\s=]+)\s*=\s*(.+)$/);
    if (match2) {
      key = match2[1];
      rest = match2[2];
    }
  }

  if (key && rest) {
    // Skip keys with brackets - these are Lua nested table accesses like vim.b[b].ptcap_type
    if (key.includes("[")) {
      continue;
    }
    // Strip trailing comma
    rest = rest.replace(/,\s*$/, "");
    const parsed = parseValue(rest, key, PARSE_OVERRIDES);

    // Split based on whether the key contains a path separator
    if (key.includes("/")) {
      // Treat root dotfiles like "/.libao" as filenames to match tft.
      if (key.startsWith("/.") && !key.slice(2).includes("/")) {
        const filenameKey = key.slice(1);
        if (!seenFilenames.has(filenameKey)) {
          recordEntry(filenameEntries, filenameKey, parsed, {
            seen: seenFilenames,
            filetypes,
          });
        }
        continue;
      }

      // Path entries go to path_suffix
      // Strip leading slash if present and convert format
      let suffixKey = key.startsWith("/") ? key.slice(1) : key;

      recordEntry(pathSuffixEntries, suffixKey, parsed, {
        allowedKinds: PATH_SUFFIX_ALLOWED_KINDS,
        filetypes,
      });
    } else {
      // Plain filename entries go to filename
      // Skip if we've already seen this filename (duplicate in source data)
      if (seenFilenames.has(key)) {
        continue;
      }

      recordEntry(filenameEntries, key, parsed, { seen: seenFilenames, filetypes });
    }
  }
}

// Add manual filename overrides from reference (override existing entries if present)
for (const [key, [kind, value]] of Object.entries(MANUAL_FILENAME_OVERRIDES)) {
  const existingIndex = filenameEntries.findIndex(([k]) => k === key);
  if (existingIndex >= 0) {
    filenameEntries[existingIndex] = [key, kind, value];
  } else {
    let parsed: ParsedValue;
    if (kind === "static") {
      parsed = { kind: "static", filetype: value };
    } else if (kind === "detect") {
      parsed = { kind: "detect", functionName: value };
    } else {
      parsed = { kind: "closure", expr: value };
    }
    recordEntry(filenameEntries, key, parsed, { seen: seenFilenames, filetypes });
  }
  if (kind === "static") {
    filetypes.add(value);
  }
}

// Add manual path suffix overrides from reference (override existing entries if present)
for (const [key, [, filetype]] of Object.entries(MANUAL_PATH_SUFFIX_OVERRIDES)) {
  const existingIndex = pathSuffixEntries.findIndex(([k]) => k === key);
  if (existingIndex >= 0) {
    pathSuffixEntries[existingIndex] = [key, "static", filetype];
  } else {
    const parsed: ParsedValue = { kind: "static", filetype };
    recordEntry(pathSuffixEntries, key, parsed, { filetypes });
  }
  filetypes.add(filetype);
}

// ============================================================================
// First pass: parse pattern table to collect all filetypes
// ============================================================================
console.log("Parsing pattern table for additional filetypes...");
for (const line of patternContent.split(/\r?\n/)) {
  const trimmed = line.trim();
  if (trimmed === "" || trimmed.startsWith("--") || trimmed === "}") continue;

  const parent = trimmed.match(/^\['([^']+)'\]\s*=\s*\{$/);
  if (!parent) {
    const patternMatch = trimmed.match(/^\['([^']+)'\]\s*=\s*(.+)$/);
    if (patternMatch) {
      let rest = patternMatch[2];
      // Strip trailing comma
      rest = rest.replace(/,\s*$/, "");
      const parsed = parseValue(rest, patternMatch[1], PARSE_OVERRIDES);
      collectFiletypesFromParsedValue(filetypes, parsed);
    }
  }
}

// ============================================================================
// Add Helix file-types (from Helix editor's languages.toml)
// ============================================================================
// Build sets of existing entries for deduplication
const existingExtensions = new Set<string>();
// Add MANUAL_OVERRIDES to existingExtensions
for (const key of Object.keys(MANUAL_OVERRIDES)) {
  existingExtensions.add(key);
}
// Add entries from parsed source data
for (const [ext, , ] of extEntries) {
  existingExtensions.add(ext);
}
const existingFilenames = new Set<string>();
for (const [filename, , ] of filenameEntries) {
  existingFilenames.add(filename);
}
const existingPathSuffixes = new Set<string>();
for (const [suffix, , ] of pathSuffixEntries) {
  existingPathSuffixes.add(suffix);
}

function escapeRegexLiteral(s: string): string {
  // Escape regex metacharacters. Note: '/' is not special in Rust regex.
  return s.replace(/[\\^$.*+?()[\]{}|]/g, "\\$&");
}

function helixGlobToRustRegex(glob: string, matchFullPath: boolean): string {
  // Convert a Helix glob to a Rust regex source string (without surrounding /.../).
  // This is intentionally minimal: it supports `*`, `**`, `?`, and simple `{a,b}` alternation.
  //
  // Important: if matchFullPath=true we later match against the full path string, so `*` should
  // not cross path separators. For filename-only matching, we can treat `*` as `.*`.

  function convert(input: string): string {
    let out = "";
    for (let i = 0; i < input.length; i++) {
      const ch = input[i];

      // `**` -> `.*`
      if (ch === "*" && input[i + 1] === "*") {
        out += ".*";
        i += 1;
        continue;
      }

      if (ch === "*") {
        out += matchFullPath ? "[^/]*" : ".*";
        continue;
      }

      if (ch === "?") {
        out += matchFullPath ? "[^/]" : ".";
        continue;
      }

      // Simple brace alternation: `{a,b}` -> `(?:a|b)`
      if (ch === "{") {
        const end = input.indexOf("}", i + 1);
        if (end !== -1) {
          const body = input.slice(i + 1, end);
          const opts = body.split(",").map((s) => s.trim()).filter(Boolean);
          if (opts.length > 0) {
            out += `(?:${opts.map(escapeRegexLiteral).join("|")})`;
            i = end;
            continue;
          }
        }
        // Fall back to treating `{` as a literal.
        out += "\\{";
        continue;
      }

      // Treat `}` as literal if we see it without a matching `{`.
      if (ch === "}") {
        out += "\\}";
        continue;
      }

      // Keep path separators literal.
      if (ch === "/") {
        out += "/";
        continue;
      }

      out += escapeRegexLiteral(ch);
    }
    return out;
  }

  const body = convert(glob);
  const prefix = matchFullPath ? "^.*" : "^";
  return `${prefix}${body}$`;
}

// Process Helix data (except patterns - those will be added after pattern parsing)
for (const mapping of grammarsMapping) {
  if (!mapping.helix_file_types) continue;

  // Get the effective filetype to use for variant lookup
  const effectiveFiletype = mapping.effective_filetype || mapping.grammar;
  const variant = toFtVariant(effectiveFiletype);

  for (const fileType of mapping.helix_file_types) {
    if (typeof fileType === "string") {
      // Helix uses plain strings for both extensions and (dot)filenames.
      // Many dotfiles like `.zprofile` have an "extension" of `zprofile` in Rust's Path API,
      // but in our detector we want them as *filenames* (to match Neovim/tft and to avoid
      // accidentally classifying `foo.zprofile`).
      const dotfile = `.${fileType}`;
      if (existingFilenames.has(dotfile)) {
        // Prefer the filename mapping; do not also add an extension mapping for `fileType`.
        continue;
      } else {
        // Simple string -> file extension (no leading dot)
        if (!existingExtensions.has(fileType)) {
          helixExtensions.push([fileType, "static", variant]);
          existingExtensions.add(fileType);
        }
      }
    } else if (fileType.glob && typeof fileType.glob === "string") {
      const glob = fileType.glob;

      const hasGlobMeta =
        glob.includes("**") || /[*?{}\[\]]/.test(glob);

      // Determine the type based on the glob pattern.
      // If it has wildcard/meta syntax, it must be a PATTERN (even if it contains `/`).
      if (hasGlobMeta) {
        const matchFullPath = glob.includes("/");
        const regexPattern = helixGlobToRustRegex(glob, matchFullPath);
        helixPatterns.push([regexPattern, "static", variant, matchFullPath]);
      } else if (glob.includes("/")) {
        // Literal path (no wildcards) -> path suffix (e.g., "i3/config")
        if (!existingPathSuffixes.has(glob)) {
          helixPathSuffixes.push([glob, "static", variant]);
          existingPathSuffixes.add(glob);
        }
      } else {
        // No / and no wildcard/meta -> filename (e.g., ".bashrc", "APKBUILD")
        if (!existingFilenames.has(glob)) {
          helixFilenames.push([glob, "static", variant]);
          existingFilenames.add(glob);
        }
      }
    }
  }
}

console.log(`Adding ${helixExtensions.length} Helix file extensions`);
console.log(`Adding ${helixFilenames.length} Helix filenames`);
console.log(`Adding ${helixPathSuffixes.length} Helix path suffixes`);
console.log(`Adding ${helixPatterns.length} Helix patterns (will be added after pattern parsing)`);

// ============================================================================
// Add all filetypes from detectReturns to ensure all detect function return types are included
// ============================================================================
for (const detectFn of Object.keys(detectReturns)) {
  for (const ft of Object.keys(detectReturns[detectFn])) {
    filetypes.add(ft);
  }
}

// Also add all filetypes from REFERENCE_MAPPING to ensure all variants are included
// This handles filetypes that exist in the reference but are not returned by any detect function
for (const ft of Object.keys(REFERENCE_MAPPING)) {
  filetypes.add(ft);
}

// ============================================================================
// Sort filetypes alphabetically and build variant mapping
// ============================================================================
const filetypeList = Array.from(filetypes).sort();

const ftToVariant: Record<string, string> = {};
for (const ft of filetypeList) {
  ftToVariant[ft] = toFtVariant(ft);
}

// Deduplicate variants - if multiple filetypes map to the same variant,
// only use the first one (which will be alphabetically first due to sorting).
// We then override with canonical names where we want specific serialization.
const VARIANT_CANONICAL_NAMES: Record<string, string> = {
  "FSharp": "fsharp",
  "CSharp": "csharp",
  "ConfIni": "ini",
  "Vento": "vto",
  "Faust": "faust",
  "HaskellPersistent": "haskellpersistent",
  "Idris": "idris2",
  "Qmljs": "qml",
  "Slang": "shaderslang",
  "Tsx": "tsx",
  "Jsx": "jsx",
  "Diff": "gitdiff",
};

const variantToFiletype: Record<string, string> = {};
for (const ft of filetypeList) {
  const variant = ftToVariant[ft];
  if (!variantToFiletype[variant]) {
    variantToFiletype[variant] = ft;
  }
}

// Apply canonical name overrides
for (const [variant, canonicalFt] of Object.entries(VARIANT_CANONICAL_NAMES)) {
  if (variantToFiletype[variant]) {
    variantToFiletype[variant] = canonicalFt;
  }
}

// Extra serialize names for variants that need multiple aliases
// Key: variant name, Value: array of additional serialize names (beyond the canonical one)
const VARIANT_EXTRA_SERIALIZES: Record<string, string[]> = {
  "CSharp": ["cs"],
  "Diff": ["diff"],
  "Tsx": ["typescriptreact"],
  "Jsx": ["javascriptreact"],
  // Helix uses `sv`/`svh` for SystemVerilog.
  "SystemVerilog": ["sv", "svh"],
  "Blueprint": ["blp"],
  "Clarity": ["clar"],
  "Elisp": ["el"],
  "Haxe": ["hx"]
};

const uniqueVariants = Object.entries(variantToFiletype).sort((a, b) => a[1].localeCompare(b[1]));

// ============================================================================
// Generate list.rs
// ============================================================================
console.log("Generating src/list.rs...");
const listRsContent = `macro_rules! list {
    ($($(#[$($attr:meta),+])? $variant:ident $(as $as:literal)?),* $(,)?) => {
        /// A non-exhaustive list of text file types.
        ///
        /// The type derives the following traits for convenience. For (de)serialization to/from strings,
        /// lowercase casing is used unless otherwise specified in the variants docs.
        ///
        /// - [\`strum::Display\`]: [\`Display\`](core::fmt::Display) formatting and a \`.to_string()\` method
        /// - [\`strum::AsRefStr\`]: [\`AsRef<str>\`] impl for conversion into \`&str\`
        /// - [\`strum::IntoStaticStr\`]: [\`From<FileType>\`] impl for conversion into \`&'static str\`
        /// - [\`strum::EnumString\`]: [\`FromStr\`](core::str::FromStr) impl for turning strings into the
        ///   corresponding variant
        /// - [\`strum::EnumVariantNames\`]: an associated \`VARIANTS\` constant containing the string names of
        ///   all variants (requires [\`strum::VariantNames\`] to be in scope)
        /// - [\`Clone\`], [\`Copy\`]
        /// - [\`Debug\`]
        /// - [\`Hash\`]
        /// - [\`PartialEq\`], [\`Eq\`]
        /// - [\`Default\`]: the default is [\`FileType::Text\`]
        /// - <span class="stab portability"><code>serde</code></span> [\`serde::Serialize\`]: serialize into
        ///   a string
        /// - <span class="stab portability"><code>serde</code></span> [\`serde::Deserialize\`]: deserialize
        ///   from a string
        #[derive(
            strum::Display,
            strum::AsRefStr,
            strum::IntoStaticStr,
            strum_macros::EnumString,
            strum_macros::VariantNames,
            Clone,
            Copy,
            Debug,
            Hash,
            PartialEq,
            Eq,
            Default,
        )]
        #[strum(serialize_all = "lowercase", use_phf)]
        #[cfg_attr(
            feature = "serde",
            derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
        )]
        #[non_exhaustive]
        pub enum FileType {
            /// A plain text file. This is the default variant. (De)serialized as \`text\`.
            #[default]
            Text,

            $(
                $(#[$($attr),+])?
                #[doc = concat!("(De)serialized as \`", $($as, "\`, **not** \`",)? casey::lower!(stringify!($variant)), "\`")]
                $(#[strum(serialize = $as)])?
                $variant,
            )*
        }
    };
}

list! {
${uniqueVariants
  .filter(([, ft]) => ft !== "text")
  .map(([variant, ft]) => {
    const simpleLower = variant.toLowerCase();
    const extraSerializes = VARIANT_EXTRA_SERIALIZES[variant];
    let attrs = "";
    if (extraSerializes && extraSerializes.length > 0) {
      const serializeAttrs = extraSerializes.map(s => `serialize = "${s}"`).join(", ");
      attrs = `    #[strum(${serializeAttrs})]\n`;
      // Always use explicit "as" when there are extra serializes to ensure canonical name is preserved
      return `${attrs}    ${variant} as "${ft}",\n`;
    }
    if (ft !== simpleLower) {
      return `${attrs}    ${variant} as "${ft}",\n`;
    }
    return `${attrs}    ${variant},\n`;
  })
  .join("")}
}
`;

Bun.write(`${baseDir}/src/list.rs`, listRsContent);

// ============================================================================
// Generate file_extension.rs
// ============================================================================
console.log("Generating src/detect/file_extension.rs...");
const extRsContent = `use phf::{phf_map, Map};

use crate::{detect, FileType, FileTypeResolver};

pub(crate) static FILE_EXTENSION: Map<&'static str, FileTypeResolver> = phf_map! {
${Object.entries(MANUAL_OVERRIDES)
  .map(([key, [type, value]]) => {
    if (type === "static") {
      // value is already the variant name for MANUAL_OVERRIDES
      return `    "${key}" => FileTypeResolver::Static(FileType::${value}),\n`;
    } else {
      return `    "${key}" => FileTypeResolver::Dynamic(detect::${value}),\n`;
    }
  })
  .join("")}${extEntries
  .filter(([, typeValue]) => typeValue === "static")
  .map(([key, , value]) => {
    const variant = ftToVariant[value];
    return `    "${key}" => FileTypeResolver::Static(FileType::${variant}),\n`;
  })
  .join("")}${extEntries
  .filter(([, typeValue]) => typeValue === "detect")
  .filter(([, , value]) => isDetectFunctionAvailable(value))
  .map(([key, , value]) => {
    return `    "${key}" => FileTypeResolver::Dynamic(detect::${value}),\n`;
  })
  .join("")}${extEntries
  .filter(([, typeValue]) => typeValue === "closure")
  .map(([key, , value]) => {
    return `    "${key}" => FileTypeResolver::Dynamic(${value}),\n`;
  })
  .join("")}${helixExtensions
  .map(([key, , value]) => {
    return `    "${key}" => FileTypeResolver::Static(FileType::${value}),\n`;
  })
  .join("")}};
`;

Bun.write(`${baseDir}/src/detect/file_extension.rs`, extRsContent);

// ============================================================================
// Generate filename.rs
// ============================================================================
console.log("Generating src/detect/filename.rs...");
const filenameRsContent = `use phf::{phf_map, Map};

use crate::{detect, FileType, FileTypeResolver};

pub(crate) static FILENAME: Map<&'static str, FileTypeResolver> = phf_map! {
${filenameEntries
  .filter(([, typeValue]) => typeValue === "static")
  .map(([key, , value]) => {
    const variant = ftToVariant[value];
    return `    "${key}" => FileTypeResolver::Static(FileType::${variant}),\n`;
  })
  .join("")}${filenameEntries
  .filter(([, typeValue]) => typeValue === "detect")
  .filter(([, , value]) => isDetectFunctionAvailable(value))
  .map(([key, , value]) => {
    return `    "${key}" => FileTypeResolver::Dynamic(detect::${value}),\n`;
  })
  .join("")}${filenameEntries
  .filter(([, typeValue]) => typeValue === "closure")
  .map(([key, , value]) => {
    return `    "${key}" => FileTypeResolver::Dynamic(${value}),\n`;
  })
  .join("")}${helixFilenames
  .map(([key, , value]) => {
    return `    "${key}" => FileTypeResolver::Static(FileType::${value}),\n`;
  })
  .join("")}};
`;

Bun.write(`${baseDir}/src/detect/filename.rs`, filenameRsContent);

// ============================================================================
// Generate path_suffix.rs
// ============================================================================
console.log("Generating src/detect/path_suffix.rs...");

const pathSuffixRsContent = `use crate::{detect, FileType, FileTypeResolver};

#[rustfmt::skip]
pub(crate) const PATH_SUFFIX: &[(&str, FileTypeResolver)] = &[
${pathSuffixEntries
  .map(([key, typeValue, value]) => {
    if (typeValue === "static") {
      const variant = ftToVariant[value];
      return `    ("${key}", FileTypeResolver::Static(FileType::${variant})),\n`;
    } else if (typeValue === "detect") {
      return `    ("${key}", FileTypeResolver::Dynamic(detect::${value})),\n`;
    } else if (typeValue === "closure") {
      return `    ("${key}", FileTypeResolver::Dynamic(${value})),\n`;
    }
    return "";
  })
  .join("")}${helixPathSuffixes
  .map(([key, , value]) => {
    return `    ("${key}", FileTypeResolver::Static(FileType::${value})),\n`;
  })
  .join("")}];
`;

Bun.write(`${baseDir}/src/detect/path_suffix.rs`, pathSuffixRsContent);

// ============================================================================
// Generate pattern.rs
// ============================================================================
console.log("Generating src/detect/pattern.rs...");

// Parse patterns first
const patternLines: string[] = [];

for (const line of patternContent.split(/\r?\n/)) {
  const trimmed = line.trim();
  if (trimmed === "" || trimmed.startsWith("--") || trimmed === "}") continue;

  const parent = trimmed.match(/^\['([^']+)'\]\s*=\s*\{$/);
  if (!parent) {
    const patternMatch = trimmed.match(/^\['([^']+)'\]\s*=\s*(.+)$/);
    if (patternMatch) {
      let rest = patternMatch[2];
      // Strip trailing comma
      rest = rest.replace(/,\s*$/, "");
      const parsed = parseValue(rest, patternMatch[1], PARSE_OVERRIDES);

      // Skip inline functions and unknown entries
      if (parsed.kind === "inline_function" || parsed.kind === "unknown") {
        continue;
      }

      const rawLuaPattern = patternMatch[1];
      const rustPattern = finalizeRustPatternForNvimMatch(luaToRustPattern(rawLuaPattern));
      // Neovim patterns that contain path separators (or start with `/`) are matched against the full path.
      // This makes entries like `Eterm/...` actually work (they cannot match a bare filename).
      const matchFullPath = rawLuaPattern.startsWith("/") || rawLuaPattern.includes("/") ? "true" : "false";

      if (parsed.kind === "static") {
        const variant = ftToVariant[parsed.filetype];
        patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::new(FileTypeResolver::Static(FileType::${variant}), None)),\n`);
      } else if (parsed.kind === "detect" && isDetectFunctionAvailable(parsed.functionName)) {
        patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::new(FileTypeResolver::Dynamic(detect::${parsed.functionName}), None)),\n`);
      } else if (parsed.kind === "starsetf_static") {
        const variant = ftToVariant[parsed.filetype];
        patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::starsetf(FileTypeResolver::Static(FileType::${variant}), None)),\n`);
      } else if (parsed.kind === "starsetf_detect" && isDetectFunctionAvailable(parsed.functionName)) {
        patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::starsetf(FileTypeResolver::Dynamic(detect::${parsed.functionName}), None)),\n`);
      }
    }
  }
}

// Add manual pattern overrides from reference
// NOTE: MANUAL_PATTERDS entries are already in Rust regex format, not Lua format
for (const [rustPattern, type, value, priority, matchFullPathOverride] of MANUAL_PATTERNS) {
  const matchFullPath =
    matchFullPathOverride !== undefined
      ? (matchFullPathOverride ? "true" : "false")
      : (rustPattern.includes("/") ? "true" : "false");

  if (type === "static") {
    if (priority === "starsetf") {
      patternLines.push(
        `        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::starsetf(FileTypeResolver::Static(FileType::${value}), None)),\n`,
      );
    } else if (typeof priority === "number") {
      patternLines.push(
        `        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::new(FileTypeResolver::Static(FileType::${value}), Some(${priority}))),\n`,
      );
    } else {
      patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::new(FileTypeResolver::Static(FileType::${value}), None)),\n`);
    }
  } else if (type === "dynamic") {
    if (priority === "starsetf") {
      patternLines.push(
        `        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::starsetf(FileTypeResolver::Dynamic(detect::${value}), None)),\n`,
      );
    } else if (typeof priority === "number") {
      patternLines.push(
        `        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::new(FileTypeResolver::Dynamic(detect::${value}), Some(${priority}))),\n`,
      );
    } else {
      patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::new(FileTypeResolver::Dynamic(detect::${value}), None)),\n`);
    }
  } else if (type === "closure") {
    if (priority === "starsetf") {
      patternLines.push(
        `        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::starsetf(FileTypeResolver::Dynamic(${value}), None)),\n`,
      );
    } else if (typeof priority === "number") {
      patternLines.push(
        `        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::new(FileTypeResolver::Dynamic(${value}), Some(${priority}))),\n`,
      );
    } else {
      patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::new(FileTypeResolver::Dynamic(${value}), None)),\n`);
    }
  }
}

// Add Helix patterns (from glob patterns with wildcards)
// First, build a set of existing patterns for deduplication
const existingPatterns = new Set<string>();
for (const line of patternLines) {
  const match = line.match(/regex!\(r"([^"]+)"\)/);
  if (match) {
    existingPatterns.add(match[1]);
  }
}
// Now add only Helix patterns that don't already exist
for (const [regexPattern, , variant, matchFullPath] of helixPatterns) {
  if (!existingPatterns.has(regexPattern)) {
    patternLines.push(`        (${matchFullPath ? "true" : "false"}, regex!(r"${regexPattern}").deref(), Pattern::new(FileTypeResolver::Static(FileType::${variant}), None)),\n`);
  }
}

const patternRsContent = `use std::ops::Deref;

use lazy_regex::regex;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{detect, FileType, FileTypeResolver};

pub(crate) struct Pattern {
    pub(crate) resolver: FileTypeResolver,
    pub(crate) priority: Option<isize>,
}

impl Pattern {
    fn new(resolver: FileTypeResolver, priority: Option<isize>) -> Self {
        Self { resolver, priority }
    }

    fn starsetf(resolver: FileTypeResolver, priority: Option<isize>) -> Self {
        Self {
            resolver,
            priority: priority.or(Some(isize::MIN)),
        }
    }
}

#[rustfmt::skip]
pub(crate) static PATTERN: Lazy<Vec<(bool, &'static Regex, Pattern)>> = Lazy::new(|| {
    let mut vec = vec![
${patternLines.join("")}    ];
    vec.sort_unstable_by(|(_, _, pat1), (_, _, pat2)| pat2.priority.unwrap_or(0).cmp(&pat1.priority.unwrap_or(0)));
    vec
});
`;

Bun.write(`${baseDir}/src/detect/pattern.rs`, patternRsContent);

// ============================================================================
// Print stats
// ============================================================================
console.log("\n✅ Generated files");
console.log("  src/list.rs");
console.log("  src/detect/file_extension.rs");
console.log("  src/detect/filename.rs");
console.log("  src/detect/path_suffix.rs");
console.log("  src/detect/pattern.rs");
console.log("\n📊 Stats:");
console.log(`  Total filetypes:  ${filetypeList.length}`);
console.log(`  Extensions: ${extEntries.length} (Neovim) + ${helixExtensions.length} (Helix) = ${extEntries.length + helixExtensions.length} total`);
console.log(`  Filenames:  ${filenameEntries.length} (Neovim) + ${helixFilenames.length} (Helix) = ${filenameEntries.length + helixFilenames.length} total`);
console.log(`  Path suffixes: ${pathSuffixEntries.length} (Neovim) + ${helixPathSuffixes.length} (Helix) = ${pathSuffixEntries.length + helixPathSuffixes.length} total`);
console.log(`  Patterns: ${patternLines.length} (Neovim) + ${helixPatterns.length} (Helix) = ${patternLines.length + helixPatterns.length} total`);

// Count dynamic entries
const extDynamic = extEntries.filter(([, t]) => t === "detect").length;
const filenameDynamic = filenameEntries.filter(([, t]) => t === "detect").length;
const pathSuffixDynamic = pathSuffixEntries.filter(([, t]) => t === "detect").length;
console.log(`\n📋 Dynamic entries: ${extDynamic} extensions, ${filenameDynamic} filenames, ${pathSuffixDynamic} path suffixes`);
console.log("   These use detect functions from src/detect/mod.rs");
