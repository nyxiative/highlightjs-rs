#!/bin/sh
./languages_clean.awk languages.raw > languages.clean

echo "" > lib.rs

FILES=codegen.d/*
for f in $FILES
do
	echo "//$f" >> code.rs;
	awk -f "$f" languages.clean >> code.rs;
done
