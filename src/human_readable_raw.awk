#!/usr/bin/awk -f
BEGIN {
	q="\""
	FS="\t"
}
{
	if ($3 == "") {
		print $1
	}
}
