#!/usr/bin/awk -f
BEGIN {
	FS="\t"
}
{
	if ($3 == "") {
		print $2
	}
}
