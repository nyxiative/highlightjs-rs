extern crate lazy_static;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref HLJS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::with_capacity(0);
        map.insert("1C", "1c");
        map.insert("4D", "4d");
        map.insert("ABNF", "abnf");
        map.insert("Access logs", "accesslog");
        map.insert("Ada", "ada");
        map.insert("Arduino", "arduino");
        map.insert("ARM assembler", "armasm");
        map.insert("AVR assembler", "avrasm");
        map.insert("ActionScript", "actionscript");
        map.insert("Alan IF", "alan");
        map.insert("Alan", "ln");
        map.insert("AngelScript", "angelscript");
        map.insert("Apache", "apache");
        map.insert("AppleScript", "applescript");
        map.insert("Arcade", "arcade");
        map.insert("AsciiDoc", "asciidoc");
        map.insert("AspectJ", "aspectj");
        map.insert("AutoHotkey", "autohotkey");
        map.insert("AutoIt", "autoit");
        map.insert("Awk", "awk");
        map.insert("Bash", "bash");
        map.insert("Basic", "basic");
        map.insert("BBCode", "bbcode");
        map.insert("Blade (Laravel)", "blade");
        map.insert("BNF", "bnf");
        map.insert("Brainfuck", "brainfuck");
        map.insert("C#", "csharp");
        map.insert("C", "c");
        map.insert("C++", "cpp");
        map.insert("C/AL", "cal");
        map.insert("Cache Object Script", "cos");
        map.insert("CMake", "cmake");
        map.insert("Coq", "coq");
        map.insert("CSP", "csp");
        map.insert("CSS", "css");
        map.insert("Cap’n Proto", "capnproto");
        map.insert("Chaos", "chaos");
        map.insert("Chapel", "chapel");
        map.insert("Cisco CLI", "cisco");
        map.insert("Clojure", "clojure");
        map.insert("CoffeeScript", "coffeescript");
        map.insert("CpcdosC+", "cpc");
        map.insert("Crmsh", "crmsh");
        map.insert("Crystal", "crystal");
        map.insert("Cypher (Neo4j)", "cypher");
        map.insert("D", "d");
        map.insert("DNS Zone file", "dns");
        map.insert("DOS", "dos");
        map.insert("Dart", "dart");
        map.insert("Delphi", "delphi");
        map.insert("Diff", "diff");
        map.insert("Django", "django");
        map.insert("Dockerfile", "dockerfile");
        map.insert("dsconfig", "dsconfig");
        map.insert("DTS (Device Tree)", "dts");
        map.insert("Dust", "dust");
        map.insert("Dylan", "dylan");
        map.insert("EBNF", "ebnf");
        map.insert("Elixir", "elixir");
        map.insert("Elm", "elm");
        map.insert("Erlang", "erlang");
        map.insert("Excel", "excel");
        map.insert("Extempore", "extempore");
        map.insert("F#", "fsharp");
        map.insert("FIX", "fix");
        map.insert("Fortran", "fortran");
        map.insert("G-Code", "gcode");
        map.insert("Gams", "gams");
        map.insert("GAUSS", "gauss");
        map.insert("GDScript", "godot");
        map.insert("Gherkin", "gherkin");
        map.insert("GN for Ninja", "gn");
        map.insert("Go", "go");
        map.insert("Grammatical Framework", "gf");
        map.insert("Golo", "golo");
        map.insert("Gradle", "gradle");
        map.insert("Groovy", "groovy");
        map.insert("HTML, XML", "xml");
        map.insert("HTTP", "http");
        map.insert("Haml", "haml");
        map.insert("Handlebars", "handlebars");
        map.insert("Haskell", "haskell");
        map.insert("Haxe", "haxe");
        map.insert("HLSL", "hlsl");
        map.insert("Hy", "hy");
        map.insert("Ini", "ini");
        map.insert("TOML", "toml");
        map.insert("Inform7", "inform7");
        map.insert("IRPF90", "irpf90");
        map.insert("JSON", "json");
        map.insert("Java", "java");
        map.insert("JavaScript", "javascript");
        map.insert("Jolie", "jolie");
        map.insert("Julia", "julia");
        map.insert("Kotlin", "kotlin");
        map.insert("LaTeX", "tex");
        map.insert("Leaf", "leaf");
        map.insert("Lean", "lean");
        map.insert("Lasso", "lasso");
        map.insert("Less", "less");
        map.insert("LDIF", "ldif");
        map.insert("Lisp", "lisp");
        map.insert("LiveCode Server", "livecodeserver");
        map.insert("LiveScript", "livescript");
        map.insert("Lua", "lua");
        map.insert("Makefile", "makefile");
        map.insert("Markdown", "markdown");
        map.insert("Mathematica", "mathematica");
        map.insert("Matlab", "matlab");
        map.insert("Maxima", "maxima");
        map.insert("Maya Embedded Language", "mel");
        map.insert("Mercury", "mercury");
        map.insert("mIRC Scripting Language", "mirc");
        map.insert("Mizar", "mizar");
        map.insert("Mojolicious", "mojolicious");
        map.insert("Monkey", "monkey");
        map.insert("Moonscript", "moonscript");
        map.insert("N1QL", "n1ql");
        map.insert("NSIS", "nsis");
        map.insert("Never", "never");
        map.insert("Nginx", "nginx");
        map.insert("Nim", "nim");
        map.insert("Nix", "nix");
        map.insert("Object Constraint Language", "ocl");
        map.insert("OCaml", "ocaml");
        map.insert("Objective C", "objectivec");
        map.insert("OpenGL Shading Language", "glsl");
        map.insert("OpenSCAD", "openscad");
        map.insert("Oracle Rules Language", "ruleslanguage");
        map.insert("Oxygene", "oxygene");
        map.insert("PF", "pf");
        map.insert("PHP", "php");
        map.insert("Parser3", "parser3");
        map.insert("Perl", "perl");
        map.insert("Plaintext", "plaintext");
        map.insert("Pony", "pony");
        map.insert("PostgreSQL", "pgsql");
        map.insert("PowerShell", "powershell");
        map.insert("Processing", "processing");
        map.insert("Prolog", "prolog");
        map.insert("Properties", "properties");
        map.insert("Protocol Buffers", "protobuf");
        map.insert("Puppet", "puppet");
        map.insert("Python", "python");
        map.insert("Python profiler results", "profile");
        map.insert("Python REPL", "python-repl");
        map.insert("Q#", "qsharp");
        map.insert("Q", "k");
        map.insert("QML", "qml");
        map.insert("R", "r");
        map.insert("Razor CSHTML", "cshtml");
        map.insert("ReasonML", "reasonml");
        map.insert("Rebol & Red", "redbol");
        map.insert("RenderMan RIB", "rib");
        map.insert("RenderMan RSL", "rsl");
        map.insert("RiScript", "risc");
        map.insert("Roboconf", "graph");
        map.insert("Robot Framework", "robot");
        map.insert("RPM spec files", "rpm-specfile");
        map.insert("Ruby", "ruby");
        map.insert("Rust", "rust");
        map.insert("SAS", "SAS");
        map.insert("SCSS", "scss");
        map.insert("SQL", "sql");
        map.insert("STEP Part 21", "p21");
        map.insert("Scala", "scala");
        map.insert("Scheme", "scheme");
        map.insert("Scilab", "scilab");
        map.insert("Shape Expressions", "shexc");
        map.insert("Shell", "shell");
        map.insert("Smali", "smali");
        map.insert("Smalltalk", "smalltalk");
        map.insert("SML", "sml");
        map.insert("Solidity", "solidity");
        map.insert("Stan", "stan");
        map.insert("Stata", "stata");
        map.insert("Structured Text", "iecst");
        map.insert("Stylus", "stylus");
        map.insert("SubUnit", "subunit");
        map.insert("Supercollider", "supercollider");
        map.insert("Svelte", "svelte");
        map.insert("Swift", "swift");
        map.insert("Tcl", "tcl");
        map.insert("Terraform (HCL)", "terraform");
        map.insert("Test Anything Protocol", "tap");
        map.insert("Thrift", "thrift");
        map.insert("TP", "tp");
        map.insert("Transact-SQL", "tsql");
        map.insert("Twig", "twig");
        map.insert("TypeScript", "typescript");
        map.insert("Unicorn Rails log", "unicorn-rails-log");
        map.insert("VB.Net", "vbnet");
        map.insert("VBA", "vba");
        map.insert("VBScript", "vbscript");
        map.insert("VHDL", "vhdl");
        map.insert("Vala", "vala");
        map.insert("Verilog", "verilog");
        map.insert("Vim Script", "vim");
        map.insert("X++", "axapta");
        map.insert("x86 Assembly", "x86asm");
        map.insert("XL", "xl");
        map.insert("XQuery", "xquery");
        map.insert("YAML", "yml");
        map.insert("Zephir", "zephir");
        map
    };
}

