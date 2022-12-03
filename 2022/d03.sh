#!/bin/bash

unique_list() {
    echo "$1" | fold -w1 | sort | uniq
}

priority() {
    val=$(printf "%d" "'$1")
    [ $val -ge 97 ] && val=$((val - 97 + 1))
    [ $val -ge 65 ] && val=$((val - 65 + 27))
    echo $val
}

p1() {
    sum=0
    while read -r first second; do
        common=$(comm -12 \
            <(unique_list "$first") \
            <(unique_list "$second"))
        prio=$(priority "$common")
        sum=$((sum + prio))
    done < <(awk '{print substr($1,0,length/2), substr($1,length/2+1)}' "$1")
    echo $sum
}

p2() {
    sum=0
    while read -r r1 && read -r r2 && read -r r3; do
        common=$(comm -12 <(comm -12 \
            <(unique_list "$r1") \
            <(unique_list "$r2")) \
            <(unique_list "$r3"))
        prio=$(priority "$common")
        sum=$((sum + prio))
    done <"$1"
    echo $sum
}

echo "part 1: $(p1 "$1")"
echo "part 2: $(p2 "$1")"
