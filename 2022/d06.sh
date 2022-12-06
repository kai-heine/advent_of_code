#!/bin/bash

find_start_sequence() {
    local line=$1
    local seq_len=$2

    local num_chars=${#line}
    for i in $(seq 0 $((num_chars - seq_len))); do
        local unique_chars
        unique_chars=$(echo "${line:i:seq_len}" | fold -w1 | sort | uniq | wc -l)
        [ "$unique_chars" -eq "$seq_len" ] && echo $((i + seq_len)) && break
    done
}

while read -r line; do
    echo "part 1: $(find_start_sequence "$line" 4)"
    echo "part 2: $(find_start_sequence "$line" 14)"
done <"$1"
