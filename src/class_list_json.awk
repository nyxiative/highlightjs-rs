#!/usr/bin/awk -f
BEGIN {
	q="\""
	FS="\t"
	print "["
}
{
	if (length($3) == 0) {
		print "\t"c q $2 q
	}
	c=","
}
END {
	print "]"
}
