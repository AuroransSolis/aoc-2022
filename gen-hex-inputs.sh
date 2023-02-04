#!/bin/zsh

for input in inputs/d{1..25}{,test}.txt
do
    if [ -f "$input" ]
    then
        output="$(echo $input | sd '(.+).txt' '$1-hex.txt')"
        if [ ! -f "$output" ]
        then
            hexdump -ve '1/1 "%02x\n"' "$input" >"$output"
        fi
    fi
done
