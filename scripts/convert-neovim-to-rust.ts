#!/usr/bin/env -S bun
/**
 * Convert Neovim's filetype.lua to Rust code for palate
 * This script reads from Neovim's runtime/lua/vim/filetype.lua
 * and generates Rust files in the base_dir
 */


const neovimFile = "/home/ivan/github/neovim/neovim/runtime/lua/vim/filetype.lua";
const neovimDetectFile = "/home/ivan/github/neovim/neovim/runtime/lua/vim/filetype/detect.lua";
const baseDir = "/home/ivan/github/casualjim/palate";

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
  "c++": "Cpp",
  "c#": "CSharp",
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
  "context": "Context",
  "cook": "Cook",
  "coq": "Coq",
  "corn": "Corn",
  "cpon": "Cpon",
  "cpp": "Cpp",
  "cqlang": "Cqlang",
  "crm": "Crm",
  "crontab": "Crontab",
  "crystal": "Crystal",
  "csc": "Csc",
  "csdl": "Csdl",
  "csh": "Csh",
  "csharp": "CSharp",
  "cs": "CSharp",
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
  "f#": "FSharp",
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
  "verilog": "Verilog",
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

// Read the Neovim filetype.lua file
const content = await Bun.file(neovimFile).text();

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
 * Manual overrides from the reference implementation
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
  "verilog": ["static", "Verilog"],
  "vh": ["static", "Verilog"],
  "vlg": ["static", "Verilog"],
  "zir": ["static", "Zir"],
};

/**
 * Manual pattern overrides from the reference implementation
 * Format: [rust_pattern, "static"|"dynamic", variant_or_function, priority?]
 * priority: -1 for negative (starsetf), undefined/0 for normal
 * NOTE: These are already in Rust regex format, not Lua pattern format
 */
const MANUAL_PATTERDS: Array<[string, "static" | "dynamic", string, number?]> = [
  // Patterns from reference that aren't in current Neovim
  ["^.*\\.git/.*$", "dynamic", "git", -1],
  ["^.*\\.[Ll][Oo][Gg]$", "dynamic", "log"],
  ["^.+~$", "dynamic", "tmp"],
];

// Track all filetypes for the enum
const filetypes = new Set<string>();

// Add manual override filetypes to ensure they're included in the enum
for (const [key, [type, value]] of Object.entries(MANUAL_OVERRIDES)) {
  if (type === "static") {
    filetypes.add(value);
  }
}

const extEntries: Array<[string, string, string]> = [];
const filenameEntries: Array<[string, string, string]> = [];

/**
 * Convert filetype name to Rust enum variant name (PascalCase)
 * Uses the reference mapping from rubixdev/tft for consistency
 */
