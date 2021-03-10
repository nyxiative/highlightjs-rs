#!/bin/sh
./languages_clean.awk languages.raw > languages.clean

echo "" > lib.rs

FILES=codegen.d/*
for f in $FILES
do
	echo "//$f" >> lib.rs;
	awk -f "$f" languages.clean >> lib.rs;
done

awk -f class_list_json.awk languages.clean > classes.json
awk -f class_list_raw.awk languages.clean > classes.raw
awk -f human_readable_json.awk languages.clean > languages.json

echo "pub static CLASSJSON: &'static str = include_str!('classes.json')" >> lib.rs
echo "pub static CLASSRAW: &'static str = include_str!('classes.raw')" >> lib.rs
echo "pub static LANGSJSON: &'static str = include_str!('languages.json')" >> lib.rs
