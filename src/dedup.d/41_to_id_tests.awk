#!/usr/bin/gawk -f
BEGIN {
	q="\""
	FS="\t"
	print "#[cfg(test)]\nmod to_id {\n\tuse super::to_id;"
}
{
	split($2, a, ",");
	gsub(/[ \t]+$/, "", a[1]);
	gsub(/^[ \t]+/, "", a[1]);
	n=a[1]
	gsub(/1/, "one", n);
	gsub(/4/, "four", n);
	gsub(/-/, "_", n);
	print "\t#[test]\n\tfn " tolower(n) "() {\n\t\tassert_eq!(to_id(" q a[1] q "), Some(" NR-1 "));\n\t}"
}
END {
	print "}"
}
