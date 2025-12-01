#!/bin/bash

day=$(ls -d day-* 2>/dev/null | sed 's/day-0*//' | sort -n | tail -1)
if [ -z "$day" ]; then
    day=0
fi
day=$((day + 1))

formatted_day=$(printf "%02d" $day)
new_folder="day-$formatted_day" 

mkdir $new_folder
cargo init $new_folder
touch $new_folder/test.txt

cp aoc.template.rs $new_folder/src/main.rs

export "$(cat .env | xargs)"

url="https://adventofcode.com/${YEAR}/day/${day}"
title="$(curl -s --cookie "session=${SESSION_COOKIE}" "$url" | pup 'article h2 text{}' | sed 's/--- Day [0-9]*: \(.*\) ---/\1/')"
echo "| ${day}   | [${title}](https://adventofcode.com/${YEAR}/day/${day}) | ★★ |" >> README.md