#! /bin/bash 

YEAR=2023
if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters"
    echo "Usage: ./get_input.sh <n>"
fi 

padded=$1
printf -v padded "%02d" $1
filename="src/day$padded/input.txt"


curl "https://adventofcode.com/$YEAR/day/$1/input" --cookie "$(cat cookie.txt)" > $filename

