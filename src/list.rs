macro_rules! list {
    (@canonical $variant:ident as $lit:literal) => { $lit };
    (@canonical $variant:ident) => { casey::lower!(stringify!($variant)) };
    ($($(#[$($attr:meta),+])? $variant:ident $(as $as:literal)?),* $(,)?) => {
        use core::fmt;

        /// A non-exhaustive list of text file types.
        ///
        /// The type derives the following traits for convenience. For (de)serialization to/from strings,
        /// lowercase casing is used unless otherwise specified in the variants docs.
        ///
        /// - [`Display`](core::fmt::Display): formatting and a `.to_string()` method (canonical form)
        /// - [`AsRef<str>`]: conversion into `&str` (canonical form)
        /// - [`From<FileType>` for `&'static str`]: conversion into `&'static str` (canonical form)
        /// - [`strum::EnumString`]: [`FromStr`](core::str::FromStr) impl for turning strings into the
        ///   corresponding variant
        /// - [`strum::EnumVariantNames`]: an associated `VARIANTS` constant containing the string names of
        ///   all variants (requires [`strum::VariantNames`] to be in scope)
        /// - [`Clone`], [`Copy`]
        /// - [`Debug`]
        /// - [`Hash`]
        /// - [`PartialEq`], [`Eq`]
        /// - [`Default`]: the default is [`FileType::Text`]
        /// - <span class="stab portability"><code>serde</code></span> [`serde::Serialize`]: serialize into
        ///   a string
        /// - <span class="stab portability"><code>serde</code></span> [`serde::Deserialize`]: deserialize
        ///   from a string
        #[derive(
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
            /// A plain text file. This is the default variant. (De)serialized as `text`.
            #[strum(serialize = "text", serialize = "fundamental", serialize = "plain-text")]

            #[default]
            Text,

            $(
                $(#[$($attr),+])?
                #[doc = concat!("(De)serialized as `", $($as, "`, **not** `",)? casey::lower!(stringify!($variant)), "`")]
                $(#[strum(serialize = $as)])?
                $variant,
            )*
        }

        impl FileType {
            /// Canonical string representation (tft/nvim filetype).
            pub const fn canonical(self) -> &'static str {
                match self {
                    FileType::Text => "text",
                    $(FileType::$variant => list!(@canonical $variant $(as $as)?),)*
                }
            }
        }

        impl AsRef<str> for FileType {
            fn as_ref(&self) -> &str {
                (*self).canonical()
            }
        }

        impl From<FileType> for &'static str {
            fn from(value: FileType) -> Self {
                value.canonical()
            }
        }

        impl fmt::Display for FileType {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str((*self).canonical())
            }
        }
    };
}

