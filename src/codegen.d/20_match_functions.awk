#!/usr/bin/gawk -f
BEGIN {
	print "pub fn custom_fuzzy<FM: fuzzy_matcher::FuzzyMatcher>(search: &str, matcher: &FM) -> Option<&'static str> {\n\tlet mut best = None;\n\tlet mut best_score = -1i64;\n\tfor (&k,&v) in HLJS.iter() {\n\t\tlet score = matcher.fuzzy_match(k, search).unwrap_or(-1);\n\t\tif score > best_score {\n\t\t\tbest_score = score;\n\t\t\tbest = Some(v);\n\t\t}\n\t}\n\tbest\n}\n"
	print "pub fn fuzzy(search: &str) -> Option<&'static str> {\n\tlet matcher = fuzzy_matcher::skim::SkimMatcherV2::default().ignore_case();\n\tcustom_fuzzy(search, &matcher)\n}\n"
	print "pub fn exact(language: &str) -> Option<&'static str> {\n\tHLJS.get(language).map(|r| *r)\n}\n"
	print "pub fn case_insensitive(language: &str) -> Option<&'static str> {\n\tlet lower_lang = language.to_lowercase();\n\tfor (k,&v) in HLJS.iter() {\n\t\tif k.to_lowercase() == lower_lang {\n\t\t\treturn Some(v);\n\t\t}\n\t}\n\tNone\n}\n"
}
