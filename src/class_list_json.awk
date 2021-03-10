#!/usr/bin/awk -f
BEGIN {
	q="\""
	FS="\t"
	print "["
}
NR > 2{
	if ($3 == "") {
		print "\t" q last q ","
	}
}
{
	last = $2
}
END {
	print "\t" q $2 q "\n]"
}
