#!/usr/bin/awk -f
BEGIN {
	q="\""
	FS="\t"
	print "["
}

NR > 1{
	if ($3 == "") {
		print "\t" q last q ","
	}
}
{
	last=$1
}
END {
	print "\t" q $1 q "\n]"
}
