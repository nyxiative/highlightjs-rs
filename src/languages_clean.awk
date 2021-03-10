#!/usr/bin/awk -f

BEGIN { FS="\t" } 
{ 
	gsub(/[(&].*/, "", $1);
	langs=split($1, a, ",");
	split($2, b, ",");
	if (langs!=1) {
		for (i = 1; i <= langs; i++) {
			gsub(/^[ \t]+/, "", a[i]);
			gsub(/[ \t]+$/, "", a[i]);
			print a[i]"\t" b[1]"\t" $3
		}
	} else {
		gsub(/^[ \t]+/, "", a[1]);
		gsub(/[ \t]+$/, "", a[1]);
		print a[1]"\t" b[1]"\t" $3
	}
}