function toFtVariant(str: string): string {
  // First, check the reference mapping
  if (REFERENCE_MAPPING[str]) {
    return REFERENCE_MAPPING[str];
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

  // Handle leading / - in Neovim patterns this means full path match,
  // but reference converts it to ^.* to match anywhere in path
  if (pattern.startsWith("/")) {
    result = "^.*";
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
      const rustMagic = ["(", ")", "[", "]", "{", "}", "|", "+", "?", "*"];
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

/**
 * Mapping of inline function keys to their corresponding detect functions
 * These are entries in Neovim's filetype.lua that use inline functions
 * but have corresponding detect functions implemented in Rust
 */
const INLINE_FUNCTION_DETECT_MAPPING: Record<string, string> = {
  "asa": "asa",
  "btm": "btm",
  "hook": "hook",
  "in": "in_",
};

/**
 * Mapping of detect.* functions that don't exist in Rust but have static fallback values
 * Format: "extension": "fallback_filetype"
 */
const STATIC_FALLBACK_EXTENSIONS: Record<string, string> = {
  // Local inline functions that don't exist as detect.*
  "rc": "rc",  // detect_rc is a local inline function
  "rch": "rc", // detect_rc is a local inline function

  // detect.vba returns 'vim' or 'vb' - use 'vb' as fallback
  "vba": "vb",
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
  "mk": "markdown",
  // mp* extensions all resolve to MpMetafun - use static (matching reference)
  "mpiv": "mpmetafun",
  "mpvi": "mpmetafun",
  "mpxl": "mpmetafun",

  // Detect functions that don't exist in Rust yet
  "class": "stata",
  "dsp": "dsp",
  "f": "fortran",

  // Local inline functions
  "rc": "rc",
};

/**
 * Mapping of extensions that need custom closures
 * These are for detect functions with non-standard signatures
 */
const CUSTOM_CLOSURE_EXTENSIONS: Record<string, string> = {
  // bindzone has signature: fn(content: &str, default: Option<FileType>) -> Option<FileType>
  "com": '|_, content| detect::bindzone(content, Some(FileType::Dcl))',
  "db": '|_, content| detect::bindzone(content, None)',

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

/**
 * Parse a value from the table
 */
function parseValue(value: string, key: string): [string, string | null, string | null] {
  // Check for custom closure extensions first
  if (key in CUSTOM_CLOSURE_EXTENSIONS) {
    return ["closure", CUSTOM_CLOSURE_EXTENSIONS[key], null];
  }

  // Check for inline function - some have corresponding detect functions
  if (value.match(/^function\s*\(/)) {
    if (key in INLINE_FUNCTION_DETECT_MAPPING) {
      const detectFunc = INLINE_FUNCTION_DETECT_MAPPING[key];
      if (isDetectFunctionAvailable(detectFunc)) {
        return ["dynamic", detectFunc, null];
      }
    }
    return ["inline_function", null, null];
  }

  // Check for detect_line1 - prefer detect function if available, otherwise use fallback as static
  // detect_line1(pat, a, b) returns 'a' if line 1 matches pat, otherwise 'b'
  const line1Match = value.match(/^detect_line1\s*\(/);
  if (line1Match) {
    // Check if there's a detect function for this key
    const rustFuncName = getDetectFunctionName(key);
    if (isDetectFunctionAvailable(rustFuncName)) {
      return ["dynamic", rustFuncName, null];
    }
    // Otherwise, use the fallback value as static mapping
    const fallback = value.match(/['"]([^'"]+)['"]\s*\)\s*$/);
    if (fallback) {
      return ["static", fallback[1], null];
    }
    return ["static", "text", null];
  }

  // Check for detect_seq
  const seqMatch = value.match(/^detect_seq\s*\(/);
  if (seqMatch) {
    const fallback = value.match(/['"]([^'"]+)['"]\s*\)\s*$/);
    if (fallback) {
      return ["static", fallback[1], null];
    }
    return ["static", "text", null];
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
          return ["starsetf_dynamic", getDetectFunctionName(funcName[1]), null];
        }
      }
      return ["starsetf_static", inner[1], null];
    }
    // Try unquoted detect reference: starsetf(detect.uci)
    const detectRef = value.match(/starsetf\s*\(\s*detect\.(\w+)\s*\)/);
    if (detectRef) {
      return ["starsetf_dynamic", getDetectFunctionName(detectRef[1]), null];
    }
    // Catch other unquoted references: starsetf(apachestyle)
    const plainRef = value.match(/starsetf\s*\(\s*(\w+)\s*\)/);
    if (plainRef) {
      return ["starsetf_static", plainRef[1], null];
    }
    // Inline function - skip this entry
    return ["inline_function", null, null];
  }

  // Check for bare detect function references (detect_noext, detect_rc, etc.)
  if (value.match(/^detect_[a-z_]+$/)) {
    const funcName = value.match(/^detect_([a-z_]+)$/);
    if (funcName) {
      const fullFuncName = funcName[0]; // e.g., "detect_noext"
      // Try to use the detect function first
      const rustFuncName = getDetectFunctionName(funcName[1]);
      if (isDetectFunctionAvailable(rustFuncName)) {
        return ["dynamic", rustFuncName, null];
      }
      // Only fall back to static if function doesn't exist
      if (key in STATIC_FALLBACK_EXTENSIONS) {
        return ["static", STATIC_FALLBACK_EXTENSIONS[key], null];
      }
      // Otherwise skip
      return ["inline_function", null, null];
    }
  }

  // Check for detect.* functions
  if (value.match(/^detect\.(\w+)/)) {
    const funcName = value.match(/^detect\.(\w+)/);
    if (funcName) {
      const detectFuncName = funcName[1]; // e.g., "bash" from "detect.bash"
      // Check if this function has a custom closure
      if (detectFuncName in CUSTOM_CLOSURE_EXTENSIONS) {
        return ["closure", CUSTOM_CLOSURE_EXTENSIONS[detectFuncName], null];
      }
      const rustFuncName = getDetectFunctionName(detectFuncName);
      // Check if this function has a static fallback
      if (key in STATIC_FALLBACK_EXTENSIONS) {
        return ["static", STATIC_FALLBACK_EXTENSIONS[key], null];
      }
      return ["dynamic", rustFuncName, null];
    }
  }

  // Extract string value from quotes: '8th' or "8th"
  const strValue = value.match(/['"]([^'"]+)['"]/);
  if (strValue) {
    return ["static", strValue[1], null];
  }

  return ["unknown", null, null];
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
  v: { v: true, verilog: true, coq: true },
  web: { web: true, winbatch: true },
  xfree86: { xf86conf3: true, xf86conf: true },
  xml: { xml: true, docbookxml4: true, docbookxml5: true, xbl: true },
  xpm: { xpm2: true, xpm: true },
  y: { yacc: true, racc: true },
};

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
    // Strip trailing comma
    rest = rest.replace(/,\s*$/, "");
    const [valueType, value1, value2] = parseValue(rest, key);

    if (valueType === "static" && value1) {
      extEntries.push([key, "static", value1]);
      filetypes.add(value1);
    } else if (valueType === "dynamic" && value1) {
      extEntries.push([key, "detect", value1]);
      // Add all possible return filetypes from this detect function
      if (detectReturns[value1]) {
        for (const ft of Object.keys(detectReturns[value1])) {
          filetypes.add(ft);
        }
      }
      // Add fallback filetype if provided
      if (value2) {
        filetypes.add(value2);
      }
    } else if (valueType === "closure" && value1) {
      extEntries.push([key, "closure", value1]);
    } else if (valueType === "starsetf_static" && value1) {
      extEntries.push([key, "starsetf_static", value1]);
      filetypes.add(value1);
    } else if (valueType === "starsetf_dynamic" && value1) {
      extEntries.push([key, "starsetf_detect", value1]);
      if (detectReturns[value1]) {
        for (const ft of Object.keys(detectReturns[value1])) {
          filetypes.add(ft);
        }
      }
    }
  }
}

