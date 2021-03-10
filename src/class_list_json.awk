#!/usr/bin/awk -f
BEGIN {
	q="\""
	FS="\t"
	print "["
}
{
	if ($3 == "") {
		print "\t" q $2 q ","
	}
}
END {
	print "]"
}
