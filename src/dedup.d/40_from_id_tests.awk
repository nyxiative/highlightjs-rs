#!/usr/bin/gawk -f
BEGIN {
	q="\""
	FS="\t"
	print "#[cfg(test)]\nmod from_id {\n\tuse super::from_id;"
}
{
	split($2, a, ",");
	gsub(/[ \t]+$/, "", a[1]);
	gsub(/^[ \t]+/, "", a[1]);
	n=a[1]
	gsub(/1/, "one", n);
	gsub(/4/, "four", n);
	gsub(/-/, "_", n);
	print "\t#[test]\n\tfn " tolower(n) "() {\n\t\tassert_eq!(from_id(" NR-1 "), Some(" q a[1] q"));\n\t}"
}
END {
	print "}"
}