pub fn custom_fuzzy<FM: fuzzy_matcher::FuzzyMatcher>(search: &str, matcher: &FM) -> Option<&'static str> {
    let mut best = None;
    let mut best_score = -1i64;
    for (&k,&v) in HLJS.iter() {
        let score = matcher.fuzzy_match(k, search).unwrap_or(-1);
        if score > best_score {
            best_score = score;
            best = Some(v);
        }
    }
    best
}

pub fn fuzzy(search: &str) -> Option<&'static str> {
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default().ignore_case();
    custom_fuzzy(search, &matcher)
}

pub fn exact(language: &str) -> Option<&'static str> {
    HLJS.get(language).map(|r| *r)
}

pub fn case_insensitive(language: &str) -> Option<&'static str> {
    let lower_lang = language.to_lowercase();
    for (k,&v) in HLJS.iter() {
        if k.to_lowercase() == lower_lang {
            return Some(v);
        }
    }
    None
}

pub fn from_id(id: usize) -> Option<&'static str> {
    for (i, (_, &v)) in HLJS.iter().enumerate() {
        if i == id {
            return Some(v);
        }
    }
    None
}

pub fn to_id(code: &str) -> Option<usize> {
    match code {
        "1c"=> Some(0),
        "4d"=> Some(1),
        "abnf"=> Some(2),
        "accesslog"=> Some(3),
        "ada"=> Some(4),
        "arduino"=> Some(5),
        "armasm"=> Some(6),
        "avrasm"=> Some(7),
        "actionscript"=> Some(8),
        "alan"=> Some(9),
        "ln"=> Some(10),
        "angelscript"=> Some(11),
        "apache"=> Some(12),
        "applescript"=> Some(13),
        "arcade"=> Some(14),
        "asciidoc"=> Some(15),
        "aspectj"=> Some(16),
        "autohotkey"=> Some(17),
        "autoit"=> Some(18),
        "awk"=> Some(19),
        "bash"=> Some(20),
        "basic"=> Some(21),
        "bbcode"=> Some(22),
        "blade"=> Some(23),
        "bnf"=> Some(24),
        "brainfuck"=> Some(25),
        "csharp"=> Some(26),
        "c"=> Some(27),
        "cpp"=> Some(28),
        "cal"=> Some(29),
        "cos"=> Some(30),
        "cmake"=> Some(31),
        "coq"=> Some(32),
        "csp"=> Some(33),
        "css"=> Some(34),
        "capnproto"=> Some(35),
        "chaos"=> Some(36),
        "chapel"=> Some(37),
        "cisco"=> Some(38),
        "clojure"=> Some(39),
        "coffeescript"=> Some(40),
        "cpc"=> Some(41),
        "crmsh"=> Some(42),
        "crystal"=> Some(43),
        "cypher"=> Some(44),
        "d"=> Some(45),
        "dns"=> Some(46),
        "dos"=> Some(47),
        "dart"=> Some(48),
        "delphi"=> Some(49),
        "diff"=> Some(50),
        "django"=> Some(51),
        "dockerfile"=> Some(52),
        "dsconfig"=> Some(53),
        "dts"=> Some(54),
        "dust"=> Some(55),
        "dylan"=> Some(56),
        "ebnf"=> Some(57),
        "elixir"=> Some(58),
        "elm"=> Some(59),
        "erlang"=> Some(60),
        "excel"=> Some(61),
        "extempore"=> Some(62),
        "fsharp"=> Some(63),
        "fix"=> Some(64),
        "fortran"=> Some(65),
        "gcode"=> Some(66),
        "gams"=> Some(67),
        "gauss"=> Some(68),
        "godot"=> Some(69),
        "gherkin"=> Some(70),
        "gn"=> Some(71),
        "go"=> Some(72),
        "gf"=> Some(73),
        "golo"=> Some(74),
        "gradle"=> Some(75),
        "groovy"=> Some(76),
        "xml"=> Some(77),
        "http"=> Some(78),
        "haml"=> Some(79),
        "handlebars"=> Some(80),
        "haskell"=> Some(81),
        "haxe"=> Some(82),
        "hlsl"=> Some(83),
        "hy"=> Some(84),
        "ini"=> Some(85),
        "inform7"=> Some(86),
        "irpf90"=> Some(87),
        "json"=> Some(88),
        "java"=> Some(89),
        "javascript"=> Some(90),
        "jolie"=> Some(91),
        "julia"=> Some(92),
        "kotlin"=> Some(93),
        "tex"=> Some(94),
        "leaf"=> Some(95),
        "lean"=> Some(96),
        "lasso"=> Some(97),
        "less"=> Some(98),
        "ldif"=> Some(99),
        "lisp"=> Some(100),
        "livecodeserver"=> Some(101),
        "livescript"=> Some(102),
        "lua"=> Some(103),
        "makefile"=> Some(104),
        "markdown"=> Some(105),
        "mathematica"=> Some(106),
        "matlab"=> Some(107),
        "maxima"=> Some(108),
        "mel"=> Some(109),
        "mercury"=> Some(110),
        "mirc"=> Some(111),
        "mizar"=> Some(112),
        "mojolicious"=> Some(113),
        "monkey"=> Some(114),
        "moonscript"=> Some(115),
        "n1ql"=> Some(116),
        "nsis"=> Some(117),
        "never"=> Some(118),
        "nginx"=> Some(119),
        "nim"=> Some(120),
        "nix"=> Some(121),
        "ocl"=> Some(122),
        "ocaml"=> Some(123),
        "objectivec"=> Some(124),
        "glsl"=> Some(125),
        "openscad"=> Some(126),
        "ruleslanguage"=> Some(127),
        "oxygene"=> Some(128),
        "pf"=> Some(129),
        "php"=> Some(130),
        "parser3"=> Some(131),
        "perl"=> Some(132),
        "plaintext"=> Some(133),
        "pony"=> Some(134),
        "pgsql"=> Some(135),
        "powershell"=> Some(136),
        "processing"=> Some(137),
        "prolog"=> Some(138),
        "properties"=> Some(139),
        "protobuf"=> Some(140),
        "puppet"=> Some(141),
        "python"=> Some(142),
        "profile"=> Some(143),
        "python-repl"=> Some(144),
        "qsharp"=> Some(145),
        "k"=> Some(146),
        "qml"=> Some(147),
        "r"=> Some(148),
        "cshtml"=> Some(149),
        "reasonml"=> Some(150),
        "redbol"=> Some(151),
        "rib"=> Some(152),
        "rsl"=> Some(153),
        "risc"=> Some(154),
        "graph"=> Some(155),
        "robot"=> Some(156),
        "rpm-specfile"=> Some(157),
        "ruby"=> Some(158),
        "rust"=> Some(159),
        "SAS"=> Some(160),
        "scss"=> Some(161),
        "sql"=> Some(162),
        "p21"=> Some(163),
        "scala"=> Some(164),
        "scheme"=> Some(165),
        "scilab"=> Some(166),
        "shexc"=> Some(167),
        "shell"=> Some(168),
        "smali"=> Some(169),
        "smalltalk"=> Some(170),
        "sml"=> Some(171),
        "solidity"=> Some(172),
        "stan"=> Some(173),
        "stata"=> Some(174),
        "iecst"=> Some(175),
        "stylus"=> Some(176),
        "subunit"=> Some(177),
        "supercollider"=> Some(178),
        "svelte"=> Some(179),
        "swift"=> Some(180),
        "tcl"=> Some(181),
        "terraform"=> Some(182),
        "tap"=> Some(183),
        "thrift"=> Some(184),
        "tp"=> Some(185),
        "tsql"=> Some(186),
        "twig"=> Some(187),
        "typescript"=> Some(188),
        "unicorn-rails-log"=> Some(189),
        "vbnet"=> Some(190),
        "vba"=> Some(191),
        "vbscript"=> Some(192),
        "vhdl"=> Some(193),
        "vala"=> Some(194),
        "verilog"=> Some(195),
        "vim"=> Some(196),
        "axapta"=> Some(197),
        "x86asm"=> Some(198),
        "xl"=> Some(199),
        "xquery"=> Some(200),
        "yml"=> Some(201),
        "zephir"=> Some(202),
        _ => None
    }
}

