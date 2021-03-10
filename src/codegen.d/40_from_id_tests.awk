#!/usr/bin/gawk -f
BEGIN {
	q="\""
	FS="\t"
	print "#[cfg(test)]\nmod from_id {\n\tuse super::from_id;"
}
{
	split($2, a, ",");
	gsub(/ /, " ", a[1]);
	gsub(/[ \t]+$/, "", a[1]);
	gsub(/^[ \t]+/, "", a[1]);
	print "\t#[test]\n\tfn " a[1] "() {\n\t\tassert_eq!(from_id(" NR-1 "), Some(" q a[1] q"));\n\t}"
}
END {
	print "}"
}
