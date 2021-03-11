#!/usr/bin/awk -f
BEGIN {
	q="\""
	FS="\t"
	print "["
}

{
	if ($3 == "") {
		print "\t" c q $1 q
	}
	c=","
}
END {
	print "]"
}
