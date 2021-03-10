//codegen.d/00_init_hashmap.awk
extern crate lazy_static;
use std::collections::HashMap;

lazy_static::lazy_static! {
	static ref HLJS: HashMap<&'static str, &'static str> = {
		let mut map = HashMap::with_capacity(205);
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
		map.insert("Blade", "blade");
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
		map.insert("Cypher", "cypher");
		map.insert("D", "d");
		map.insert("DNS Zone file", "dns");
		map.insert("DOS", "dos");
		map.insert("Dart", "dart");
		map.insert("Delphi", "delphi");
		map.insert("Diff", "diff");
		map.insert("Django", "django");
		map.insert("Dockerfile", "dockerfile");
		map.insert("dsconfig", "dsconfig");
		map.insert("DTS", "dts");
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
		map.insert("HTML", "xml");
		map.insert("XML", "xml");
		map.insert("HTTP", "http");
		map.insert("Haml", "haml");
		map.insert("Handlebars", "handlebars");
		map.insert("Haskell", "haskell");
		map.insert("Haxe", "haxe");
		map.insert("High-level shader language", "hlsl");
		map.insert("Hy", "hy");
		map.insert("Ini", "ini");
		map.insert("TOML", "ini");
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
		map.insert("Rebol", "redbol");
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
		map.insert("Terraform", "terraform");
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
//codegen.d/10_from_id_conversions.awk
pub fn from_id(id: usize) -> Option<&'static str> {
	match id {
		0 => Some("1c"),
		1 => Some("4d"),
		2 => Some("abnf"),
		3 => Some("accesslog"),
		4 => Some("ada"),
		5 => Some("arduino"),
		6 => Some("armasm"),
		7 => Some("avrasm"),
		8 => Some("actionscript"),
		9 => Some("alan"),
		10 => Some("ln"),
		11 => Some("angelscript"),
		12 => Some("apache"),
		13 => Some("applescript"),
		14 => Some("arcade"),
		15 => Some("asciidoc"),
		16 => Some("aspectj"),
		17 => Some("autohotkey"),
		18 => Some("autoit"),
		19 => Some("awk"),
		20 => Some("bash"),
		21 => Some("basic"),
		22 => Some("bbcode"),
		23 => Some("blade"),
		24 => Some("bnf"),
		25 => Some("brainfuck"),
		26 => Some("csharp"),
		27 => Some("c"),
		28 => Some("cpp"),
		29 => Some("cal"),
		30 => Some("cos"),
		31 => Some("cmake"),
		32 => Some("coq"),
		33 => Some("csp"),
		34 => Some("css"),
		35 => Some("capnproto"),
		36 => Some("chaos"),
		37 => Some("chapel"),
		38 => Some("cisco"),
		39 => Some("clojure"),
		40 => Some("coffeescript"),
		41 => Some("cpc"),
		42 => Some("crmsh"),
		43 => Some("crystal"),
		44 => Some("cypher"),
		45 => Some("d"),
		46 => Some("dns"),
		47 => Some("dos"),
		48 => Some("dart"),
		49 => Some("delphi"),
		50 => Some("diff"),
		51 => Some("django"),
		52 => Some("dockerfile"),
		53 => Some("dsconfig"),
		54 => Some("dts"),
		55 => Some("dust"),
		56 => Some("dylan"),
		57 => Some("ebnf"),
		58 => Some("elixir"),
		59 => Some("elm"),
		60 => Some("erlang"),
		61 => Some("excel"),
		62 => Some("extempore"),
		63 => Some("fsharp"),
		64 => Some("fix"),
		65 => Some("fortran"),
		66 => Some("gcode"),
		67 => Some("gams"),
		68 => Some("gauss"),
		69 => Some("godot"),
		70 => Some("gherkin"),
		71 => Some("gn"),
		72 => Some("go"),
		73 => Some("gf"),
		74 => Some("golo"),
		75 => Some("gradle"),
		76 => Some("groovy"),
		77 => Some("xml"),
		78 => Some("xml"),
		79 => Some("http"),
		80 => Some("haml"),
		81 => Some("handlebars"),
		82 => Some("haskell"),
		83 => Some("haxe"),
		84 => Some("hlsl"),
		85 => Some("hy"),
		86 => Some("ini"),
		87 => Some("ini"),
		88 => Some("inform7"),
		89 => Some("irpf90"),
		90 => Some("json"),
		91 => Some("java"),
		92 => Some("javascript"),
		93 => Some("jolie"),
		94 => Some("julia"),
		95 => Some("kotlin"),
		96 => Some("tex"),
		97 => Some("leaf"),
		98 => Some("lean"),
		99 => Some("lasso"),
		100 => Some("less"),
		101 => Some("ldif"),
		102 => Some("lisp"),
		103 => Some("livecodeserver"),
		104 => Some("livescript"),
		105 => Some("lua"),
		106 => Some("makefile"),
		107 => Some("markdown"),
		108 => Some("mathematica"),
		109 => Some("matlab"),
		110 => Some("maxima"),
		111 => Some("mel"),
		112 => Some("mercury"),
		113 => Some("mirc"),
		114 => Some("mizar"),
		115 => Some("mojolicious"),
		116 => Some("monkey"),
		117 => Some("moonscript"),
		118 => Some("n1ql"),
		119 => Some("nsis"),
		120 => Some("never"),
		121 => Some("nginx"),
		122 => Some("nim"),
		123 => Some("nix"),
		124 => Some("ocl"),
		125 => Some("ocaml"),
		126 => Some("objectivec"),
		127 => Some("glsl"),
		128 => Some("openscad"),
		129 => Some("ruleslanguage"),
		130 => Some("oxygene"),
		131 => Some("pf"),
		132 => Some("php"),
		133 => Some("parser3"),
		134 => Some("perl"),
		135 => Some("plaintext"),
		136 => Some("pony"),
		137 => Some("pgsql"),
		138 => Some("powershell"),
		139 => Some("processing"),
		140 => Some("prolog"),
		141 => Some("properties"),
		142 => Some("protobuf"),
		143 => Some("puppet"),
		144 => Some("python"),
		145 => Some("profile"),
		146 => Some("python-repl"),
		147 => Some("qsharp"),
		148 => Some("k"),
		149 => Some("qml"),
		150 => Some("r"),
		151 => Some("cshtml"),
		152 => Some("reasonml"),
		153 => Some("redbol"),
		154 => Some("rib"),
		155 => Some("rsl"),
		156 => Some("risc"),
		157 => Some("graph"),
		158 => Some("robot"),
		159 => Some("rpm-specfile"),
		160 => Some("ruby"),
		161 => Some("rust"),
		162 => Some("SAS"),
		163 => Some("scss"),
		164 => Some("sql"),
		165 => Some("p21"),
		166 => Some("scala"),
		167 => Some("scheme"),
		168 => Some("scilab"),
		169 => Some("shexc"),
		170 => Some("shell"),
		171 => Some("smali"),
		172 => Some("smalltalk"),
		173 => Some("sml"),
		174 => Some("solidity"),
		175 => Some("stan"),
		176 => Some("stata"),
		177 => Some("iecst"),
		178 => Some("stylus"),
		179 => Some("subunit"),
		180 => Some("supercollider"),
		181 => Some("svelte"),
		182 => Some("swift"),
		183 => Some("tcl"),
		184 => Some("terraform"),
		185 => Some("tap"),
		186 => Some("thrift"),
		187 => Some("tp"),
		188 => Some("tsql"),
		189 => Some("twig"),
		190 => Some("typescript"),
		191 => Some("unicorn-rails-log"),
		192 => Some("vbnet"),
		193 => Some("vba"),
		194 => Some("vbscript"),
		195 => Some("vhdl"),
		196 => Some("vala"),
		197 => Some("verilog"),
		198 => Some("vim"),
		199 => Some("axapta"),
		200 => Some("x86asm"),
		201 => Some("xl"),
		202 => Some("xquery"),
		203 => Some("yml"),
		204 => Some("zephir"),
		_ => None
	}
}
//codegen.d/11_to_id_conversions.awk
pub fn to_id(code: usize) -> Option<usize> {
	match code {
		"1c" => Some(0),
		"4d" => Some(1),
		"abnf" => Some(2),
		"accesslog" => Some(3),
		"ada" => Some(4),
		"arduino" => Some(5),
		"armasm" => Some(6),
		"avrasm" => Some(7),
		"actionscript" => Some(8),
		"alan" => Some(9),
		"ln" => Some(10),
		"angelscript" => Some(11),
		"apache" => Some(12),
		"applescript" => Some(13),
		"arcade" => Some(14),
		"asciidoc" => Some(15),
		"aspectj" => Some(16),
		"autohotkey" => Some(17),
		"autoit" => Some(18),
		"awk" => Some(19),
		"bash" => Some(20),
		"basic" => Some(21),
		"bbcode" => Some(22),
		"blade" => Some(23),
		"bnf" => Some(24),
		"brainfuck" => Some(25),
		"csharp" => Some(26),
		"c" => Some(27),
		"cpp" => Some(28),
		"cal" => Some(29),
		"cos" => Some(30),
		"cmake" => Some(31),
		"coq" => Some(32),
		"csp" => Some(33),
		"css" => Some(34),
		"capnproto" => Some(35),
		"chaos" => Some(36),
		"chapel" => Some(37),
		"cisco" => Some(38),
		"clojure" => Some(39),
		"coffeescript" => Some(40),
		"cpc" => Some(41),
		"crmsh" => Some(42),
		"crystal" => Some(43),
		"cypher" => Some(44),
		"d" => Some(45),
		"dns" => Some(46),
		"dos" => Some(47),
		"dart" => Some(48),
		"delphi" => Some(49),
		"diff" => Some(50),
		"django" => Some(51),
		"dockerfile" => Some(52),
		"dsconfig" => Some(53),
		"dts" => Some(54),
		"dust" => Some(55),
		"dylan" => Some(56),
		"ebnf" => Some(57),
		"elixir" => Some(58),
		"elm" => Some(59),
		"erlang" => Some(60),
		"excel" => Some(61),
		"extempore" => Some(62),
		"fsharp" => Some(63),
		"fix" => Some(64),
		"fortran" => Some(65),
		"gcode" => Some(66),
		"gams" => Some(67),
		"gauss" => Some(68),
		"godot" => Some(69),
		"gherkin" => Some(70),
		"gn" => Some(71),
		"go" => Some(72),
		"gf" => Some(73),
		"golo" => Some(74),
		"gradle" => Some(75),
		"groovy" => Some(76),
		"xml" => Some(77),
		"xml" => Some(78),
		"http" => Some(79),
		"haml" => Some(80),
		"handlebars" => Some(81),
		"haskell" => Some(82),
		"haxe" => Some(83),
		"hlsl" => Some(84),
		"hy" => Some(85),
		"ini" => Some(86),
		"ini" => Some(87),
		"inform7" => Some(88),
		"irpf90" => Some(89),
		"json" => Some(90),
		"java" => Some(91),
		"javascript" => Some(92),
		"jolie" => Some(93),
		"julia" => Some(94),
		"kotlin" => Some(95),
		"tex" => Some(96),
		"leaf" => Some(97),
		"lean" => Some(98),
		"lasso" => Some(99),
		"less" => Some(100),
		"ldif" => Some(101),
		"lisp" => Some(102),
		"livecodeserver" => Some(103),
		"livescript" => Some(104),
		"lua" => Some(105),
		"makefile" => Some(106),
		"markdown" => Some(107),
		"mathematica" => Some(108),
		"matlab" => Some(109),
		"maxima" => Some(110),
		"mel" => Some(111),
		"mercury" => Some(112),
		"mirc" => Some(113),
		"mizar" => Some(114),
		"mojolicious" => Some(115),
		"monkey" => Some(116),
		"moonscript" => Some(117),
		"n1ql" => Some(118),
		"nsis" => Some(119),
		"never" => Some(120),
		"nginx" => Some(121),
		"nim" => Some(122),
		"nix" => Some(123),
		"ocl" => Some(124),
		"ocaml" => Some(125),
		"objectivec" => Some(126),
		"glsl" => Some(127),
		"openscad" => Some(128),
		"ruleslanguage" => Some(129),
		"oxygene" => Some(130),
		"pf" => Some(131),
		"php" => Some(132),
		"parser3" => Some(133),
		"perl" => Some(134),
		"plaintext" => Some(135),
		"pony" => Some(136),
		"pgsql" => Some(137),
		"powershell" => Some(138),
		"processing" => Some(139),
		"prolog" => Some(140),
		"properties" => Some(141),
		"protobuf" => Some(142),
		"puppet" => Some(143),
		"python" => Some(144),
		"profile" => Some(145),
		"python-repl" => Some(146),
		"qsharp" => Some(147),
		"k" => Some(148),
		"qml" => Some(149),
		"r" => Some(150),
		"cshtml" => Some(151),
		"reasonml" => Some(152),
		"redbol" => Some(153),
		"rib" => Some(154),
		"rsl" => Some(155),
		"risc" => Some(156),
		"graph" => Some(157),
		"robot" => Some(158),
		"rpm-specfile" => Some(159),
		"ruby" => Some(160),
		"rust" => Some(161),
		"SAS" => Some(162),
		"scss" => Some(163),
		"sql" => Some(164),
		"p21" => Some(165),
		"scala" => Some(166),
		"scheme" => Some(167),
		"scilab" => Some(168),
		"shexc" => Some(169),
		"shell" => Some(170),
		"smali" => Some(171),
		"smalltalk" => Some(172),
		"sml" => Some(173),
		"solidity" => Some(174),
		"stan" => Some(175),
		"stata" => Some(176),
		"iecst" => Some(177),
		"stylus" => Some(178),
		"subunit" => Some(179),
		"supercollider" => Some(180),
		"svelte" => Some(181),
		"swift" => Some(182),
		"tcl" => Some(183),
		"terraform" => Some(184),
		"tap" => Some(185),
		"thrift" => Some(186),
		"tp" => Some(187),
		"tsql" => Some(188),
		"twig" => Some(189),
		"typescript" => Some(190),
		"unicorn-rails-log" => Some(191),
		"vbnet" => Some(192),
		"vba" => Some(193),
		"vbscript" => Some(194),
		"vhdl" => Some(195),
		"vala" => Some(196),
		"verilog" => Some(197),
		"vim" => Some(198),
		"axapta" => Some(199),
		"x86asm" => Some(200),
		"xl" => Some(201),
		"xquery" => Some(202),
		"yml" => Some(203),
		"zephir" => Some(204),
		_ => None
	}
}
//codegen.d/20_match_functions.awk
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

//codegen.d/30_regular_tests.awk
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
//codegen.d/40_from_id_tests.awk
#[cfg(test)]
mod from_id {
	use super::from_id;
	#[test]
	fn 1c() {
		assert_eq!(from_id(0), Some("1c"));
	}
	#[test]
	fn 4d() {
		assert_eq!(from_id(1), Some("4d"));
	}
	#[test]
	fn abnf() {
		assert_eq!(from_id(2), Some("abnf"));
	}
	#[test]
	fn accesslog() {
		assert_eq!(from_id(3), Some("accesslog"));
	}
	#[test]
	fn ada() {
		assert_eq!(from_id(4), Some("ada"));
	}
	#[test]
	fn arduino() {
		assert_eq!(from_id(5), Some("arduino"));
	}
	#[test]
	fn armasm() {
		assert_eq!(from_id(6), Some("armasm"));
	}
	#[test]
	fn avrasm() {
		assert_eq!(from_id(7), Some("avrasm"));
	}
	#[test]
	fn actionscript() {
		assert_eq!(from_id(8), Some("actionscript"));
	}
	#[test]
	fn alan() {
		assert_eq!(from_id(9), Some("alan"));
	}
	#[test]
	fn ln() {
		assert_eq!(from_id(10), Some("ln"));
	}
	#[test]
	fn angelscript() {
		assert_eq!(from_id(11), Some("angelscript"));
	}
	#[test]
	fn apache() {
		assert_eq!(from_id(12), Some("apache"));
	}
	#[test]
	fn applescript() {
		assert_eq!(from_id(13), Some("applescript"));
	}
	#[test]
	fn arcade() {
		assert_eq!(from_id(14), Some("arcade"));
	}
	#[test]
	fn asciidoc() {
		assert_eq!(from_id(15), Some("asciidoc"));
	}
	#[test]
	fn aspectj() {
		assert_eq!(from_id(16), Some("aspectj"));
	}
	#[test]
	fn autohotkey() {
		assert_eq!(from_id(17), Some("autohotkey"));
	}
	#[test]
	fn autoit() {
		assert_eq!(from_id(18), Some("autoit"));
	}
	#[test]
	fn awk() {
		assert_eq!(from_id(19), Some("awk"));
	}
	#[test]
	fn bash() {
		assert_eq!(from_id(20), Some("bash"));
	}
	#[test]
	fn basic() {
		assert_eq!(from_id(21), Some("basic"));
	}
	#[test]
	fn bbcode() {
		assert_eq!(from_id(22), Some("bbcode"));
	}
	#[test]
	fn blade() {
		assert_eq!(from_id(23), Some("blade"));
	}
	#[test]
	fn bnf() {
		assert_eq!(from_id(24), Some("bnf"));
	}
	#[test]
	fn brainfuck() {
		assert_eq!(from_id(25), Some("brainfuck"));
	}
	#[test]
	fn csharp() {
		assert_eq!(from_id(26), Some("csharp"));
	}
	#[test]
	fn c() {
		assert_eq!(from_id(27), Some("c"));
	}
	#[test]
	fn cpp() {
		assert_eq!(from_id(28), Some("cpp"));
	}
	#[test]
	fn cal() {
		assert_eq!(from_id(29), Some("cal"));
	}
	#[test]
	fn cos() {
		assert_eq!(from_id(30), Some("cos"));
	}
	#[test]
	fn cmake() {
		assert_eq!(from_id(31), Some("cmake"));
	}
	#[test]
	fn coq() {
		assert_eq!(from_id(32), Some("coq"));
	}
	#[test]
	fn csp() {
		assert_eq!(from_id(33), Some("csp"));
	}
	#[test]
	fn css() {
		assert_eq!(from_id(34), Some("css"));
	}
	#[test]
	fn capnproto() {
		assert_eq!(from_id(35), Some("capnproto"));
	}
	#[test]
	fn chaos() {
		assert_eq!(from_id(36), Some("chaos"));
	}
	#[test]
	fn chapel() {
		assert_eq!(from_id(37), Some("chapel"));
	}
	#[test]
	fn cisco() {
		assert_eq!(from_id(38), Some("cisco"));
	}
	#[test]
	fn clojure() {
		assert_eq!(from_id(39), Some("clojure"));
	}
	#[test]
	fn coffeescript() {
		assert_eq!(from_id(40), Some("coffeescript"));
	}
	#[test]
	fn cpc() {
		assert_eq!(from_id(41), Some("cpc"));
	}
	#[test]
	fn crmsh() {
		assert_eq!(from_id(42), Some("crmsh"));
	}
	#[test]
	fn crystal() {
		assert_eq!(from_id(43), Some("crystal"));
	}
	#[test]
	fn cypher() {
		assert_eq!(from_id(44), Some("cypher"));
	}
	#[test]
	fn d() {
		assert_eq!(from_id(45), Some("d"));
	}
	#[test]
	fn dns() {
		assert_eq!(from_id(46), Some("dns"));
	}
	#[test]
	fn dos() {
		assert_eq!(from_id(47), Some("dos"));
	}
	#[test]
	fn dart() {
		assert_eq!(from_id(48), Some("dart"));
	}
	#[test]
	fn delphi() {
		assert_eq!(from_id(49), Some("delphi"));
	}
	#[test]
	fn diff() {
		assert_eq!(from_id(50), Some("diff"));
	}
	#[test]
	fn django() {
		assert_eq!(from_id(51), Some("django"));
	}
	#[test]
	fn dockerfile() {
		assert_eq!(from_id(52), Some("dockerfile"));
	}
	#[test]
	fn dsconfig() {
		assert_eq!(from_id(53), Some("dsconfig"));
	}
	#[test]
	fn dts() {
		assert_eq!(from_id(54), Some("dts"));
	}
	#[test]
	fn dust() {
		assert_eq!(from_id(55), Some("dust"));
	}
	#[test]
	fn dylan() {
		assert_eq!(from_id(56), Some("dylan"));
	}
	#[test]
	fn ebnf() {
		assert_eq!(from_id(57), Some("ebnf"));
	}
	#[test]
	fn elixir() {
		assert_eq!(from_id(58), Some("elixir"));
	}
	#[test]
	fn elm() {
		assert_eq!(from_id(59), Some("elm"));
	}
	#[test]
	fn erlang() {
		assert_eq!(from_id(60), Some("erlang"));
	}
	#[test]
	fn excel() {
		assert_eq!(from_id(61), Some("excel"));
	}
	#[test]
	fn extempore() {
		assert_eq!(from_id(62), Some("extempore"));
	}
	#[test]
	fn fsharp() {
		assert_eq!(from_id(63), Some("fsharp"));
	}
	#[test]
	fn fix() {
		assert_eq!(from_id(64), Some("fix"));
	}
	#[test]
	fn fortran() {
		assert_eq!(from_id(65), Some("fortran"));
	}
	#[test]
	fn gcode() {
		assert_eq!(from_id(66), Some("gcode"));
	}
	#[test]
	fn gams() {
		assert_eq!(from_id(67), Some("gams"));
	}
	#[test]
	fn gauss() {
		assert_eq!(from_id(68), Some("gauss"));
	}
	#[test]
	fn godot() {
		assert_eq!(from_id(69), Some("godot"));
	}
	#[test]
	fn gherkin() {
		assert_eq!(from_id(70), Some("gherkin"));
	}
	#[test]
	fn gn() {
		assert_eq!(from_id(71), Some("gn"));
	}
	#[test]
	fn go() {
		assert_eq!(from_id(72), Some("go"));
	}
	#[test]
	fn gf() {
		assert_eq!(from_id(73), Some("gf"));
	}
	#[test]
	fn golo() {
		assert_eq!(from_id(74), Some("golo"));
	}
	#[test]
	fn gradle() {
		assert_eq!(from_id(75), Some("gradle"));
	}
	#[test]
	fn groovy() {
		assert_eq!(from_id(76), Some("groovy"));
	}
	#[test]
	fn xml() {
		assert_eq!(from_id(77), Some("xml"));
	}
	#[test]
	fn xml() {
		assert_eq!(from_id(78), Some("xml"));
	}
	#[test]
	fn http() {
		assert_eq!(from_id(79), Some("http"));
	}
	#[test]
	fn haml() {
		assert_eq!(from_id(80), Some("haml"));
	}
	#[test]
	fn handlebars() {
		assert_eq!(from_id(81), Some("handlebars"));
	}
	#[test]
	fn haskell() {
		assert_eq!(from_id(82), Some("haskell"));
	}
	#[test]
	fn haxe() {
		assert_eq!(from_id(83), Some("haxe"));
	}
	#[test]
	fn hlsl() {
		assert_eq!(from_id(84), Some("hlsl"));
	}
	#[test]
	fn hy() {
		assert_eq!(from_id(85), Some("hy"));
	}
	#[test]
	fn ini() {
		assert_eq!(from_id(86), Some("ini"));
	}
	#[test]
	fn ini() {
		assert_eq!(from_id(87), Some("ini"));
	}
	#[test]
	fn inform7() {
		assert_eq!(from_id(88), Some("inform7"));
	}
	#[test]
	fn irpf90() {
		assert_eq!(from_id(89), Some("irpf90"));
	}
	#[test]
	fn json() {
		assert_eq!(from_id(90), Some("json"));
	}
	#[test]
	fn java() {
		assert_eq!(from_id(91), Some("java"));
	}
	#[test]
	fn javascript() {
		assert_eq!(from_id(92), Some("javascript"));
	}
	#[test]
	fn jolie() {
		assert_eq!(from_id(93), Some("jolie"));
	}
	#[test]
	fn julia() {
		assert_eq!(from_id(94), Some("julia"));
	}
	#[test]
	fn kotlin() {
		assert_eq!(from_id(95), Some("kotlin"));
	}
	#[test]
	fn tex() {
		assert_eq!(from_id(96), Some("tex"));
	}
	#[test]
	fn leaf() {
		assert_eq!(from_id(97), Some("leaf"));
	}
	#[test]
	fn lean() {
		assert_eq!(from_id(98), Some("lean"));
	}
	#[test]
	fn lasso() {
		assert_eq!(from_id(99), Some("lasso"));
	}
	#[test]
	fn less() {
		assert_eq!(from_id(100), Some("less"));
	}
	#[test]
	fn ldif() {
		assert_eq!(from_id(101), Some("ldif"));
	}
	#[test]
	fn lisp() {
		assert_eq!(from_id(102), Some("lisp"));
	}
	#[test]
	fn livecodeserver() {
		assert_eq!(from_id(103), Some("livecodeserver"));
	}
	#[test]
	fn livescript() {
		assert_eq!(from_id(104), Some("livescript"));
	}
	#[test]
	fn lua() {
		assert_eq!(from_id(105), Some("lua"));
	}
	#[test]
	fn makefile() {
		assert_eq!(from_id(106), Some("makefile"));
	}
	#[test]
	fn markdown() {
		assert_eq!(from_id(107), Some("markdown"));
	}
	#[test]
	fn mathematica() {
		assert_eq!(from_id(108), Some("mathematica"));
	}
	#[test]
	fn matlab() {
		assert_eq!(from_id(109), Some("matlab"));
	}
	#[test]
	fn maxima() {
		assert_eq!(from_id(110), Some("maxima"));
	}
	#[test]
	fn mel() {
		assert_eq!(from_id(111), Some("mel"));
	}
	#[test]
	fn mercury() {
		assert_eq!(from_id(112), Some("mercury"));
	}
	#[test]
	fn mirc() {
		assert_eq!(from_id(113), Some("mirc"));
	}
	#[test]
	fn mizar() {
		assert_eq!(from_id(114), Some("mizar"));
	}
	#[test]
	fn mojolicious() {
		assert_eq!(from_id(115), Some("mojolicious"));
	}
	#[test]
	fn monkey() {
		assert_eq!(from_id(116), Some("monkey"));
	}
	#[test]
	fn moonscript() {
		assert_eq!(from_id(117), Some("moonscript"));
	}
	#[test]
	fn n1ql() {
		assert_eq!(from_id(118), Some("n1ql"));
	}
	#[test]
	fn nsis() {
		assert_eq!(from_id(119), Some("nsis"));
	}
	#[test]
	fn never() {
		assert_eq!(from_id(120), Some("never"));
	}
	#[test]
	fn nginx() {
		assert_eq!(from_id(121), Some("nginx"));
	}
	#[test]
	fn nim() {
		assert_eq!(from_id(122), Some("nim"));
	}
	#[test]
	fn nix() {
		assert_eq!(from_id(123), Some("nix"));
	}
	#[test]
	fn ocl() {
		assert_eq!(from_id(124), Some("ocl"));
	}
	#[test]
	fn ocaml() {
		assert_eq!(from_id(125), Some("ocaml"));
	}
	#[test]
	fn objectivec() {
		assert_eq!(from_id(126), Some("objectivec"));
	}
	#[test]
	fn glsl() {
		assert_eq!(from_id(127), Some("glsl"));
	}
	#[test]
	fn openscad() {
		assert_eq!(from_id(128), Some("openscad"));
	}
	#[test]
	fn ruleslanguage() {
		assert_eq!(from_id(129), Some("ruleslanguage"));
	}
	#[test]
	fn oxygene() {
		assert_eq!(from_id(130), Some("oxygene"));
	}
	#[test]
	fn pf() {
		assert_eq!(from_id(131), Some("pf"));
	}
	#[test]
	fn php() {
		assert_eq!(from_id(132), Some("php"));
	}
	#[test]
	fn parser3() {
		assert_eq!(from_id(133), Some("parser3"));
	}
	#[test]
	fn perl() {
		assert_eq!(from_id(134), Some("perl"));
	}
	#[test]
	fn plaintext() {
		assert_eq!(from_id(135), Some("plaintext"));
	}
	#[test]
	fn pony() {
		assert_eq!(from_id(136), Some("pony"));
	}
	#[test]
	fn pgsql() {
		assert_eq!(from_id(137), Some("pgsql"));
	}
	#[test]
	fn powershell() {
		assert_eq!(from_id(138), Some("powershell"));
	}
	#[test]
	fn processing() {
		assert_eq!(from_id(139), Some("processing"));
	}
	#[test]
	fn prolog() {
		assert_eq!(from_id(140), Some("prolog"));
	}
	#[test]
	fn properties() {
		assert_eq!(from_id(141), Some("properties"));
	}
	#[test]
	fn protobuf() {
		assert_eq!(from_id(142), Some("protobuf"));
	}
	#[test]
	fn puppet() {
		assert_eq!(from_id(143), Some("puppet"));
	}
	#[test]
	fn python() {
		assert_eq!(from_id(144), Some("python"));
	}
	#[test]
	fn profile() {
		assert_eq!(from_id(145), Some("profile"));
	}
	#[test]
	fn python-repl() {
		assert_eq!(from_id(146), Some("python-repl"));
	}
	#[test]
	fn qsharp() {
		assert_eq!(from_id(147), Some("qsharp"));
	}
	#[test]
	fn k() {
		assert_eq!(from_id(148), Some("k"));
	}
	#[test]
	fn qml() {
		assert_eq!(from_id(149), Some("qml"));
	}
	#[test]
	fn r() {
		assert_eq!(from_id(150), Some("r"));
	}
	#[test]
	fn cshtml() {
		assert_eq!(from_id(151), Some("cshtml"));
	}
	#[test]
	fn reasonml() {
		assert_eq!(from_id(152), Some("reasonml"));
	}
	#[test]
	fn redbol() {
		assert_eq!(from_id(153), Some("redbol"));
	}
	#[test]
	fn rib() {
		assert_eq!(from_id(154), Some("rib"));
	}
	#[test]
	fn rsl() {
		assert_eq!(from_id(155), Some("rsl"));
	}
	#[test]
	fn risc() {
		assert_eq!(from_id(156), Some("risc"));
	}
	#[test]
	fn graph() {
		assert_eq!(from_id(157), Some("graph"));
	}
	#[test]
	fn robot() {
		assert_eq!(from_id(158), Some("robot"));
	}
	#[test]
	fn rpm-specfile() {
		assert_eq!(from_id(159), Some("rpm-specfile"));
	}
	#[test]
	fn ruby() {
		assert_eq!(from_id(160), Some("ruby"));
	}
	#[test]
	fn rust() {
		assert_eq!(from_id(161), Some("rust"));
	}
	#[test]
	fn SAS() {
		assert_eq!(from_id(162), Some("SAS"));
	}
	#[test]
	fn scss() {
		assert_eq!(from_id(163), Some("scss"));
	}
	#[test]
	fn sql() {
		assert_eq!(from_id(164), Some("sql"));
	}
	#[test]
	fn p21() {
		assert_eq!(from_id(165), Some("p21"));
	}
	#[test]
	fn scala() {
		assert_eq!(from_id(166), Some("scala"));
	}
	#[test]
	fn scheme() {
		assert_eq!(from_id(167), Some("scheme"));
	}
	#[test]
	fn scilab() {
		assert_eq!(from_id(168), Some("scilab"));
	}
	#[test]
	fn shexc() {
		assert_eq!(from_id(169), Some("shexc"));
	}
	#[test]
	fn shell() {
		assert_eq!(from_id(170), Some("shell"));
	}
	#[test]
	fn smali() {
		assert_eq!(from_id(171), Some("smali"));
	}
	#[test]
	fn smalltalk() {
		assert_eq!(from_id(172), Some("smalltalk"));
	}
	#[test]
	fn sml() {
		assert_eq!(from_id(173), Some("sml"));
	}
	#[test]
	fn solidity() {
		assert_eq!(from_id(174), Some("solidity"));
	}
	#[test]
	fn stan() {
		assert_eq!(from_id(175), Some("stan"));
	}
	#[test]
	fn stata() {
		assert_eq!(from_id(176), Some("stata"));
	}
	#[test]
	fn iecst() {
		assert_eq!(from_id(177), Some("iecst"));
	}
	#[test]
	fn stylus() {
		assert_eq!(from_id(178), Some("stylus"));
	}
	#[test]
	fn subunit() {
		assert_eq!(from_id(179), Some("subunit"));
	}
	#[test]
	fn supercollider() {
		assert_eq!(from_id(180), Some("supercollider"));
	}
	#[test]
	fn svelte() {
		assert_eq!(from_id(181), Some("svelte"));
	}
	#[test]
	fn swift() {
		assert_eq!(from_id(182), Some("swift"));
	}
	#[test]
	fn tcl() {
		assert_eq!(from_id(183), Some("tcl"));
	}
	#[test]
	fn terraform() {
		assert_eq!(from_id(184), Some("terraform"));
	}
	#[test]
	fn tap() {
		assert_eq!(from_id(185), Some("tap"));
	}
	#[test]
	fn thrift() {
		assert_eq!(from_id(186), Some("thrift"));
	}
	#[test]
	fn tp() {
		assert_eq!(from_id(187), Some("tp"));
	}
	#[test]
	fn tsql() {
		assert_eq!(from_id(188), Some("tsql"));
	}
	#[test]
	fn twig() {
		assert_eq!(from_id(189), Some("twig"));
	}
	#[test]
	fn typescript() {
		assert_eq!(from_id(190), Some("typescript"));
	}
	#[test]
	fn unicorn-rails-log() {
		assert_eq!(from_id(191), Some("unicorn-rails-log"));
	}
	#[test]
	fn vbnet() {
		assert_eq!(from_id(192), Some("vbnet"));
	}
	#[test]
	fn vba() {
		assert_eq!(from_id(193), Some("vba"));
	}
	#[test]
	fn vbscript() {
		assert_eq!(from_id(194), Some("vbscript"));
	}
	#[test]
	fn vhdl() {
		assert_eq!(from_id(195), Some("vhdl"));
	}
	#[test]
	fn vala() {
		assert_eq!(from_id(196), Some("vala"));
	}
	#[test]
	fn verilog() {
		assert_eq!(from_id(197), Some("verilog"));
	}
	#[test]
	fn vim() {
		assert_eq!(from_id(198), Some("vim"));
	}
	#[test]
	fn axapta() {
		assert_eq!(from_id(199), Some("axapta"));
	}
	#[test]
	fn x86asm() {
		assert_eq!(from_id(200), Some("x86asm"));
	}
	#[test]
	fn xl() {
		assert_eq!(from_id(201), Some("xl"));
	}
	#[test]
	fn xquery() {
		assert_eq!(from_id(202), Some("xquery"));
	}
	#[test]
	fn yml() {
		assert_eq!(from_id(203), Some("yml"));
	}
	#[test]
	fn zephir() {
		assert_eq!(from_id(204), Some("zephir"));
	}
}
//codegen.d/41_to_id_tests.awk
#[cfg(test)]
mod to_id {
	use super::to_id;
	#[test]
	fn 1c() {
		assert_eq!(to_id("1c"), Some(0));
	}
	#[test]
	fn 4d() {
		assert_eq!(to_id("4d"), Some(1));
	}
	#[test]
	fn abnf() {
		assert_eq!(to_id("abnf"), Some(2));
	}
	#[test]
	fn accesslog() {
		assert_eq!(to_id("accesslog"), Some(3));
	}
	#[test]
	fn ada() {
		assert_eq!(to_id("ada"), Some(4));
	}
	#[test]
	fn arduino() {
		assert_eq!(to_id("arduino"), Some(5));
	}
	#[test]
	fn armasm() {
		assert_eq!(to_id("armasm"), Some(6));
	}
	#[test]
	fn avrasm() {
		assert_eq!(to_id("avrasm"), Some(7));
	}
	#[test]
	fn actionscript() {
		assert_eq!(to_id("actionscript"), Some(8));
	}
	#[test]
	fn alan() {
		assert_eq!(to_id("alan"), Some(9));
	}
	#[test]
	fn ln() {
		assert_eq!(to_id("ln"), Some(10));
	}
	#[test]
	fn angelscript() {
		assert_eq!(to_id("angelscript"), Some(11));
	}
	#[test]
	fn apache() {
		assert_eq!(to_id("apache"), Some(12));
	}
	#[test]
	fn applescript() {
		assert_eq!(to_id("applescript"), Some(13));
	}
	#[test]
	fn arcade() {
		assert_eq!(to_id("arcade"), Some(14));
	}
	#[test]
	fn asciidoc() {
		assert_eq!(to_id("asciidoc"), Some(15));
	}
	#[test]
	fn aspectj() {
		assert_eq!(to_id("aspectj"), Some(16));
	}
	#[test]
	fn autohotkey() {
		assert_eq!(to_id("autohotkey"), Some(17));
	}
	#[test]
	fn autoit() {
		assert_eq!(to_id("autoit"), Some(18));
	}
	#[test]
	fn awk() {
		assert_eq!(to_id("awk"), Some(19));
	}
	#[test]
	fn bash() {
		assert_eq!(to_id("bash"), Some(20));
	}
	#[test]
	fn basic() {
		assert_eq!(to_id("basic"), Some(21));
	}
	#[test]
	fn bbcode() {
		assert_eq!(to_id("bbcode"), Some(22));
	}
	#[test]
	fn blade() {
		assert_eq!(to_id("blade"), Some(23));
	}
	#[test]
	fn bnf() {
		assert_eq!(to_id("bnf"), Some(24));
	}
	#[test]
	fn brainfuck() {
		assert_eq!(to_id("brainfuck"), Some(25));
	}
	#[test]
	fn csharp() {
		assert_eq!(to_id("csharp"), Some(26));
	}
	#[test]
	fn c() {
		assert_eq!(to_id("c"), Some(27));
	}
	#[test]
	fn cpp() {
		assert_eq!(to_id("cpp"), Some(28));
	}
	#[test]
	fn cal() {
		assert_eq!(to_id("cal"), Some(29));
	}
	#[test]
	fn cos() {
		assert_eq!(to_id("cos"), Some(30));
	}
	#[test]
	fn cmake() {
		assert_eq!(to_id("cmake"), Some(31));
	}
	#[test]
	fn coq() {
		assert_eq!(to_id("coq"), Some(32));
	}
	#[test]
	fn csp() {
		assert_eq!(to_id("csp"), Some(33));
	}
	#[test]
	fn css() {
		assert_eq!(to_id("css"), Some(34));
	}
	#[test]
	fn capnproto() {
		assert_eq!(to_id("capnproto"), Some(35));
	}
	#[test]
	fn chaos() {
		assert_eq!(to_id("chaos"), Some(36));
	}
	#[test]
	fn chapel() {
		assert_eq!(to_id("chapel"), Some(37));
	}
	#[test]
	fn cisco() {
		assert_eq!(to_id("cisco"), Some(38));
	}
	#[test]
	fn clojure() {
		assert_eq!(to_id("clojure"), Some(39));
	}
	#[test]
	fn coffeescript() {
		assert_eq!(to_id("coffeescript"), Some(40));
	}
	#[test]
	fn cpc() {
		assert_eq!(to_id("cpc"), Some(41));
	}
	#[test]
	fn crmsh() {
		assert_eq!(to_id("crmsh"), Some(42));
	}
	#[test]
	fn crystal() {
		assert_eq!(to_id("crystal"), Some(43));
	}
	#[test]
	fn cypher() {
		assert_eq!(to_id("cypher"), Some(44));
	}
	#[test]
	fn d() {
		assert_eq!(to_id("d"), Some(45));
	}
	#[test]
	fn dns() {
		assert_eq!(to_id("dns"), Some(46));
	}
	#[test]
	fn dos() {
		assert_eq!(to_id("dos"), Some(47));
	}
	#[test]
	fn dart() {
		assert_eq!(to_id("dart"), Some(48));
	}
	#[test]
	fn delphi() {
		assert_eq!(to_id("delphi"), Some(49));
	}
	#[test]
	fn diff() {
		assert_eq!(to_id("diff"), Some(50));
	}
	#[test]
	fn django() {
		assert_eq!(to_id("django"), Some(51));
	}
	#[test]
	fn dockerfile() {
		assert_eq!(to_id("dockerfile"), Some(52));
	}
	#[test]
	fn dsconfig() {
		assert_eq!(to_id("dsconfig"), Some(53));
	}
	#[test]
	fn dts() {
		assert_eq!(to_id("dts"), Some(54));
	}
	#[test]
	fn dust() {
		assert_eq!(to_id("dust"), Some(55));
	}
	#[test]
	fn dylan() {
		assert_eq!(to_id("dylan"), Some(56));
	}
	#[test]
	fn ebnf() {
		assert_eq!(to_id("ebnf"), Some(57));
	}
	#[test]
	fn elixir() {
		assert_eq!(to_id("elixir"), Some(58));
	}
	#[test]
	fn elm() {
		assert_eq!(to_id("elm"), Some(59));
	}
	#[test]
	fn erlang() {
		assert_eq!(to_id("erlang"), Some(60));
	}
	#[test]
	fn excel() {
		assert_eq!(to_id("excel"), Some(61));
	}
	#[test]
	fn extempore() {
		assert_eq!(to_id("extempore"), Some(62));
	}
	#[test]
	fn fsharp() {
		assert_eq!(to_id("fsharp"), Some(63));
	}
	#[test]
	fn fix() {
		assert_eq!(to_id("fix"), Some(64));
	}
	#[test]
	fn fortran() {
		assert_eq!(to_id("fortran"), Some(65));
	}
	#[test]
	fn gcode() {
		assert_eq!(to_id("gcode"), Some(66));
	}
	#[test]
	fn gams() {
		assert_eq!(to_id("gams"), Some(67));
	}
	#[test]
	fn gauss() {
		assert_eq!(to_id("gauss"), Some(68));
	}
	#[test]
	fn godot() {
		assert_eq!(to_id("godot"), Some(69));
	}
	#[test]
	fn gherkin() {
		assert_eq!(to_id("gherkin"), Some(70));
	}
	#[test]
	fn gn() {
		assert_eq!(to_id("gn"), Some(71));
	}
	#[test]
	fn go() {
		assert_eq!(to_id("go"), Some(72));
	}
	#[test]
	fn gf() {
		assert_eq!(to_id("gf"), Some(73));
	}
	#[test]
	fn golo() {
		assert_eq!(to_id("golo"), Some(74));
	}
	#[test]
	fn gradle() {
		assert_eq!(to_id("gradle"), Some(75));
	}
	#[test]
	fn groovy() {
		assert_eq!(to_id("groovy"), Some(76));
	}
	#[test]
	fn xml() {
		assert_eq!(to_id("xml"), Some(77));
	}
	#[test]
	fn xml() {
		assert_eq!(to_id("xml"), Some(78));
	}
	#[test]
	fn http() {
		assert_eq!(to_id("http"), Some(79));
	}
	#[test]
	fn haml() {
		assert_eq!(to_id("haml"), Some(80));
	}
	#[test]
	fn handlebars() {
		assert_eq!(to_id("handlebars"), Some(81));
	}
	#[test]
	fn haskell() {
		assert_eq!(to_id("haskell"), Some(82));
	}
	#[test]
	fn haxe() {
		assert_eq!(to_id("haxe"), Some(83));
	}
	#[test]
	fn hlsl() {
		assert_eq!(to_id("hlsl"), Some(84));
	}
	#[test]
	fn hy() {
		assert_eq!(to_id("hy"), Some(85));
	}
	#[test]
	fn ini() {
		assert_eq!(to_id("ini"), Some(86));
	}
	#[test]
	fn ini() {
		assert_eq!(to_id("ini"), Some(87));
	}
	#[test]
	fn inform7() {
		assert_eq!(to_id("inform7"), Some(88));
	}
	#[test]
	fn irpf90() {
		assert_eq!(to_id("irpf90"), Some(89));
	}
	#[test]
	fn json() {
		assert_eq!(to_id("json"), Some(90));
	}
	#[test]
	fn java() {
		assert_eq!(to_id("java"), Some(91));
	}
	#[test]
	fn javascript() {
		assert_eq!(to_id("javascript"), Some(92));
	}
	#[test]
	fn jolie() {
		assert_eq!(to_id("jolie"), Some(93));
	}
	#[test]
	fn julia() {
		assert_eq!(to_id("julia"), Some(94));
	}
	#[test]
	fn kotlin() {
		assert_eq!(to_id("kotlin"), Some(95));
	}
	#[test]
	fn tex() {
		assert_eq!(to_id("tex"), Some(96));
	}
	#[test]
	fn leaf() {
		assert_eq!(to_id("leaf"), Some(97));
	}
	#[test]
	fn lean() {
		assert_eq!(to_id("lean"), Some(98));
	}
	#[test]
	fn lasso() {
		assert_eq!(to_id("lasso"), Some(99));
	}
	#[test]
	fn less() {
		assert_eq!(to_id("less"), Some(100));
	}
	#[test]
	fn ldif() {
		assert_eq!(to_id("ldif"), Some(101));
	}
	#[test]
	fn lisp() {
		assert_eq!(to_id("lisp"), Some(102));
	}
	#[test]
	fn livecodeserver() {
		assert_eq!(to_id("livecodeserver"), Some(103));
	}
	#[test]
	fn livescript() {
		assert_eq!(to_id("livescript"), Some(104));
	}
	#[test]
	fn lua() {
		assert_eq!(to_id("lua"), Some(105));
	}
	#[test]
	fn makefile() {
		assert_eq!(to_id("makefile"), Some(106));
	}
	#[test]
	fn markdown() {
		assert_eq!(to_id("markdown"), Some(107));
	}
	#[test]
	fn mathematica() {
		assert_eq!(to_id("mathematica"), Some(108));
	}
	#[test]
	fn matlab() {
		assert_eq!(to_id("matlab"), Some(109));
	}
	#[test]
	fn maxima() {
		assert_eq!(to_id("maxima"), Some(110));
	}
	#[test]
	fn mel() {
		assert_eq!(to_id("mel"), Some(111));
	}
	#[test]
	fn mercury() {
		assert_eq!(to_id("mercury"), Some(112));
	}
	#[test]
	fn mirc() {
		assert_eq!(to_id("mirc"), Some(113));
	}
	#[test]
	fn mizar() {
		assert_eq!(to_id("mizar"), Some(114));
	}
	#[test]
	fn mojolicious() {
		assert_eq!(to_id("mojolicious"), Some(115));
	}
	#[test]
	fn monkey() {
		assert_eq!(to_id("monkey"), Some(116));
	}
	#[test]
	fn moonscript() {
		assert_eq!(to_id("moonscript"), Some(117));
	}
	#[test]
	fn n1ql() {
		assert_eq!(to_id("n1ql"), Some(118));
	}
	#[test]
	fn nsis() {
		assert_eq!(to_id("nsis"), Some(119));
	}
	#[test]
	fn never() {
		assert_eq!(to_id("never"), Some(120));
	}
	#[test]
	fn nginx() {
		assert_eq!(to_id("nginx"), Some(121));
	}
	#[test]
	fn nim() {
		assert_eq!(to_id("nim"), Some(122));
	}
	#[test]
	fn nix() {
		assert_eq!(to_id("nix"), Some(123));
	}
	#[test]
	fn ocl() {
		assert_eq!(to_id("ocl"), Some(124));
	}
	#[test]
	fn ocaml() {
		assert_eq!(to_id("ocaml"), Some(125));
	}
	#[test]
	fn objectivec() {
		assert_eq!(to_id("objectivec"), Some(126));
	}
	#[test]
	fn glsl() {
		assert_eq!(to_id("glsl"), Some(127));
	}
	#[test]
	fn openscad() {
		assert_eq!(to_id("openscad"), Some(128));
	}
	#[test]
	fn ruleslanguage() {
		assert_eq!(to_id("ruleslanguage"), Some(129));
	}
	#[test]
	fn oxygene() {
		assert_eq!(to_id("oxygene"), Some(130));
	}
	#[test]
	fn pf() {
		assert_eq!(to_id("pf"), Some(131));
	}
	#[test]
	fn php() {
		assert_eq!(to_id("php"), Some(132));
	}
	#[test]
	fn parser3() {
		assert_eq!(to_id("parser3"), Some(133));
	}
	#[test]
	fn perl() {
		assert_eq!(to_id("perl"), Some(134));
	}
	#[test]
	fn plaintext() {
		assert_eq!(to_id("plaintext"), Some(135));
	}
	#[test]
	fn pony() {
		assert_eq!(to_id("pony"), Some(136));
	}
	#[test]
	fn pgsql() {
		assert_eq!(to_id("pgsql"), Some(137));
	}
	#[test]
	fn powershell() {
		assert_eq!(to_id("powershell"), Some(138));
	}
	#[test]
	fn processing() {
		assert_eq!(to_id("processing"), Some(139));
	}
	#[test]
	fn prolog() {
		assert_eq!(to_id("prolog"), Some(140));
	}
	#[test]
	fn properties() {
		assert_eq!(to_id("properties"), Some(141));
	}
	#[test]
	fn protobuf() {
		assert_eq!(to_id("protobuf"), Some(142));
	}
	#[test]
	fn puppet() {
		assert_eq!(to_id("puppet"), Some(143));
	}
	#[test]
	fn python() {
		assert_eq!(to_id("python"), Some(144));
	}
	#[test]
	fn profile() {
		assert_eq!(to_id("profile"), Some(145));
	}
	#[test]
	fn python-repl() {
		assert_eq!(to_id("python-repl"), Some(146));
	}
	#[test]
	fn qsharp() {
		assert_eq!(to_id("qsharp"), Some(147));
	}
	#[test]
	fn k() {
		assert_eq!(to_id("k"), Some(148));
	}
	#[test]
	fn qml() {
		assert_eq!(to_id("qml"), Some(149));
	}
	#[test]
	fn r() {
		assert_eq!(to_id("r"), Some(150));
	}
	#[test]
	fn cshtml() {
		assert_eq!(to_id("cshtml"), Some(151));
	}
	#[test]
	fn reasonml() {
		assert_eq!(to_id("reasonml"), Some(152));
	}
	#[test]
	fn redbol() {
		assert_eq!(to_id("redbol"), Some(153));
	}
	#[test]
	fn rib() {
		assert_eq!(to_id("rib"), Some(154));
	}
	#[test]
	fn rsl() {
		assert_eq!(to_id("rsl"), Some(155));
	}
	#[test]
	fn risc() {
		assert_eq!(to_id("risc"), Some(156));
	}
	#[test]
	fn graph() {
		assert_eq!(to_id("graph"), Some(157));
	}
	#[test]
	fn robot() {
		assert_eq!(to_id("robot"), Some(158));
	}
	#[test]
	fn rpm-specfile() {
		assert_eq!(to_id("rpm-specfile"), Some(159));
	}
	#[test]
	fn ruby() {
		assert_eq!(to_id("ruby"), Some(160));
	}
	#[test]
	fn rust() {
		assert_eq!(to_id("rust"), Some(161));
	}
	#[test]
	fn SAS() {
		assert_eq!(to_id("SAS"), Some(162));
	}
	#[test]
	fn scss() {
		assert_eq!(to_id("scss"), Some(163));
	}
	#[test]
	fn sql() {
		assert_eq!(to_id("sql"), Some(164));
	}
	#[test]
	fn p21() {
		assert_eq!(to_id("p21"), Some(165));
	}
	#[test]
	fn scala() {
		assert_eq!(to_id("scala"), Some(166));
	}
	#[test]
	fn scheme() {
		assert_eq!(to_id("scheme"), Some(167));
	}
	#[test]
	fn scilab() {
		assert_eq!(to_id("scilab"), Some(168));
	}
	#[test]
	fn shexc() {
		assert_eq!(to_id("shexc"), Some(169));
	}
	#[test]
	fn shell() {
		assert_eq!(to_id("shell"), Some(170));
	}
	#[test]
	fn smali() {
		assert_eq!(to_id("smali"), Some(171));
	}
	#[test]
	fn smalltalk() {
		assert_eq!(to_id("smalltalk"), Some(172));
	}
	#[test]
	fn sml() {
		assert_eq!(to_id("sml"), Some(173));
	}
	#[test]
	fn solidity() {
		assert_eq!(to_id("solidity"), Some(174));
	}
	#[test]
	fn stan() {
		assert_eq!(to_id("stan"), Some(175));
	}
	#[test]
	fn stata() {
		assert_eq!(to_id("stata"), Some(176));
	}
	#[test]
	fn iecst() {
		assert_eq!(to_id("iecst"), Some(177));
	}
	#[test]
	fn stylus() {
		assert_eq!(to_id("stylus"), Some(178));
	}
	#[test]
	fn subunit() {
		assert_eq!(to_id("subunit"), Some(179));
	}
	#[test]
	fn supercollider() {
		assert_eq!(to_id("supercollider"), Some(180));
	}
	#[test]
	fn svelte() {
		assert_eq!(to_id("svelte"), Some(181));
	}
	#[test]
	fn swift() {
		assert_eq!(to_id("swift"), Some(182));
	}
	#[test]
	fn tcl() {
		assert_eq!(to_id("tcl"), Some(183));
	}
	#[test]
	fn terraform() {
		assert_eq!(to_id("terraform"), Some(184));
	}
	#[test]
	fn tap() {
		assert_eq!(to_id("tap"), Some(185));
	}
	#[test]
	fn thrift() {
		assert_eq!(to_id("thrift"), Some(186));
	}
	#[test]
	fn tp() {
		assert_eq!(to_id("tp"), Some(187));
	}
	#[test]
	fn tsql() {
		assert_eq!(to_id("tsql"), Some(188));
	}
	#[test]
	fn twig() {
		assert_eq!(to_id("twig"), Some(189));
	}
	#[test]
	fn typescript() {
		assert_eq!(to_id("typescript"), Some(190));
	}
	#[test]
	fn unicorn-rails-log() {
		assert_eq!(to_id("unicorn-rails-log"), Some(191));
	}
	#[test]
	fn vbnet() {
		assert_eq!(to_id("vbnet"), Some(192));
	}
	#[test]
	fn vba() {
		assert_eq!(to_id("vba"), Some(193));
	}
	#[test]
	fn vbscript() {
		assert_eq!(to_id("vbscript"), Some(194));
	}
	#[test]
	fn vhdl() {
		assert_eq!(to_id("vhdl"), Some(195));
	}
	#[test]
	fn vala() {
		assert_eq!(to_id("vala"), Some(196));
	}
	#[test]
	fn verilog() {
		assert_eq!(to_id("verilog"), Some(197));
	}
	#[test]
	fn vim() {
		assert_eq!(to_id("vim"), Some(198));
	}
	#[test]
	fn axapta() {
		assert_eq!(to_id("axapta"), Some(199));
	}
	#[test]
	fn x86asm() {
		assert_eq!(to_id("x86asm"), Some(200));
	}
	#[test]
	fn xl() {
		assert_eq!(to_id("xl"), Some(201));
	}
	#[test]
	fn xquery() {
		assert_eq!(to_id("xquery"), Some(202));
	}
	#[test]
	fn yml() {
		assert_eq!(to_id("yml"), Some(203));
	}
	#[test]
	fn zephir() {
		assert_eq!(to_id("zephir"), Some(204));
	}
}
