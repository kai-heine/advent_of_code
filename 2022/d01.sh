#!/bin/bash

p1() {
    paste -sd '+' "$1" |  # replace all newlines with plus signs (but keep final newline)
        sed 's/++/\n/g' | # replace double pluses with single newlines
        bc |              # evaluate lines as arithmetic expressions
        sort -nr |        # sort numerically, descending
        head -n1          # output the first line only
}

p2() {
    paste -sd '+' "$1" | sed 's/++/\n/g' | bc | sort -nr |
        head -n3 |      # get the top 3
        paste -sd '+' | # add plus signs again
        bc              # and calculate
}

echo "part 1: $(p1 "$1")"
echo "part 2: $(p2 "$1")"
