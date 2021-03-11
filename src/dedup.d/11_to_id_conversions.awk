#!/usr/bin/gawk -f
BEGIN {
	q="\""
	FS="\t"
	print "pub fn to_id(code: &str) -> Option<usize> {\n\tmatch code {"
}
{
	split($2, a, ",");
	gsub(/[(&].*/, "", $1);
	gsub(/[ \t]+$/, "", $1);
	print "\t\t"q a[1] q" => Some(" NR-1 "),"
}
END {
	print "\t\t_ => None\n\t}\n}"
}