// ============================================================================
// Parse filename table
// ============================================================================
const pathSuffixEntries: Array<[string, string, string]> = [];

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
    const [valueType, value1, value2] = parseValue(rest, key);

    // Split based on whether the key contains a path separator
    if (key.includes("/")) {
      // Path entries go to path_suffix
      // Strip leading slash if present and convert format
      let suffixKey = key.startsWith("/") ? key.slice(1) : key;

      if (valueType === "static" && value1) {
        pathSuffixEntries.push([suffixKey, "static", value1]);
        filetypes.add(value1);
      } else if (valueType === "dynamic" && value1) {
        pathSuffixEntries.push([suffixKey, "detect", value1]);
        if (detectReturns[value1]) {
          for (const ft of Object.keys(detectReturns[value1])) {
            filetypes.add(ft);
          }
        }
        if (value2) {
          filetypes.add(value2);
        }
      } else if (valueType === "closure" && value1) {
        pathSuffixEntries.push([suffixKey, "closure", value1]);
      }
    } else {
      // Plain filename entries go to filename
      if (valueType === "static" && value1) {
        filenameEntries.push([key, "static", value1]);
        filetypes.add(value1);
      } else if (valueType === "dynamic" && value1) {
        filenameEntries.push([key, "detect", value1]);
        if (detectReturns[value1]) {
          for (const ft of Object.keys(detectReturns[value1])) {
            filetypes.add(ft);
          }
        }
        if (value2) {
          filetypes.add(value2);
        }
      } else if (valueType === "starsetf_static" && value1) {
        filenameEntries.push([key, "starsetf_static", value1]);
        filetypes.add(value1);
      } else if (valueType === "starsetf_dynamic" && value1) {
        filenameEntries.push([key, "starsetf_detect", value1]);
        if (detectReturns[value1]) {
          for (const ft of Object.keys(detectReturns[value1])) {
            filetypes.add(ft);
          }
        }
      } else if (valueType === "closure" && value1) {
        filenameEntries.push([key, "closure", value1]);
      }
    }
  }
}

