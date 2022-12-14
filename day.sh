#!/bin/sh

set -e

DAY=$1
touch input/day_"$DAY".txt 
touch input/day_"$DAY"_example.txt 
cp src/template.rs src/day_"$DAY".rs


PREV=$((DAY - 1))
HEAD_COUNT=$(grep -n "day $PREV" src/main.rs | cut -d':' -f 1)
TOTAL=$(cat src/main.rs | wc -l)
TAIL_COUNT=$((TOTAL - HEAD_COUNT))

head -n $HEAD_COUNT src/main.rs > src/temp_main.rs
echo "    day $DAY" >> src/temp_main.rs
tail -n $TAIL_COUNT src/main.rs >> src/temp_main.rs

mv src/temp_main.rs src/main.rs