list! {
    Ft1cEnterprise as "1c-enterprise",
    Ft2DimensionalArray as "2-dimensional-array",
    Ft4d as "4d",
    A2ps,
    A65,
    Aap,
    Abap,
    AbapCds as "abap-cds",
    Abaqus,
    Abc,
    Abel,
    Abnf,
    Acedb,
    Actionscript,
    #[strum(serialize = "ad-block", serialize = "adb", serialize = "adblock", serialize = "adblock-filter-list")]
    AdBlockFilters as "ad-block-filters",
    #[strum(serialize = "ada2005", serialize = "ada95")]
    Ada as "ada",
    #[strum(serialize = "opentype-feature-file")]
    Afdko as "afdko",
    Agda,
    #[strum(serialize = "ags-script")]
    Ags as "ags",
    Ahdl,
    Aidl,
    Aiken,
    Algol,
    Alloy,
    AlsaConf,
    #[strum(serialize = "altium-designer")]
    Altium as "altium",
    Aml,
    Ampl,
    #[strum(serialize = "emacs-muse", serialize = "muse")]
    Amusewiki as "amusewiki",
    AnswerSetProgramming as "answer-set-programming",
    #[strum(serialize = "ant-build-system")]
    Ant as "ant",
    Antlers,
    #[strum(serialize = "antlr")]
    Antlr4 as "antlr4",
    #[strum(serialize = "aconf", serialize = "apacheconf")]
    Apache as "apache",
    ApacheStyle,
    Apex,
    ApiBlueprint as "api-blueprint",
    #[strum(serialize = "abuild", serialize = "alpine-abuild")]
    Apkbuild as "apkbuild",
    Apl,
    ApolloGuidanceComputer as "apollo-guidance-computer",
    #[strum(serialize = "osascript")]
    AppleScript as "applescript",
    AptConf,
    Arc,
    Arch,
    #[strum(serialize = "processing")]
    Arduino as "arduino",
    Art,
    #[strum(serialize = "stl", serialize = "stla")]
    AsciiStl as "ascii-stl",
    AsciiDoc,
    Asl,
    Asm,
    #[strum(serialize = "asn-1")]
    Asn as "asn",
    #[strum(serialize = "classic-asp")]
    Asp as "asp",
    Aspectj,
    AspPerl,
    AspVbs,
    #[strum(serialize = "asp-net", serialize = "aspx-vb")]
    Aspx as "aspx",
    Asterisk,
    AsteriskVoiceMail as "asteriskvm",
    Astro,
    #[strum(serialize = "asymptote", serialize = "ltspice-symbol")]
    Asy as "asy",
    #[strum(serialize = "actionscript-3", serialize = "actionscript3", serialize = "angelscript", serialize = "as3")]
    Atlas as "atlas",
    #[strum(serialize = "ats")]
    Ats2 as "ats2",
    Augeas,
    Authzed,
    #[strum(serialize = "ahk")]
    AutoHotKey as "autohotkey",
    #[strum(serialize = "au3", serialize = "autoit3", serialize = "autoitscript")]
    AutoIt as "autoit",
    Automake,
    Autopkgtest,
    Ave,
    AvroIdl as "avro-idl",
    Awk,
    #[strum(serialize = "b-formal-method")]
    B as "b",
    Ballerina,
    Bash,
    #[strum(serialize = "console", serialize = "shellsession")]
    BashSession as "bash-session",
    Basic,
    #[strum(serialize = "b4x")]
    BasicForAndroid as "basic-for-android",
    Bass,
    Bat,
    Bc,
    #[strum(serialize = "glyph-bitmap-distribution-format")]
    Bdf as "bdf",
    #[strum(serialize = "berry")]
    Be as "be",
    BeanCount,
    Beef,
    Befunge,
    #[strum(serialize = "bibtex")]
    Bib as "bib",
    Bicep,
    BicepParams as "bicep-params",
    #[strum(serialize = "dns-zone")]
    Bindzone as "bindzone",
    Bison,
    #[strum(serialize = "b3d", serialize = "blitz3d", serialize = "blitzbasic", serialize = "blitzplus", serialize = "bplus")]
    Bitbake as "bitbake",
    Blade,
    Blank,
    #[strum(serialize = "blp")]
    Blueprint as "blueprint",
    #[strum(serialize = "bluespec", serialize = "bsv")]
    BluespecBsv as "bluespec-bsv",
    #[strum(serialize = "blitzmax")]
    Bmax as "bmax",
    Boo,
    Boogie,
    Bp,
    Bpftrace,
    Bqn,
    Brainfuck,
    #[strum(serialize = "bh", serialize = "bikeshed", serialize = "bluespec-bh", serialize = "bluespec-classic")]
    Brighterscript as "brighterscript",
    Brightscript,
    #[strum(serialize = "zeek")]
    Bro as "bro",
    Browserslist,
    Bru,
    Bsdl,
    #[strum(serialize = "bibtex-style", serialize = "buildstream")]
    Bst as "bst",
    Btm,
    #[strum(serialize = "dm")]
    Byond as "byond",
    #[strum(serialize = "bazel")]
    Bzl as "bzl",
    Bzr,
    #[strum(serialize = "quakec")]
    C as "c",
    CObjdump as "c-objdump",
    C3,
    #[strum(serialize = "cabal-config")]
    Cabal as "cabal",
    CabalConfig,
    CabalProject,
    #[strum(serialize = "caddyfile")]
    Caddy as "caddy",
    #[strum(serialize = "cairo-zero")]
    Cairo as "cairo",
    Calendar,
    Cameligo,
    Cangjie,
    #[strum(serialize = "cap-n-proto")]
    Capnp as "capnp",
    Carbon,
    Catalog,
    #[strum(serialize = "cadence")]
    Cdc as "cdc",
    Cdl,
    CdrdaoConf,
    #[strum(serialize = "world-of-warcraft-addon-data")]
    Cdrtoc as "cdrtoc",
    #[strum(serialize = "cap-cds")]
    Cds as "cds",
    Cedar,
    Ceylon,
    #[strum(serialize = "cfc", serialize = "cfm", serialize = "cfml", serialize = "coldfusion", serialize = "coldfusion-cfc", serialize = "coldfusion-html")]
    Cf as "cf",
    CfEngine,
    Cfg,
    Cgdbrc,
    Ch,
    ChaiScript,
    Change,
    Changelog,
    Charity,
    #[strum(serialize = "c2hs", serialize = "c2hs-haskell")]
    Chaskell as "chaskell",
    Chatito,
    #[strum(serialize = "checksums", serialize = "hash", serialize = "hashes", serialize = "sum", serialize = "sums")]
    Checksum as "checksum",
    Chill,
    Chordpro,
    #[strum(serialize = "chapel")]
    Chpl as "chpl",
    Chuck,
    Cil,
    Circom,
    Cirru,
    Cl,
    Clarion,
    #[strum(serialize = "clar")]
    Clarity as "clarity",
    Clean,
    Click,
    #[strum(serialize = "advpl", serialize = "foxpro", serialize = "xbase")]
    Clipper as "clipper",
    Clojure,
    CloudFirestoreSecurityRules as "cloud-firestore-security-rules",
    Clue,
    CMake,
    CMakeCache,
    CMod,
    Cmusrc,
    Cobol,
    #[strum(serialize = "smpl")]
    Coccinelle as "coccinelle",
    Coco,
    Codeowners,
    Collada,
    Comment,
    ComponentPascal as "component-pascal",
    ConaryRecipe,
    Conf,
    #[strum(serialize = "autoconf", serialize = "m4sugar")]
    Config as "config",
    #[strum(serialize = "conll-u", serialize = "conll-x")]
    Conll as "conll",
    Context,
    #[strum(serialize = "cooklang")]
    Cook as "cook",
    #[strum(serialize = "rocq", serialize = "rocq-prover")]
    Coq as "coq",
    Corn,
    Cpon,
    Cpp,
    Cqlang,
    Creole,
    Crm,
    #[strum(serialize = "cron", serialize = "cron-table")]
    Crontab as "crontab",
    Crystal,
    #[strum(serialize = "gsc")]
    Csc as "csc",
    Csdl,
    Csh,
    #[strum(serialize = "c_sharp", serialize = "c-sharp", serialize = "cake", serialize = "cakescript", serialize = "coffee", serialize = "coffee-script", serialize = "coffeescript", serialize = "cs")]
    CSharp as "csharp",
    Cson,
    #[strum(serialize = "csound-document")]
    CsoundCsd as "csound-csd",
    #[strum(serialize = "csound")]
    CsoundOrc as "csound-orc",
    #[strum(serialize = "csound-score")]
    CsoundSco as "csound-sco",
    Csp,
    Css,
    Csv,
    CTerm,
    #[strum(serialize = "gherkin")]
    Cucumber as "cucumber",
    Cuda,
    #[strum(serialize = "cue-sheet")]
    Cue as "cue",
    Cupl,
    Cuplsim,
    #[strum(serialize = "curl-config")]
    Curlrc as "curlrc",
    Curry,
    Cvs,
    Cvsrc,
    Cweb,
    #[strum(serialize = "common-workflow-language")]
    Cwl as "cwl",
    Cycript,
    Cylc,
    Cynpp,
    Cypher,
    #[strum(serialize = "dlang")]
    D as "d",
    DObjdump as "d-objdump",
    #[strum(serialize = "d2")]
    D2lang as "d2lang",
    Dafny,
    Dart,
    Daslang,
    DataScript,
    Dataweave,
    Dax,
    Dcd,
    #[strum(serialize = "digital-command-language")]
    Dcl as "dcl",
    DebChangelog,
    DebControl,
    DebCopyright,
    DebianPackageControlFile as "debian-package-control-file",
    #[strum(serialize = "deb822sources")]
    DebSources as "debsources",
    Def,
    Denizenscript,
    DenyHosts,
    Dep3Patch,
    Desc,
    Desktop,
    Dhall,
    DictConf,
    DictdConf,
    DirColors,
    Disassembly,
    Diva,
    Djot,
    DnsMasq,
    DocBookSgml4 as "docbk-sgml-4",
    DocBookXml4 as "docbk-xml-4",
    DocBookXml5 as "docbk-xml-5",
    Docbookxml4,
    Docbookxml5,
    #[strum(serialize = "containerfile")]
    Dockerfile as "dockerfile",
    Dogescript,
    #[strum(serialize = "batch", serialize = "batchfile")]
    DosBatch as "dosbatch",
    #[strum(serialize = "npm-config", serialize = "npmrc")]
    DosIni as "dosini",
    #[strum(serialize = "graphviz-dot")]
    Dot as "dot",
    Dotenv,
    Doxygen,
    #[strum(serialize = "darcs-patch")]
    Dpatch as "dpatch",
    Dracula,
    Dsl,
    Dtd,
    #[strum(serialize = "dtrace-script")]
    DTrace as "dtrace",
    #[strum(serialize = "devicetree")]
    Dts as "dts",
    Dune,
    Dylan,
    DylanIntr,
    DylanLid,
    E,
    Eagle,
    #[strum(serialize = "earthly")]
    Earthfile as "earthfile",
    Easybuild,
    Ebnf,
    Ecd,
    EcereProjects as "ecere-projects",
    Ecl,
    Eclipse,
    #[strum(serialize = "ecmarkup")]
    Ecmarkdown as "ecmarkdown",
    #[strum(serialize = "html-ecr")]
    Ecr as "ecr",
    Edge,
    Edif,
    #[strum(serialize = "editor-config")]
    EditorConfig as "editorconfig",
    EdjeDataCollection as "edje-data-collection",
    Edn,
    Eds,
    #[strum(serialize = "eex")]
    EElixir as "eelixir",
    #[strum(serialize = "kicad-schematic")]
    EeschemaSchematic as "eeschema-schematic",
    Eiffel,
    #[strum(serialize = "8th")]
    Eighth as "eighth",
    #[strum(serialize = "ejs")]
    EJavaScript as "ejavascript",
    Elf,
    ELinks,
    #[strum(serialize = "el")]
    Elisp as "elisp",
    Elixir,
    Elm,
    ElmFilt,
    Elsa,
    Elvish,
    ElvishTranscript as "elvish-transcript",
    Emberscript,
    Enforce,
    EPuppet,
    Eq,
    Erlang,
    #[strum(serialize = "embedded_template", serialize = "erb", serialize = "html-erb", serialize = "html+ruby", serialize = "rhtml")]
    ERuby as "eruby",
    #[strum(serialize = "edgeql")]
    Esdl as "esdl",
    Esmtprc,
    #[strum(serialize = "ec")]
    Esqlc as "esqlc",
    Esterel,
    Eterm,
    Euphoria,
    Euphoria3,
    Execline,
    Exim,
    Expect,
    Exports,
    Factor,
    Falcon,
    #[strum(serialize = "fantom")]
    Fan as "fan",
    Fancy,
    #[strum(serialize = "dsp")]
    Faust as "faust",
    Fennel,
    FetchMail,
    Fga,
    #[strum(serialize = "genero-4gl")]
    Fgl as "fgl",
    Fidl,
    #[strum(serialize = "figlet-font")]
    Figfont as "figfont",
    Filterscript,
    Firrtl,
    Fish,
    Flix,
    #[strum(serialize = "freemarker", serialize = "ftl")]
    Fluent as "fluent",
    Foam,
    FocExec,
    Form,
    Forth,
    #[strum(serialize = "filebench-wml", serialize = "formatted", serialize = "fortran-free-form")]
    Fortran as "fortran",
    FpcMake,
    FrameScript,
    #[strum(serialize = "fb")]
    FreeBasic as "freebasic",
    Frege,
    #[strum(serialize = "facility")]
    Facility as "fsd",
    Fsh,
    #[strum(serialize = "f", serialize = "fsharp_signature")]
    FSharp as "fsharp",
    FsTab,
    Fstar,
    Func,
    Fusion,
    Futhark,
    Fvwm,
    Fvwm1 as "fvwm-1",
    #[strum(serialize = "fvwm-2")]
    Fvwm2 as "fvwm2",
    Fvwm2M4,
    GameMakerLanguage as "game-maker-language",
    Gaml,
    Gams,
    Gap,
    Gaptst,
    #[strum(serialize = "gnu-asm", serialize = "unix-asm", serialize = "unix-assembly")]
    Gas as "gas",
    Gdb,
    #[strum(serialize = "modelica", serialize = "motoko")]
    Gdmo as "gdmo",
    #[strum(serialize = "godot_resource", serialize = "godot-resource")]
    GdResource as "gdresource",
    GdScript,
    #[strum(serialize = "shaderlab")]
    GdShader as "gdshader",
    Gdshader as "gdshaderinc",
    Gedcom,
    Gel,
    GemfileLock as "gemfile-lock",
    #[strum(serialize = "gemini")]
    GemText as "gemtext",
    GeneroPer as "genero-per",
    GentooEbuild as "gentoo-ebuild",
    GentooEclass as "gentoo-eclass",
    #[strum(serialize = "grammatical-framework")]
    Gf as "gf",
    Gift,
    Git,
    #[strum(serialize = "git-revision-list")]
    GitBlameIgnoreRevs as "git-blame-ignore-revs",
    #[strum(serialize = "git-attributes")]
    GitAttributes as "gitattributes",
    #[strum(serialize = "commit", serialize = "git-commit")]
    GitCommit as "gitcommit",
    #[strum(serialize = "git_config", serialize = "git-config", serialize = "gitmodules")]
    GitConfig as "gitconfig",
    #[strum(serialize = "diff", serialize = "udiff")]
    Diff as "gitdiff",
    #[strum(serialize = "git-ignore", serialize = "ignore", serialize = "ignore-list")]
    GitIgnore as "gitignore",
    GitOlite,
    #[strum(serialize = "git_rebase", serialize = "git-rebase")]
    GitRebase as "gitrebase",
    GitSendEmail,
    Gkrellmrc,
    Gleam,
    Glsl,
    Glyph,
    Gn,
    Gnash,
    #[strum(serialize = "gp")]
    GnuPlot as "gnuplot",
    #[strum(serialize = "golang")]
    Go as "go",
    Goaccess,
    Goctl,
    Golo,
    #[strum(serialize = "go-mod", serialize = "go-module", serialize = "go.mod")]
    GoMod as "gomod",
    #[strum(serialize = "go-checksums", serialize = "go-sum", serialize = "go-work-sum", serialize = "go.sum", serialize = "go.work.sum")]
    GoSum as "gosum",
    Gotmpl,
    #[strum(serialize = "go-work", serialize = "go-workspace", serialize = "go.work")]
    GoWork as "gowork",
    Gpg,
    Grace,
    GradleKotlinDsl as "gradle-kotlin-dsl",
    #[strum(serialize = "genie", serialize = "gosu")]
    Grads as "grads",
    GraphModelingLanguage as "graph-modeling-language",
    GraphQl,
    Gren,
    Gretl,
    Groff,
    #[strum(serialize = "gradle")]
    Groovy as "groovy",
    Groq,
    Group,
    Grub,
    #[strum(serialize = "groovy-server-pages", serialize = "java-server-page")]
    Gsp as "gsp",
    Gstlaunch,
    Gtkrc,
    Gyp,
    Hack,
    Haml,
    Hamster,
    #[strum(serialize = "glimmer", serialize = "hbs", serialize = "htmlbars")]
    Handlebars as "handlebars",
    Haproxy,
    Hare,
    Haskell,
    #[strum(serialize = "haskell_persistent", serialize = "haskell-persistent")]
    HaskellPersistent as "haskellpersistent",
    Haste,
    HastePreProc,
    #[strum(serialize = "hx")]
    Haxe as "haxe",
    #[strum(serialize = "harbour")]
    Hb as "hb",
    #[strum(serialize = "hashicorp-configuration-language", serialize = "opentofu")]
    Hcl as "hcl",
    #[strum(serialize = "html-eex")]
    Heex as "heex",
    Helm,
    #[strum(serialize = "vim-help-file", serialize = "vimdoc")]
    VimHelp as "help",
    Hercules,
    Hex,
    HexDump,
    Hgcommit,
    Hip,
    Hiveql,
    HJson,
    #[strum(serialize = "flux")]
    Hlsl as "hlsl",
    #[strum(serialize = "hls-playlist", serialize = "m3u", serialize = "m3u-playlist")]
    HlsPlaylist as "hlsplaylist",
    Hocon,
    Hog,
    Hollywood,
    Holyc,
    Hoon,
    HostConf,
    #[strum(serialize = "hosts-file")]
    Hosts as "hosts",
    HostsAccess,
    #[strum(serialize = "html-razor")]
    Html as "html",
    #[strum(serialize = "angular")]
    Angular as "htmlangular",
    HtmlDjango,
    HtmlM4,
    HtTest,
    Http,
    Hurl,
    Hxml,
    #[strum(serialize = "hylang")]
    Hy as "hy",
    Hylo,
    Hyphy,
    Hyprlang,
    I3Config,
    #[strum(serialize = "inform-7", serialize = "inform7")]
    I7 as "i7",
    IBasic,
    #[strum(serialize = "icalendar")]
    Ical as "ical",
    IceMenu,
    Icon,
    Idl,
    Idlang,
    #[strum(serialize = "idris")]
    Idris as "idris2",
    #[strum(serialize = "igor-pro", serialize = "igorpro")]
    Igor as "igor",
    #[strum(serialize = "imagej-macro")]
    Ijm as "ijm",
    Imba,
    Indent,
    Inform,
    #[strum(serialize = "confini")]
    ConfIni as "ini",
    Initng,
    InitTab,
    Ink,
    Inko,
    InstallShield,
    Io,
    Ioke,
    IpFilter,
    Ipkg,
    #[strum(serialize = "irc-log", serialize = "irc-logs")]
    Irc as "irc",
    Isabelle,
    IsabelleRoot as "isabelle-root",
    Ispc,
    #[strum(serialize = "inno-setup")]
    Iss as "iss",
    Ist,
    J,
    Jac,
    Jai,
    Jal,
    Jam,
    #[strum(serialize = "janet_simple")]
    JanetSimple as "janet",
    JarManifest as "jar-manifest",
    Jasmin,
    Java,
    JavaCc,
    Javadoc,
    #[strum(serialize = "js", serialize = "node")]
    JavaScript as "javascript",
    JavascriptErb as "javascript-erb",
    #[strum(serialize = "gjs", serialize = "glimmer_javascript", serialize = "glimmer-js")]
    JavaScriptGlimmer as "javascript.glimmer",
    Jcl,
    #[strum(serialize = "clips")]
    Jess as "jess",
    JestSnapshot as "jest-snapshot",
    Jflex,
    JGraph,
    #[strum(serialize = "django", serialize = "html+django", serialize = "html+jinja")]
    Jinja as "jinja",
    JinjaInline as "jinja_inline",
    Jison,
    JisonLex as "jison-lex",
    Jjdescription,
    Jolie,
    Jovial,
    #[strum(serialize = "java-properties", serialize = "properties")]
    JProperties as "jproperties",
    #[strum(serialize = "jsoniq")]
    Jq as "jq",
    Jsdoc,
    #[strum(serialize = "geojson", serialize = "ipython-notebook", serialize = "jupyter-notebook", serialize = "oasv2-json", serialize = "oasv3-json", serialize = "sarif", serialize = "topojson")]
    Json as "json",
    Json5,
    #[strum(serialize = "json-with-comments")]
    JsonC as "jsonc",
    JsonL,
    Jsonld,
    Jsonnet,
    #[strum(serialize = "java-server-pages")]
    Jsp as "jsp",
    #[strum(serialize = "javascriptreact")]
    Jsx as "jsx",
    #[strum(serialize = "java-template-engine")]
    Jte as "jte",
    Julia,
    JuliaRepl as "julia-repl",
    #[strum(serialize = "justfile")]
    Just as "just",
    #[strum(serialize = "kakounescript", serialize = "kakscript")]
    Kak as "kak",
    Karel,
    Kcl,
    KConfig,
    Kdl,
    Kerml,
    KicadLegacyLayout as "kicad-legacy-layout",
    Kit,
    Kitty,
    #[strum(serialize = "kvlang")]
    Kivy as "kivy",
    Kix,
    Koka,
    KolmafiaAsh as "kolmafia-ash",
    Kos,
    Kotlin,
    Koto,
    Krl,
    #[strum(serialize = "kerboscript", serialize = "kickstart")]
    KScript as "kscript",
    Ksh,
    #[strum(serialize = "kaitai-struct")]
    Ksy as "ksy",
    Kusto,
    #[strum(serialize = "kframework")]
    Kwt as "kwt",
    Labview,
    Lace,
    Lalrpop,
    Lambdapi,
    LambdaProlog,
    Langium,
    Larch,
    Lark,
    #[strum(serialize = "lasso")]
    Lassoscript as "lassoscript",
    Latte,
    #[strum(serialize = "linker-script", serialize = "linkerscript")]
    Ld as "ld",
    Ldapconf,
    Ldif,
    #[strum(serialize = "lean-4", serialize = "lean4")]
    Lean as "lean",
    Ledger,
    Leex,
    Leo,
    #[strum(serialize = "less-css")]
    Less as "less",
    #[strum(serialize = "flex", serialize = "picolisp")]
    Lex as "lex",
    Lf,
    Lfe,
    Lftp,
    #[strum(serialize = "lhs", serialize = "literate-haskell")]
    LHaskell as "lhaskell",
    Libao,
    Lidris2,
    Ligolang,
    Lilo,
    Lilypond,
    Limbo,
    Limits,
    LinearProgramming as "linear-programming",
    LinuxKernelModule as "linux-kernel-module",
    Liquid,
    Liquidsoap,
    #[strum(serialize = "common-lisp", serialize = "commonlisp", serialize = "cool", serialize = "emacs", serialize = "emacs-lisp", serialize = "newlisp", serialize = "opencl")]
    Lisp as "lisp",
    #[strum(serialize = "literate-coffeescript")]
    Litcoffee as "litcoffee",
    Lite,
    LiterateAgda as "literate-agda",
    LiteStep,
    #[strum(serialize = "livescript", serialize = "ls")]
    LiveScript as "live-script",
    Livebook,
    LivecodeScript as "livecode-script",
    Llvm,
    LogCheck,
    LoginAccess,
    LoginDefs,
    Logtalk,
    Lolcode,
    Lookml,
    Loomscript,
    Lotos,
    Lout,
    Lpc,
    Lsl,
    Lss,
    Lua,
    Luadoc,
    Luap,
    Luau,
    Lynx,
    Lyrics,
    M17ndb,
    M3Build,
    M3Quake,
    M4,
    #[strum(serialize = "asm68k", serialize = "motorola-68k-assembly")]
    M68k as "m68k",
    Magik,
    #[strum(serialize = "e-mail", serialize = "email", serialize = "eml", serialize = "mbox")]
    Mail as "mail",
    MailAliases,
    MailCap,
    #[strum(serialize = "bsdmake", serialize = "makefile", serialize = "microsoft-developer-studio-project")]
    Make as "make",
    Mako,
    Mallard,
    ManConf,
    Map,
    #[strum(serialize = "jetbrains-mps", serialize = "mps")]
    Maple as "maple",
    #[strum(serialize = "gcc-machine-description", serialize = "md")]
    Markdown as "markdown",
    MarkdownInline as "markdown_inline",
    #[strum(serialize = "marko")]
    Markojs as "markojs",
    Mask,
    Masm,
    Mason,
    Master,
    Matlab,
    MavenPom as "maven-pom",
    #[strum(serialize = "max", serialize = "maxmsp")]
    MaxMsp as "max/msp",
    Maxima,
    Maxscript,
    Mbsync,
    Mcfunction,
    Mdsvex,
    Mdx,
    #[strum(serialize = "wiki", serialize = "wikitext")]
    Mediawiki as "mediawiki",
    Mel,
    Menhir,
    #[strum(serialize = "mermaid-example")]
    Mermaid as "mermaid",
    Meson,
    Messages,
    Metal,
    Mf,
    Mgl,
    Mgp,
    Mib,
    Minid,
    Minizinc,
    MinizincData as "minizinc-data",
    Mint,
    Mirah,
    MircScript as "mirc-script",
    Mix,
    Mlir,
    #[strum(serialize = "mathematica", serialize = "wl", serialize = "wolfram", serialize = "wolfram-lang", serialize = "wolfram-language")]
    Mma as "mma",
    Mmix,
    Mmp,
    ModConf,
    Modsim3,
    #[strum(serialize = "m2", serialize = "macaulay2", serialize = "modula-2")]
    Modula2 as "modula2",
    #[strum(serialize = "modula-3")]
    Modula3 as "modula3",
    ModuleManagementSystem as "module-management-system",
    Mojo,
    Monk,
    Monkey,
    MonkeyC as "monkey-c",
    #[strum(serialize = "mercury", serialize = "moocode")]
    Moo as "moo",
    Moonbit,
    MoonScript,
    Move,
    Mp,
    MpMetafun as "mp-metafun",
    MPlayerConf,
    Mql4,
    Mql5,
    Mrxvtrc,
    Msidl,
    MsMessages,
    Msmtp,
    Msql,
    #[strum(serialize = "carto", serialize = "cartocss")]
    Mss as "mss",
    Mtml,
    Muf,
    #[strum(serialize = "m")]
    Mumps as "mumps",
    Mupad,
    Murphi,
    Mush,
    Mustache,
    Muttrc,
    Myghty,
    MySql,
    N1ql,
    Named,
    Nanorc,
    Nasal,
    Nasl,
    #[strum(serialize = "assembly")]
    Nasm as "nasm",
    Natural,
    Ncf,
    Nearley,
    Nemerle,
    Neomuttlog,
    Neomuttrc,
    Nesc,
    Netlinx,
    NetlinxErb as "netlinx-erb",
    Netlogo,
    Netrc,
    #[strum(serialize = "ne-on", serialize = "neon")]
    NetteObjectNotation as "nette-object-notation",
    Nextflow,
    #[strum(serialize = "nginx-configuration-file")]
    Nginx as "nginx",
    #[strum(serialize = "ncl")]
    Nickel as "nickel",
    Nim,
    NimFormatString as "nim_format_string",
    Ninja,
    Nit,
    #[strum(serialize = "nixos")]
    Nix as "nix",
    #[strum(serialize = "nunjucks")]
    Njk as "njk",
    Nl,
    Nmodl,
    Norg,
    Nq,
    Nqc,
    #[strum(serialize = "man", serialize = "man-page", serialize = "manpage", serialize = "mdoc", serialize = "nargo", serialize = "noir", serialize = "roff", serialize = "roff-manpage", serialize = "troff")]
    Nroff as "nroff",
    Nsis,
    Ntriples,
    #[strum(serialize = "nu-script", serialize = "nush", serialize = "nushell", serialize = "nushell-script")]
    Nu as "nu",
    Numbat,
    Numpy,
    Nwscript,
    #[strum(serialize = "openapi-specification-v2")]
    Oasv2 as "oasv2",
    #[strum(serialize = "openapi-specification-v3")]
    Oasv3 as "oasv3",
    Oberon,
    #[strum(serialize = "wavefront-object")]
    Obj as "obj",
    #[strum(serialize = "objective-j", serialize = "objectivej", serialize = "objj")]
    ObjJ as "obj-j",
    #[strum(serialize = "obj-c", serialize = "obj-c++", serialize = "objc++", serialize = "objective-c", serialize = "objectivec", serialize = "objectivec++")]
    ObjC as "objc",
    ObjCpp,
    #[strum(serialize = "c++-objdump", serialize = "cpp-objdump")]
    Objdump as "objdump",
    Objectscript,
    Obse,
    OCaml,
    #[strum(serialize = "ocaml_interface", serialize = "ocaml-interface")]
    OCamlInterface as "ocamlinterface",
    Ocamllex,
    Occam,
    Octave,
    #[strum(serialize = "object-data-instance-notation", serialize = "odin-lang", serialize = "odinlang")]
    Odin as "odin",
    Omgrofl,
    #[strum(serialize = "omnet-msg")]
    OmnetppMsg as "omnetpp-msg",
    #[strum(serialize = "omnet-ned")]
    OmnetppNed as "omnetpp-ned",
    OmniMark,
    Ondir,
    Ooc,
    Opa,
    Opal,
    Opam,
    Openqasm,
    #[strum(serialize = "openrc-runscript")]
    Openrc as "openrc",
    OpenRoad,
    OpenScad,
    OpenstepPropertyList as "openstep-property-list",
    OpenVpn,
    Opl,
    #[strum(serialize = "ackrc", serialize = "option-list")]
    Opts as "opts",
    Ora,
    Org,
    Overpassql,
    Ox,
    Oxygene,
    Oz,
    P4,
    Pact,
    PamConf,
    PamEnv,
    Pan,
    #[strum(serialize = "pure-data")]
    Pandoc as "pandoc",
    Papp,
    Papyrus,
    Parrot,
    #[strum(serialize = "delphi", serialize = "objectpascal")]
    Pascal as "pascal",
    #[strum(serialize = "parrot-assembly")]
    Pasm as "pasm",
    Passwd,
    Pawn,
    #[strum(serialize = "protobuf-text-format", serialize = "protocol-buffer-text-format", serialize = "text-proto", serialize = "textproto")]
    Pbtxt as "pbtxt",
    #[strum(serialize = "kicad-layout")]
    Pcbnew as "pcbnew",
    #[strum(serialize = "g-code")]
    Pccts as "pccts",
    Pcmk,
    Pddl,
    Pdf,
    PegJs as "peg-js",
    Pem,
    Pep8,
    #[strum(serialize = "al", serialize = "cperl")]
    Perl as "perl",
    Pf,
    PfMain,
    Pgn,
    #[strum(serialize = "html-php", serialize = "inc")]
    Php as "php",
    #[strum(serialize = "php-only")]
    PhpOnly as "php_only",
    Phpdoc,
    Piglatin,
    #[strum(serialize = "pic")]
    Pikchr as "pikchr",
    Pike,
    Pilrc,
    Pine,
    PInfo,
    Pioasm,
    #[strum(serialize = "parrot-internal-representation")]
    Pir as "pir",
    #[strum(serialize = "pickle")]
    Pkl as "pkl",
    PlainTex,
    Plantuml,
    Pli,
    Plm,
    Plp,
    Plpgsql,
    Plsql,
    #[strum(serialize = "gettext-catalog", serialize = "pot")]
    Po as "po",
    #[strum(serialize = "pod-6")]
    Pod as "pod",
    #[strum(serialize = "poe_filter")]
    PoeFilter as "poefilter",
    Pogoscript,
    Poke,
    Polar,
    Pony,
    Portugol,
    Postcss,
    #[strum(serialize = "acfm", serialize = "adobe-composite-font-metrics", serialize = "adobe-font-metrics", serialize = "adobe-multiple-font-metrics", serialize = "amfm", serialize = "postscript")]
    Postscr as "postscr",
    #[strum(serialize = "pov-ray", serialize = "pov-ray-sdl", serialize = "povray")]
    Pov as "pov",
    PovIni,
    Powerbuilder,
    Ppd,
    Ppwiz,
    Pq,
    Praat,
    Printf,
    Prisma,
    #[strum(serialize = "ros-interface", serialize = "rosmsg")]
    Privoxy as "privoxy",
    Problog,
    Proc,
    Procfile,
    ProcMail,
    #[strum(serialize = "abl", serialize = "openedge", serialize = "openedge-abl")]
    Progress as "progress",
    Proguard,
    Prolog,
    Promela,
    Promql,
    PropellerSpin as "propeller-spin",
    #[strum(serialize = "protobuf", serialize = "protocol-buffer", serialize = "protocol-buffers")]
    Proto as "proto",
    Protocols,
    Prql,
    #[strum(serialize = "posh", serialize = "powershell", serialize = "pwsh")]
    Ps1 as "ps1",
    Ps1Xml,
    Psf,
    Psl,
    Psv,
    PtcapPrint as "ptcap-print",
    PtcapTerm as "ptcap-term",
    Ptx,
    PublicKey as "public-key",
    Pug,
    Puppet,
    Purebasic,
    Purescript,
    #[strum(serialize = "python-console")]
    Pycon as "pycon",
    Pymanifest,
    Pyret,
    #[strum(serialize = "cython")]
    Pyrex as "pyrex",
    #[strum(serialize = "python3", serialize = "rusthon")]
    Python as "python",
    PythonTraceback as "python-traceback",
    Q,
    #[strum(serialize = "classic-qbasic", serialize = "classic-quickbasic", serialize = "qb", serialize = "qbasic", serialize = "quickbasic")]
    Qb64 as "qb64",
    #[strum(serialize = "codeql")]
    Ql as "ql",
    Qmake,
    #[strum(serialize = "qmljs")]
    Qmljs as "qml",
    QmlDir,
    Qsharp,
    QtScript as "qt-script",
    Quake,
    #[strum(serialize = "rmarkdown")]
    Quarto as "quarto",
    TreeSitterQuery as "query",
    Quickbms,
    R,
    Racc,
    Racket,
    #[strum(serialize = "unity3d-asset")]
    Radiance as "radiance",
    #[strum(serialize = "ragel", serialize = "ragel-ruby")]
    RagelRb as "ragel-rb",
    #[strum(serialize = "perl-6", serialize = "perl6")]
    Raku as "raku",
    Ralph,
    Raml,
    Rapid,
    Rascript,
    Rasi,
    RatPoison,
    #[strum(serialize = "raw-token-data")]
    Raw as "raw",
    Razor,
    Rbs,
    Rc,
    Rcs,
    Rdoc,
    Re2c,
    #[strum(serialize = "inputrc", serialize = "readline-config")]
    Readline as "readline",
    Realbasic,
    Reason,
    Reasonligo,
    Rebol,
    RecordJar as "record-jar",
    #[strum(serialize = "red")]
    RedSystem as "red/system",
    Redcode,
    Redif,
    #[strum(serialize = "redirect-rules")]
    Redirects as "redirects",
    #[strum(serialize = "regexp", serialize = "regular-expression")]
    Regex as "regex",
    Registry,
    #[strum(serialize = "open-policy-agent")]
    Rego as "rego",
    Remind,
    #[strum(serialize = "ren-py")]
    Renpy as "renpy",
    #[strum(serialize = "pip-requirements")]
    Requirements as "requirements",
    ReScript,
    Resolv,
    Reva,
    #[strum(serialize = "arexx")]
    Rexx as "rexx",
    Rez,
    #[strum(serialize = "rscript", serialize = "splus")]
    RHelp as "rhelp",
    Rib,
    Rifleconf,
    Ring,
    Riot,
    Rmd,
    Rnc,
    Rng,
    #[strum(serialize = "sweave")]
    Rnoweb as "rnoweb",
    #[strum(serialize = "robotframework")]
    Robot as "robot",
    #[strum(serialize = "robots_txt", serialize = "robots-txt")]
    Robots as "robots",
    Roc,
    Ron,
    Rouge,
    #[strum(serialize = "rascal", serialize = "routeros-script")]
    RouterOs as "routeros",
    #[strum(serialize = "directx-3d-file", serialize = "logos", serialize = "oncrpc", serialize = "rpc", serialize = "xdr")]
    Rpcgen as "rpcgen",
    #[strum(serialize = "ile-rpg", serialize = "sqlrpgle")]
    Rpgle as "rpgle",
    Rpl,
    Rrst,
    #[strum(serialize = "restructuredtext")]
    Rst as "rst",
    #[strum(serialize = "rich-text-format")]
    Rtf as "rtf",
    #[strum(serialize = "jruby", serialize = "macruby", serialize = "rake", serialize = "rb", serialize = "rbx")]
    Ruby as "ruby",
    Runescript,
    Runoff,
    Rush,
    #[strum(serialize = "renderscript", serialize = "rs")]
    Rust as "rust",
    Sage,
    Sail,
    #[strum(serialize = "saltstack", serialize = "saltstate")]
    Salt as "salt",
    Samba,
    Sas,
    Sass,
    Sather,
    Sbt,
    Scala,
    Scaml,
    Scdoc,
    Scenic,
    Scfg,
    #[strum(serialize = "tree-sitter-query", serialize = "tsq")]
    Scheme as "scheme",
    Scilab,
    Screen,
    Scss,
    Sd,
    Sdc,
    Sdl,
    Sed,
    SelfLang as "self",
    #[strum(serialize = "selinux-policy", serialize = "sepolicy")]
    SelinuxKernelPolicyLanguage as "selinux-kernel-policy-language",
    Sensors,
    Services,
    SetSerial,
    Sexplib,
    Sflog,
    #[strum(serialize = "simple-file-verification")]
    Sfv as "sfv",
    Sgml,
    SgmlDecl,
    #[strum(serialize = "envrc", serialize = "shell", serialize = "shell-script")]
    Sh as "sh",
    #[strum(serialize = "slang", serialize = "slash")]
    Slang as "shaderslang",
    #[strum(serialize = "shellcheck-config")]
    Shellcheckrc as "shellcheckrc",
    Shen,
    Sieve,
    Sil,
    Sile,
    Simula,
    Sinda,
    Singularity,
    Sisu,
    Skhd,
    Skill,
    Slice,
    Slim,
    Slint,
    SlpConf,
    SlpReg,
    SlpSpi,
    Slrnrc,
    Slrnsc,
    Sm,
    Smali,
    Smarty,
    Smcl,
    Smgllnx,
    Smil,
    #[strum(serialize = "smt")]
    Smith as "smith",
    Smithy,
    #[strum(serialize = "standard-ml")]
    Sml as "sml",
    #[strum(serialize = "snakefile")]
    Snakemake as "snakemake",
    #[strum(serialize = "neosnippet", serialize = "ultisnip", serialize = "ultisnips", serialize = "vim-snippet")]
    Snipmate as "snipmate",
    #[strum(serialize = "yas", serialize = "yasnippet")]
    Snippet as "snippet",
    Snobol4,
    Solidity,
    #[strum(serialize = "microsoft-visual-studio-solution")]
    Solution as "solution",
    Soong,
    Soql,
    Sosl,
    Sourcepawn,
    #[strum(serialize = "closure-templates")]
    Soy as "soy",
    Spajson,
    Sparql,
    #[strum(serialize = "rpm-spec", serialize = "specfile")]
    Spec as "spec",
    SpecMan,
    #[strum(serialize = "sourcemod")]
    Spice as "spice",
    SplineFontDatabase as "spline-font-database",
    Sproto,
    Spup,
    Spyce,
    Sqf,
    Sql,
    Sqlj,
    Sqlpl,
    Sqr,
    Squid,
    Squirrel,
    Srec,
    #[strum(serialize = "srecode-template", serialize = "subrip-text")]
    Srt as "srt",
    Ssa,
    #[strum(serialize = "ssh_config", serialize = "ssh-config", serialize = "sshd_config")]
    SshConfig as "sshconfig",
    SshdConfig,
    #[strum(serialize = "smalltalk", serialize = "squeak", serialize = "stringtemplate")]
    St as "st",
    Stan,
    #[strum(serialize = "star")]
    Starlark as "starlark",
    Stata,
    Ston,
    Stp,
    Strace,
    Structurizr,
    Styled,
    Stylus,
    Sudoers,
    Sugarss,
    Supercollider,
    Superhtml,
    #[strum(serialize = "sface")]
    Surface as "surface",
    #[strum(serialize = "surrealql")]
    Surql as "surql",
    SurvexData as "survex-data",
    Svelte,
    Svg,
    Svn,
    Sway,
    SwayConfig,
    Swift,
    SwiftGyb,
    Swig,
    Sxhkdrc,
    Sysctl,
    Sysml,
    Systemd,
    Systemtap,
    #[strum(serialize = "sv", serialize = "svh", serialize = "verilog")]
    SystemVerilog as "systemverilog",
    Tablegen,
    Tact,
    Tads,
    Tags,
    Tak,
    #[strum(serialize = "uxntal")]
    Tal as "tal",
    Talon,
    TaskData,
    TaskEdit,
    #[strum(serialize = "xdc")]
    Tcl as "tcl",
    Tcsh,
    Tea,
    #[strum(serialize = "tl", serialize = "type-language")]
    Teal as "teal",
    Templ,
    #[strum(serialize = "go-template")]
    Template as "template",
    Tera,
    Teraterm,
    Terminfo,
    Terra,
    Terraform,
    TerraformTemplate as "terraform-template",
    TerraformVars as "terraform-vars",
    #[strum(serialize = "latex")]
    Tex as "tex",
    TexInfo,
    TexMF,
    Textgrid,
    Textile,
    Tf,
    Thrift,
    TiProgram as "ti-program",
    Tiasm,
    Tidy,
    Tiger,
    Tilde,
    Tiltfile,
    TlVerilog as "tl-verilog",
    #[strum(serialize = "tlaplus")]
    Tla as "tla",
    Tli,
    #[strum(serialize = "textmate-properties")]
    TmProperties as "tm-properties",
    Tmux,
    Todotxt,
    Toit,
    Toml,
    #[strum(serialize = "tor-config")]
    Torrc as "torrc",
    Tpp,
    #[strum(serialize = "t32")]
    Trace32 as "trace32",
    Trasys,
    Treetop,
    Trig,
    Trustees,
    Tsalt,
    Tsql,
    Tsscl,
    Tssgm,
    Tssop,
    #[strum(serialize = "tab-seperated-values")]
    Tsv as "tsv",
    #[strum(serialize = "typescriptreact")]
    Tsx as "tsx",
    Turing,
    Turtle,
    Tutor,
    Twig,
    Txl,
    #[strum(serialize = "ts")]
    TypeScript as "typescript",
    #[strum(serialize = "gerber-image", serialize = "glimmer_typescript", serialize = "glimmer-ts", serialize = "gts", serialize = "rs-274x")]
    TypeScriptGlimmer as "typescript.glimmer",
    #[strum(serialize = "traveling-salesman-problem", serialize = "travelling-salesman-problem", serialize = "tsp", serialize = "tsplib-data")]
    Typespec as "typespec",
    Typoscript,
    #[strum(serialize = "typ")]
    Typst as "typst",
    #[strum(serialize = "unrealscript")]
    Uc as "uc",
    UdevConf,
    UdevPerm,
    #[strum(serialize = "udev")]
    UdevRules as "udevrules",
    Uil,
    Ungrammar,
    UnifiedParallelC as "unified-parallel-c",
    Unison,
    Uno,
    UntypedPlutusCore as "untyped-plutus-core",
    UpdateDb,
    Upstart,
    UpstreamDat,
    UpstreamInstallLog,
    UpstreamLog,
    #[strum(serialize = "ur", serialize = "urweb")]
    UrWeb as "ur/web",
    UrlShortcut,
    Ursa,
    Usd,
    UsServerLog,
    Usw2KagtLog,
    #[strum(serialize = "vlang")]
    V as "v",
    Vala,
    #[strum(serialize = "classic-visual-basic", serialize = "vb-6", serialize = "vb-net", serialize = "vb.net", serialize = "vb6", serialize = "vbnet", serialize = "vbscript", serialize = "visual-basic", serialize = "visual-basic-6", serialize = "visual-basic-6-0", serialize = "visual-basic-classic", serialize = "visual-basic-net")]
    Vb as "vb",
    Vcl,
    #[strum(serialize = "keyvalues", serialize = "valve-data-format")]
    Vdf as "vdf",
    Vdmpp,
    Vdmrt,
    Vdmsl,
    Vera,
    Verilogams,
    Vgrindefs,
    Vhdl,
    #[strum(serialize = "tape")]
    Vhs as "vhs",
    #[strum(serialize = "nvim", serialize = "vba", serialize = "vim-script", serialize = "viml", serialize = "vimscript", serialize = "visual-basic-for-applications")]
    Vim as "vim",
    Vimhelp,
    VimInfo,
    Virata,
    #[strum(serialize = "electronic-business-card", serialize = "vcard")]
    VirtualContactFile as "virtual-contact-file",
    Vmasm,
    Volt,
    Voscm,
    Vrl,
    Vrml,
    Vroom,
    #[strum(serialize = "velocity", serialize = "velocity-template-language")]
    Vtl as "vtl",
    #[strum(serialize = "vento")]
    Vento as "vto",
    #[strum(serialize = "webvtt")]
    Vtt as "vtt",
    Vue,
    Vyper,
    #[strum(serialize = "wasm", serialize = "webassembly")]
    Wast as "wast",
    Wat,
    WavefrontMaterial as "wavefront-material",
    #[strum(serialize = "workflow-description-language")]
    Wdl as "wdl",
    Web,
    WebOntologyLanguage as "web-ontology-language",
    Webidl,
    WebMacro,
    #[strum(serialize = "wget-config", serialize = "wgetrc")]
    Wget as "wget",
    Wget2,
    Wgsl,
    WgslBevy as "wgsl_bevy",
    Whiley,
    Win32MessageFile as "win32-message-file",
    WinBatch,
    WindowsRegistryEntries as "windows-registry-entries",
    Wing,
    Wisp,
    #[strum(serialize = "webassembly-interface-type")]
    Wit as "wit",
    WitcherScript as "witcher-script",
    Wml,
    Wollok,
    #[strum(serialize = "wren")]
    Wrenlang as "wrenlang",
    Wsh,
    Wsml,
    WvDial,
    Wxml,
    XFontDirectoryIndex as "x-font-directory-index",
    Xbl,
    #[strum(serialize = "x-bitmap")]
    Xbm as "xbm",
    Xc,
    Xcompose,
    #[strum(serialize = "xresources")]
    XDefaults as "xdefaults",
    XF86Conf,
    XF86Conf3 as "xf86conf-3",
    XF86Conf4 as "xf86conf-4",
    Xf86conf3,
    Xhtml,
    Xinetd,
    Xmake,
    XMath,
    #[strum(serialize = "rss", serialize = "wsdl")]
    Xml as "xml",
    XmlPropertyList as "xml-property-list",
    #[strum(serialize = "genshi", serialize = "xml+kid")]
    XmlGenshi as "xml+genshi",
    XModMap,
    Xojo,
    Xonsh,
    Xpages,
    #[strum(serialize = "x-pixmap")]
    Xpm as "xpm",
    Xpm2,
    Xproc,
    XQuery,
    Xs,
    Xsd,
    #[strum(serialize = "xsl")]
    Xslt as "xslt",
    #[strum(serialize = "x10")]
    Xten as "xten",
    Xtend,
    Yacc,
    #[strum(serialize = "miniyaml", serialize = "oasv2-yaml", serialize = "oasv3-yaml", serialize = "yml")]
    Yaml as "yaml",
    Yang,
    Yara,
    Yasm,
    Yuck,
    Yul,
    Z8a,
    Zap,
    Zathurarc,
    Zephir,
    Zig,
    Ziggy,
    ZiggySchema as "ziggy_schema",
    Zil,
    Zimbu,
    ZimbuTempl,
    Zimpl,
    Zir,
    Zmodel,
    #[strum(serialize = "zenscript")]
    Zserio as "zserio",
    Zsh,

}
