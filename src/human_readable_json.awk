#!/usr/bin/awk -f
BEGIN {
	q="\""
	FS="\t"
	print "["
}
{
	if ($3 == "") {
		print "\t" q $1 q ","
	}
}
END {
	print "]"
}