// ============================================================================
// First pass: parse pattern table to collect all filetypes
// ============================================================================
console.error("Parsing pattern table for additional filetypes...");
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
      const [valueType, value1, value2] = parseValue(rest, patternMatch[1]);

      if (valueType === "static" && value1) {
        filetypes.add(value1);
      } else if (valueType === "dynamic" && value1) {
        if (detectReturns[value1]) {
          for (const ft of Object.keys(detectReturns[value1])) {
            filetypes.add(ft);
          }
        }
        if (value2) {
          filetypes.add(value2);
        }
      } else if (valueType === "starsetf_static" && value1) {
        filetypes.add(value1);
      } else if (valueType === "starsetf_dynamic" && value1) {
        if (detectReturns[value1]) {
          for (const ft of Object.keys(detectReturns[value1])) {
            filetypes.add(ft);
          }
        }
      }
    }
  }
}

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
// only use the first one (which will be alphabetically first due to sorting)
const variantToFiletype: Record<string, string> = {};
for (const ft of filetypeList) {
  const variant = ftToVariant[ft];
  if (!variantToFiletype[variant]) {
    variantToFiletype[variant] = ft;
  }
}
const uniqueVariants = Object.entries(variantToFiletype).sort((a, b) => a[1].localeCompare(b[1]));

// ============================================================================
// Generate list.rs
// ============================================================================
console.error("Generating src/list.rs...");
const listRsContent = `macro_rules! list {
    ($($variant:ident $(as $as:literal)?),* $(,)?) => {
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
            strum::EnumString,
            strum::EnumVariantNames,
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
                #[doc = concat!("(De)serialized as \`", $($as, ", **not** \`",)? casey::lower!(stringify!($variant)), "\`.")]
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
    if (ft !== simpleLower) {
      return `    ${variant} as "${ft}",\n`;
    }
    return `    ${variant},\n`;
  })
  .join("")}
}
`;

Bun.write(`${baseDir}/src/list.rs`, listRsContent);

// ============================================================================
// Generate file_extension.rs
// ============================================================================
console.error("Generating src/detect/file_extension.rs...");
const extRsContent = `use phf::{phf_map, Map};

use crate::{detect, FileType, FileTypeResolver};

pub(crate) static FILE_EXTENSION: Map<&'static str, FileTypeResolver> = phf_map! {
${Object.entries(MANUAL_OVERRIDES)
  .map(([key, [type, value]]) => {
    if (type === "static") {
      const variant = ftToVariant[value];
      return `    "${key}" => FileTypeResolver::Static(FileType::${variant}),\n`;
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
  .join("")}};
`;

Bun.write(`${baseDir}/src/detect/file_extension.rs`, extRsContent);

