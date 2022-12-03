#!/bin/bash

p1() {
    local sum=0
    while read -r x y; do
        val=$(((y - x + 1) % 3))
        [ $val -lt 0 ] && val=$((val + 3))
        sum=$((sum + y + val * 3))
    done < <(tr 'ABCXYZ' '123123' <"$1")
    echo $sum
}

p2() {
    declare -A arr
    arr[0, 0]=3
    arr[0, 1]=1
    arr[0, 2]=2
    arr[1, 0]=1
    arr[1, 1]=2
    arr[1, 2]=3
    arr[2, 0]=2
    arr[2, 1]=3
    arr[2, 2]=1

    local sum=0
    while read -r op res; do
        choice=${arr[$op, $res]}
        sum=$((sum + choice + res * 3))
    done < <(tr 'ABCXYZ' '012012' <$1)
    echo $sum
}

echo "part 1: $(p1 "$1")"
echo "part 2: $(p2 "$1")"
