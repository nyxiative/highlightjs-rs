#!/usr/bin/gawk -f
BEGIN {
	q="\""
	FS="\t"
}
BEGINFILE {
	numLines = 0
	while ( (getline line < FILENAME) > 0 ) {
		numLines++
	}
	print "extern crate lazy_static;\nuse std::collections::HashMap;\n\nlazy_static::lazy_static! {\n\tstatic ref HLJS: HashMap<&'static str, &'static str> = {\n\t\tlet mut map = HashMap::with_capacity("numLines");";
}
{
	split($2, a, ",");
	gsub(/[(&].*/, "", $1);
	gsub(/[ \t]+$/, "", $1);
	print "\t\tmap.insert("q$1q", "q$2q");"
}
END {
	print "\t\tmap\n\t};\n}"
}
