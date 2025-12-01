#!/bin/bash

day=$(ls -d day-* 2>/dev/null | sed 's/day-0*//' | sort -n | tail -1)
if [ -z "$day" ]; then
    day=0
fi

echo "$day"

url="https://adventofcode.com/2025/day/${day}/input"


formatted_day=$(printf "%02d" $day)
new_folder="day-$formatted_day"

mkdir -p "${new_folder}"

export $(cat .env | xargs)

curl -b "session=${SESSION_COOKIE}" "$url" > "${new_folder}/input.txt"