#[cfg(test)]
mod lookup {
    use crate::exact as exactfn;
    use crate::case_insensitive as case_insensitivefn;

    #[test]
    fn exact() {
        assert_eq!(exactfn("Rust"), Some("rust"));
        assert_eq!(exactfn("Swift"), Some("swift"));
        assert_eq!(exactfn("TypeScript"), Some("typescript"));
        assert_eq!(exactfn("C++"), Some("cpp"));
        assert_eq!(exactfn("Python"), Some("python"));
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(case_insensitivefn("rust"), Some("rust"));
        assert_eq!(case_insensitivefn("RuSt"), Some("rust"));
        assert_eq!(case_insensitivefn("swift"), Some("swift"));
        assert_eq!(case_insensitivefn("sWifT"), Some("swift"));
        assert_eq!(case_insensitivefn("typescript"), Some("typescript"));
        assert_eq!(case_insensitivefn("Typescript"), Some("typescript"));
        assert_eq!(case_insensitivefn("c++"), Some("cpp"));
        assert_eq!(case_insensitivefn("C++"), Some("cpp"));
        assert_eq!(case_insensitivefn("python"), Some("python"));
        assert_eq!(case_insensitivefn("Python"), Some("python"));

    }
}

#[cfg(test)]
mod fuzzymatch {
    use crate::fuzzy;
    
