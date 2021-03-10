#!/bin/sh
./languages_clean.awk languages.raw > languages.clean
./languages_dedup.awk languages.raw > languages.dedup

echo "" > lib.rs

for f in clean.d/*
do
	echo "//$f" >> lib.rs;
	awk -f "$f" languages.clean >> lib.rs;
done

for f in dedup.d/*
do
	echo "//$f" >> lib.rs;
	awk -f "$f" languages.dedup >> lib.rs;
done

awk -f class_list_json.awk languages.clean > classes.json
awk -f class_list_raw.awk languages.clean > classes.raw
awk -f human_readable_json.awk languages.clean > human_readable.json
awk -f human_readable_raw.awk languages.clean > human_readable.raw

echo "pub static CLASSJSON: &'static str = include_str!(\"classes.json\");" >> lib.rs
echo "pub static CLASSRAW: &'static str = include_str!(\"classes.raw\");" >> lib.rs
echo "pub static LANGSJSON: &'static str = include_str!(\"human_readable.json\");" >> lib.rs
echo "pub static LANGSRAW: &'static str = include_str!(\"human_readable.raw\");" >> lib.rs
