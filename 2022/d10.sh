#!/bin/bash

x=(1)

while read -r instr val; do
    case $instr in
    noop)
        x+=("${x[-1]}")
        ;;
    addx)
        x+=("${x[-1]}" "$((x[-1] + val))")
        ;;
    *)
        echo "Illegal instruction \"$instr\"!"
        ;;
    esac
done <"$1"

sum=0

for i in {20..220..40}; do
    sum=$((sum + i * x[i - 1]))
done

echo "part 1: $sum"

echo "part 2:"

for i in {0..239}; do
    col=$((i%40))
    distance=$((col - x[i]))
    if [ $distance -ge -1 ] && [ $distance -le 1 ]; then
        echo -n "#"
    else
        echo -n " "
    fi

    [ $col -eq 39 ] && echo ""
done