// ============================================================================
// Generate filename.rs
// ============================================================================
console.error("Generating src/detect/filename.rs...");
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
  .join("")}};
`;

Bun.write(`${baseDir}/src/detect/filename.rs`, filenameRsContent);

// ============================================================================
// Generate path_suffix.rs
// ============================================================================
console.error("Generating src/detect/path_suffix.rs...");

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
  .join("")}];
`;

Bun.write(`${baseDir}/src/detect/path_suffix.rs`, pathSuffixRsContent);

// ============================================================================
// Generate pattern.rs
// ============================================================================
console.error("Generating src/detect/pattern.rs...");

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
      const [valueType, value1] = parseValue(rest, patternMatch[1]);

      // Skip inline functions and unknown entries
      if (valueType === "inline_function" || valueType === "unknown") {
        continue;
      }

      const rustPattern = luaToRustPattern(patternMatch[1]);
      const matchFullPath = patternMatch[1].startsWith("/") ? "true" : "false";

      if (valueType === "static" && value1) {
        const variant = ftToVariant[value1];
        patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::new(FileTypeResolver::Static(FileType::${variant}), None)),\n`);
      } else if (valueType === "dynamic" && value1 && isDetectFunctionAvailable(value1)) {
        patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::new(FileTypeResolver::Dynamic(detect::${value1}), None)),\n`);
      } else if (valueType === "starsetf_static" && value1) {
        const variant = ftToVariant[value1];
        patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::starsetf(FileTypeResolver::Static(FileType::${variant}), None)),\n`);
      } else if (valueType === "starsetf_dynamic" && value1 && isDetectFunctionAvailable(value1)) {
        patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::starsetf(FileTypeResolver::Dynamic(detect::${value1}), None)),\n`);
      }
    }
  }
}

// Add manual pattern overrides from reference
// NOTE: MANUAL_PATTERDS entries are already in Rust regex format, not Lua format
for (const [rustPattern, type, value, priority] of MANUAL_PATTERDS) {
  const matchFullPath = rustPattern.startsWith("^.*") ? "true" : "false";

  if (type === "static") {
    const variant = ftToVariant[value];
    if (priority === -1) {
      patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::starsetf(FileTypeResolver::Static(FileType::${variant}), None)),\n`);
    } else {
      patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::new(FileTypeResolver::Static(FileType::${variant}), None)),\n`);
    }
  } else if (type === "dynamic") {
    if (priority === -1) {
      patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::starsetf(FileTypeResolver::Dynamic(detect::${value}), None)),\n`);
    } else {
      patternLines.push(`        (${matchFullPath}, regex!(r"${rustPattern}").deref(), Pattern::new(FileTypeResolver::Dynamic(detect::${value}), None)),\n`);
    }
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
console.error("\n✅ Generated files");
console.error("  src/list.rs");
console.error("  src/detect/file_extension.rs");
console.error("  src/detect/filename.rs");
console.error("  src/detect/path_suffix.rs");
console.error("  src/detect/pattern.rs");
console.error("\n📊 Stats:");
console.error(`  Total filetypes:  ${filetypeList.length}`);
console.error(`  Extensions: ${extEntries.length}`);
console.error(`  Filenames:  ${filenameEntries.length}`);
console.error(`  Path suffixes: ${pathSuffixEntries.length}`);
console.error(`  Patterns included: ${patternLines.length}`);

// Count dynamic entries
const extDynamic = extEntries.filter(([, t]) => t === "detect").length;
const filenameDynamic = filenameEntries.filter(([, t]) => t === "detect").length;
const pathSuffixDynamic = pathSuffixEntries.filter(([, t]) => t === "detect").length;
console.error(`\n📋 Dynamic entries: ${extDynamic} extensions, ${filenameDynamic} filenames, ${pathSuffixDynamic} path suffixes`);
console.error("   These use detect functions from src/detect/mod.rs");
