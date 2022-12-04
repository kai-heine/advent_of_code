#!/bin/bash

p1() {
    sum=0
    while IFS='-,' read -r f1 l1 f2 l2; do
        num_sections1=$((l1 - f1 + 1))
        num_sections2=$((l2 - f2 + 1))
        num_overlaps=$(comm -12 <(seq "$f1" "$l1" | sort) <(seq "$f2" "$l2" | sort) | wc -l)
        if [ $num_sections1 -eq "$num_overlaps" ] || [ $num_sections2 -eq "$num_overlaps" ]; then
            sum=$((sum + 1))
        fi
    done <"$1"
    echo $sum
}

p2() {
    sum=0
    while IFS='-,' read -r f1 l1 f2 l2; do
        [ -n "$(comm -12 <(seq "$f1" "$l1" | sort) <(seq "$f2" "$l2" | sort))" ] && sum=$((sum+1))
    done <"$1"
    echo $sum
}

echo "part 1: $(p1 "$1")"
echo "part 2: $(p2 "$1")"
