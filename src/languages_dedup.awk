#!/usr/bin/awk -f

BEGIN { FS="\t" } 
{ 
	gsub(/[(&].*/, "", $1);
	langs=split($1, a, ",");
	split($2, b, ",");
	gsub(/^[ \t]+/, "", a[1]);
	gsub(/[ \t]+$/, "", a[1]);
	print a[1]"\t" b[1]"\t" $3
}
