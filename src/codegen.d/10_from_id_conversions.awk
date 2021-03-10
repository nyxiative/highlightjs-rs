#!/usr/bin/gawk -f
BEGIN {
	q="\""
	FS="\t"
	print "pub fn from_id(id: usize) -> Option<&'static str> {\n\tmatch id {"
}
{
	split($2, a, ",");
	gsub(/[(&].*/, "", $1);
	gsub(/[ \t]+$/, "", $1);
	print "\t\t" NR-1 " => Some("q a[1] q"),"
}
END {
	print "\t\t_ => None\n\t}\n}"
}