    #[test]
    fn postgres() {
        assert_eq!(fuzzy("Postgres"), Some("pgsql"));
    }
    
    #[test]
    fn objectivec() {
        assert_eq!(fuzzy("Objective"), Some("objectivec"));
        assert_eq!(fuzzy("Objective C"), Some("objectivec"));
    }
    
    #[test]
    fn scad() {
        assert_eq!(fuzzy("SCAD"), Some("openscad"));
    }
    
    #[test]
    fn powershell() {
        assert_eq!(fuzzy("pwsh"), Some("powershell"));
    }
    
    #[test]
    fn protobuf() {
        assert_eq!(fuzzy("protobuf"), Some("protobuf"));
        assert_eq!(fuzzy("probuf"), Some("protobuf"));
        assert_eq!(fuzzy("pbuf"), Some("protobuf"));
        assert_eq!(fuzzy("ProtoBuffer"), Some("protobuf"));
        assert_eq!(fuzzy("Proto Buffer"), Some("protobuf"));
    }
    
    #[test]
    fn plaintext() {
        assert_eq!(fuzzy("Plain"), Some("plaintext"));
    }
}

#[cfg(test)]
mod id_test {
    use super::to_id as id;
    #[test]
    fn onec () {
        assert_eq!(id("1c"), Some(0));
    }
    #[test]
    fn fourd () {
        assert_eq!(id("4d"), Some(1));
    }
    #[test]
    fn abnf () {
        assert_eq!(id("abnf"), Some(2));
    }
    #[test]
    fn access_logs () {
        assert_eq!(id("accesslog"), Some(3));
    }
    #[test]
    fn ada () {
        assert_eq!(id("ada"), Some(4));
    }
    #[test]
    fn arduino () {
        assert_eq!(id("arduino"), Some(5));
    }
    #[test]
    fn arm_assembler () {
        assert_eq!(id("armasm"), Some(6));
    }
    #[test]
    fn avr_assembler () {
        assert_eq!(id("avrasm"), Some(7));
    }
    #[test]
    fn actionscript () {
        assert_eq!(id("actionscript"), Some(8));
    }
    #[test]
    fn alan_if () {
        assert_eq!(id("alan"), Some(9));
    }
    #[test]
    fn alan () {
        assert_eq!(id("ln"), Some(10));
    }
    #[test]
    fn angelscript () {
        assert_eq!(id("angelscript"), Some(11));
    }
    #[test]
    fn apache () {
        assert_eq!(id("apache"), Some(12));
    }
    #[test]
    fn applescript () {
        assert_eq!(id("applescript"), Some(13));
    }
    #[test]
    fn arcade () {
        assert_eq!(id("arcade"), Some(14));
    }
    #[test]
    fn asciidoc () {
        assert_eq!(id("asciidoc"), Some(15));
    }
    #[test]
    fn aspectj () {
        assert_eq!(id("aspectj"), Some(16));
    }
    #[test]
    fn autohotkey () {
        assert_eq!(id("autohotkey"), Some(17));
    }
    #[test]
    fn autoit () {
        assert_eq!(id("autoit"), Some(18));
    }
    #[test]
    fn awk () {
        assert_eq!(id("awk"), Some(19));
    }
    #[test]
    fn bash () {
        assert_eq!(id("bash"), Some(20));
    }
    #[test]
    fn basic () {
        assert_eq!(id("basic"), Some(21));
    }
    #[test]
    fn bbcode () {
        assert_eq!(id("bbcode"), Some(22));
    }
    #[test]
    fn blade () {
        assert_eq!(id("blade"), Some(23));
    }
    #[test]
    fn bnf () {
        assert_eq!(id("bnf"), Some(24));
    }
    #[test]
    fn brainfuck () {
        assert_eq!(id("brainfuck"), Some(25));
    }
    #[test]
    fn csharp () {
        assert_eq!(id("csharp"), Some(26));
    }
    #[test]
    fn c () {
        assert_eq!(id("c"), Some(27));
    }
    #[test]
    fn cpp () {
        assert_eq!(id("cpp"), Some(28));
    }
    #[test]
    fn cal () {
        assert_eq!(id("cal"), Some(29));
    }
    #[test]
    fn cache_object_script () {
        assert_eq!(id("cos"), Some(30));
    }
    #[test]
    fn cmake () {
        assert_eq!(id("cmake"), Some(31));
    }
    #[test]
    fn coq () {
        assert_eq!(id("coq"), Some(32));
    }
    #[test]
    fn csp () {
        assert_eq!(id("csp"), Some(33));
    }
    #[test]
    fn css () {
        assert_eq!(id("css"), Some(34));
    }
    #[test]
    fn capn_proto () {
        assert_eq!(id("capnproto"), Some(35));
    }
    #[test]
    fn chaos () {
        assert_eq!(id("chaos"), Some(36));
    }
    #[test]
    fn chapel () {
        assert_eq!(id("chapel"), Some(37));
    }
    #[test]
    fn cisco_cli () {
        assert_eq!(id("cisco"), Some(38));
    }
    #[test]
    fn clojure () {
        assert_eq!(id("clojure"), Some(39));
    }
    #[test]
    fn coffeescript () {
        assert_eq!(id("coffeescript"), Some(40));
    }
    #[test]
    fn cpcdoscp () {
        assert_eq!(id("cpc"), Some(41));
    }
    #[test]
    fn crmsh () {
        assert_eq!(id("crmsh"), Some(42));
    }
    #[test]
    fn crystal () {
        assert_eq!(id("crystal"), Some(43));
    }
    #[test]
    fn cypher () {
        assert_eq!(id("cypher"), Some(44));
    }
    #[test]
    fn d () {
        assert_eq!(id("d"), Some(45));
    }
    #[test]
    fn dns_zone_file () {
        assert_eq!(id("dns"), Some(46));
    }
    #[test]
    fn dos () {
        assert_eq!(id("dos"), Some(47));
    }
    #[test]
    fn dart () {
        assert_eq!(id("dart"), Some(48));
    }
    #[test]
    fn delphi () {
        assert_eq!(id("delphi"), Some(49));
    }
    #[test]
    fn diff () {
        assert_eq!(id("diff"), Some(50));
    }
    #[test]
    fn django () {
        assert_eq!(id("django"), Some(51));
    }
    #[test]
    fn dockerfile () {
        assert_eq!(id("dockerfile"), Some(52));
    }
    #[test]
    fn dsconfig () {
        assert_eq!(id("dsconfig"), Some(53));
    }
    #[test]
    fn dts () {
        assert_eq!(id("dts"), Some(54));
    }
    #[test]
    fn dust () {
        assert_eq!(id("dust"), Some(55));
    }
    #[test]
    fn dylan () {
        assert_eq!(id("dylan"), Some(56));
    }
    #[test]
    fn ebnf () {
        assert_eq!(id("ebnf"), Some(57));
    }
    #[test]
    fn elixir () {
        assert_eq!(id("elixir"), Some(58));
    }
    #[test]
    fn elm () {
        assert_eq!(id("elm"), Some(59));
    }
    #[test]
    fn erlang () {
        assert_eq!(id("erlang"), Some(60));
    }
    #[test]
    fn excel () {
        assert_eq!(id("excel"), Some(61));
    }
    #[test]
    fn extempore () {
        assert_eq!(id("extempore"), Some(62));
    }
    #[test]
    fn fsharp () {
        assert_eq!(id("fsharp"), Some(63));
    }
    #[test]
    fn fix () {
        assert_eq!(id("fix"), Some(64));
    }
    #[test]
    fn fortran () {
        assert_eq!(id("fortran"), Some(65));
    }
    #[test]
    fn gcode () {
        assert_eq!(id("gcode"), Some(66));
    }
    #[test]
    fn gams () {
        assert_eq!(id("gams"), Some(67));
    }
    #[test]
    fn gauss () {
        assert_eq!(id("gauss"), Some(68));
    }
    #[test]
    fn gdscript () {
        assert_eq!(id("godot"), Some(69));
    }
    #[test]
    fn gherkin () {
        assert_eq!(id("gherkin"), Some(70));
    }
    #[test]
    fn gn_for_ninja () {
        assert_eq!(id("gn"), Some(71));
    }
    #[test]
    fn go () {
        assert_eq!(id("go"), Some(72));
    }
    #[test]
    fn grammatical_framework () {
        assert_eq!(id("gf"), Some(73));
    }
    #[test]
    fn golo () {
        assert_eq!(id("golo"), Some(74));
    }
    #[test]
    fn gradle () {
        assert_eq!(id("gradle"), Some(75));
    }
    #[test]
    fn groovy () {
        assert_eq!(id("groovy"), Some(76));
    }
    #[test]
    fn html_xml () {
        assert_eq!(id("xml"), Some(77));
    }
    #[test]
    fn http () {
        assert_eq!(id("http"), Some(78));
    }
    #[test]
    fn haml () {
        assert_eq!(id("haml"), Some(79));
    }
    #[test]
    fn handlebars () {
        assert_eq!(id("handlebars"), Some(80));
    }
    #[test]
    fn haskell () {
        assert_eq!(id("haskell"), Some(81));
    }
    #[test]
    fn haxe () {
        assert_eq!(id("haxe"), Some(82));
    }
    #[test]
    fn highlevel_shader_language () {
        assert_eq!(id("hlsl"), Some(83));
    }
    #[test]
    fn hy () {
        assert_eq!(id("hy"), Some(84));
    }
    #[test]
    fn ini_toml () {
        assert_eq!(id("ini"), Some(85));
    }
    #[test]
    fn inform7 () {
        assert_eq!(id("inform7"), Some(86));
    }
    #[test]
    fn irpf90 () {
        assert_eq!(id("irpf90"), Some(87));
    }
    #[test]
    fn json () {
        assert_eq!(id("json"), Some(88));
    }
    #[test]
    fn java () {
        assert_eq!(id("java"), Some(89));
    }
    #[test]
    fn javascript () {
        assert_eq!(id("javascript"), Some(90));
    }
    #[test]
    fn jolie () {
        assert_eq!(id("jolie"), Some(91));
    }
    #[test]
    fn julia () {
        assert_eq!(id("julia"), Some(92));
    }
    #[test]
    fn kotlin () {
        assert_eq!(id("kotlin"), Some(93));
    }
    #[test]
    fn latex () {
        assert_eq!(id("tex"), Some(94));
    }
    #[test]
    fn leaf () {
        assert_eq!(id("leaf"), Some(95));
    }
    #[test]
    fn lean () {
        assert_eq!(id("lean"), Some(96));
    }
    #[test]
    fn lasso () {
        assert_eq!(id("lasso"), Some(97));
    }
    #[test]
    fn less () {
        assert_eq!(id("less"), Some(98));
    }
    #[test]
    fn ldif () {
        assert_eq!(id("ldif"), Some(99));
    }
    #[test]
    fn lisp () {
        assert_eq!(id("lisp"), Some(100));
    }
    #[test]
    fn livecode_server () {
        assert_eq!(id("livecodeserver"), Some(101));
    }
    #[test]
    fn livescript () {
        assert_eq!(id("livescript"), Some(102));
    }
    #[test]
    fn lua () {
        assert_eq!(id("lua"), Some(103));
    }
    #[test]
    fn makefile () {
        assert_eq!(id("makefile"), Some(104));
    }
    #[test]
    fn markdown () {
        assert_eq!(id("markdown"), Some(105));
    }
    #[test]
    fn mathematica () {
        assert_eq!(id("mathematica"), Some(106));
    }
    #[test]
    fn matlab () {
        assert_eq!(id("matlab"), Some(107));
    }
    #[test]
    fn maxima () {
        assert_eq!(id("maxima"), Some(108));
    }
    #[test]
    fn maya_embedded_language () {
        assert_eq!(id("mel"), Some(109));
    }
    #[test]
    fn mercury () {
        assert_eq!(id("mercury"), Some(110));
    }
    #[test]
    fn mirc_scripting_language () {
        assert_eq!(id("mirc"), Some(111));
    }
    #[test]
    fn mizar () {
        assert_eq!(id("mizar"), Some(112));
    }
    #[test]
    fn mojolicious () {
        assert_eq!(id("mojolicious"), Some(113));
    }
    #[test]
    fn monkey () {
        assert_eq!(id("monkey"), Some(114));
    }
    #[test]
    fn moonscript () {
        assert_eq!(id("moonscript"), Some(115));
    }
    #[test]
    fn noneql () {
        assert_eq!(id("n1ql"), Some(116));
    }
    #[test]
    fn nsis () {
        assert_eq!(id("nsis"), Some(117));
    }
    #[test]
    fn never () {
        assert_eq!(id("never"), Some(118));
    }
    #[test]
    fn nginx () {
        assert_eq!(id("nginx"), Some(119));
    }
    #[test]
    fn nim () {
        assert_eq!(id("nim"), Some(120));
    }
    #[test]
    fn nix () {
        assert_eq!(id("nix"), Some(121));
    }
    #[test]
    fn object_constraint_language () {
        assert_eq!(id("ocl"), Some(122));
    }
    #[test]
    fn ocaml () {
        assert_eq!(id("ocaml"), Some(123));
    }
    #[test]
    fn objective_c () {
        assert_eq!(id("objectivec"), Some(124));
    }
    #[test]
    fn opengl_shading_language () {
        assert_eq!(id("glsl"), Some(125));
    }
    #[test]
    fn openscad () {
        assert_eq!(id("openscad"), Some(126));
    }
    #[test]
    fn oracle_rules_language () {
        assert_eq!(id("ruleslanguage"), Some(127));
    }
    #[test]
    fn oxygene () {
        assert_eq!(id("oxygene"), Some(128));
    }
    #[test]
    fn pf () {
        assert_eq!(id("pf"), Some(129));
    }
    #[test]
    fn php () {
        assert_eq!(id("php"), Some(130));
    }
    #[test]
    fn parser3 () {
        assert_eq!(id("parser3"), Some(131));
    }
    #[test]
    fn perl () {
        assert_eq!(id("perl"), Some(132));
    }
    #[test]
    fn plaintext () {
        assert_eq!(id("plaintext"), Some(133));
    }
    #[test]
    fn pony () {
        assert_eq!(id("pony"), Some(134));
    }
    #[test]
    fn postgresqlplpgsql () {
        assert_eq!(id("pgsql"), Some(135));
    }
    #[test]
    fn powershell () {
        assert_eq!(id("powershell"), Some(136));
    }
    #[test]
    fn processing () {
        assert_eq!(id("processing"), Some(137));
    }
    #[test]
    fn prolog () {
        assert_eq!(id("prolog"), Some(138));
    }
    #[test]
    fn properties () {
        assert_eq!(id("properties"), Some(139));
    }
    #[test]
    fn protocol_buffers () {
        assert_eq!(id("protobuf"), Some(140));
    }
    #[test]
    fn puppet () {
        assert_eq!(id("puppet"), Some(141));
    }
    #[test]
    fn python () {
        assert_eq!(id("python"), Some(142));
    }
    #[test]
    fn python_profiler_results () {
        assert_eq!(id("profile"), Some(143));
    }
    #[test]
    fn python_repl () {
        assert_eq!(id("python-repl"), Some(144));
    }
    #[test]
    fn qsharp () {
        assert_eq!(id("qsharp"), Some(145));
    }
    #[test]
    fn q () {
        assert_eq!(id("k"), Some(146));
    }
    #[test]
    fn qml () {
        assert_eq!(id("qml"), Some(147));
    }
    #[test]
    fn r () {
        assert_eq!(id("r"), Some(148));
    }
    #[test]
    fn razor_cshtml () {
        assert_eq!(id("cshtml"), Some(149));
    }
    #[test]
    fn reasonml () {
        assert_eq!(id("reasonml"), Some(150));
    }
    #[test]
    fn rebolred () {
        assert_eq!(id("redbol"), Some(151));
    }
    #[test]
    fn renderman_rib () {
        assert_eq!(id("rib"), Some(152));
    }
    #[test]
    fn renderman_rsl () {
        assert_eq!(id("rsl"), Some(153));
    }
    #[test]
    fn riscript () {
        assert_eq!(id("risc"), Some(154));
    }
    #[test]
    fn roboconf () {
        assert_eq!(id("graph"), Some(155));
    }
    #[test]
    fn robot_framework () {
        assert_eq!(id("robot"), Some(156));
    }
    #[test]
    fn rpm_spec_files () {
        assert_eq!(id("rpm-specfile"), Some(157));
    }
    #[test]
    fn ruby () {
        assert_eq!(id("ruby"), Some(158));
    }
    #[test]
    fn rust () {
        assert_eq!(id("rust"), Some(159));
    }
    #[test]
    fn sas () {
        assert_eq!(id("SAS"), Some(160));
    }
    #[test]
    fn scss () {
        assert_eq!(id("scss"), Some(161));
    }
    #[test]
    fn sql () {
        assert_eq!(id("sql"), Some(162));
    }
    #[test]
    fn step_part_2one () {
        assert_eq!(id("p21"), Some(163));
    }
    #[test]
    fn scala () {
        assert_eq!(id("scala"), Some(164));
    }
    #[test]
    fn scheme () {
        assert_eq!(id("scheme"), Some(165));
    }
    #[test]
    fn scilab () {
        assert_eq!(id("scilab"), Some(166));
    }
    #[test]
    fn shape_expressions () {
        assert_eq!(id("shexc"), Some(167));
    }
    #[test]
    fn shell () {
        assert_eq!(id("shell"), Some(168));
    }
    #[test]
    fn smali () {
        assert_eq!(id("smali"), Some(169));
    }
    #[test]
    fn smalltalk () {
        assert_eq!(id("smalltalk"), Some(170));
    }
    #[test]
    fn sml () {
        assert_eq!(id("sml"), Some(171));
    }
    #[test]
    fn solidity () {
        assert_eq!(id("solidity"), Some(172));
    }
    #[test]
    fn stan () {
        assert_eq!(id("stan"), Some(173));
    }
    #[test]
    fn stata () {
        assert_eq!(id("stata"), Some(174));
    }
    #[test]
    fn structured_text () {
        assert_eq!(id("iecst"), Some(175));
    }
    #[test]
    fn stylus () {
        assert_eq!(id("stylus"), Some(176));
    }
    #[test]
    fn subunit () {
        assert_eq!(id("subunit"), Some(177));
    }
    #[test]
    fn supercollider () {
        assert_eq!(id("supercollider"), Some(178));
    }
    #[test]
    fn svelte () {
        assert_eq!(id("svelte"), Some(179));
    }
    #[test]
    fn swift () {
        assert_eq!(id("swift"), Some(180));
    }
    #[test]
    fn tcl () {
        assert_eq!(id("tcl"), Some(181));
    }
    #[test]
    fn terraform () {
        assert_eq!(id("terraform"), Some(182));
    }
    #[test]
    fn test_anything_protocol () {
        assert_eq!(id("tap"), Some(183));
    }
    #[test]
    fn thrift () {
        assert_eq!(id("thrift"), Some(184));
    }
    #[test]
    fn tp () {
        assert_eq!(id("tp"), Some(185));
    }
    #[test]
    fn transactsql () {
        assert_eq!(id("tsql"), Some(186));
    }
    #[test]
    fn twig () {
        assert_eq!(id("twig"), Some(187));
    }
    #[test]
    fn typescript () {
        assert_eq!(id("typescript"), Some(188));
    }
    #[test]
    fn unicorn_rails_log () {
        assert_eq!(id("unicorn-rails-log"), Some(189));
    }
    #[test]
    fn vbnet () {
        assert_eq!(id("vbnet"), Some(190));
    }
    #[test]
    fn vba () {
        assert_eq!(id("vba"), Some(191));
    }
    #[test]
    fn vbscript () {
        assert_eq!(id("vbscript"), Some(192));
    }
    #[test]
    fn vhdl () {
        assert_eq!(id("vhdl"), Some(193));
    }
    #[test]
    fn vala () {
        assert_eq!(id("vala"), Some(194));
    }
    #[test]
    fn verilog () {
        assert_eq!(id("verilog"), Some(195));
    }
    #[test]
    fn vim_script () {
        assert_eq!(id("vim"), Some(196));
    }
    #[test]
    fn xpp () {
        assert_eq!(id("axapta"), Some(197));
    }
    #[test]
    fn x86_assembly () {
        assert_eq!(id("x86asm"), Some(198));
    }
    #[test]
    fn xl () {
        assert_eq!(id("xl"), Some(199));
    }
    #[test]
    fn xquery () {
        assert_eq!(id("xquery"), Some(200));
    }
    #[test]
    fn yaml () {
        assert_eq!(id("yml"), Some(201));
    }
    #[test]
    fn zephir () {
        assert_eq!(id("zephir"), Some(202));
    }
}
