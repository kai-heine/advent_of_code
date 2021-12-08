const INPUT: &str = include_str!("../inputs/day08");

fn puzzle1(note_entries: &str) -> usize {
    note_entries
        .lines()
        .flat_map(|line| {
            line.split(" | ")
                .skip(1)
                .flat_map(|output_values| output_values.split_whitespace())
        })
        .filter(|&output| [2, 3, 4, 7].contains(&output.len()))
        .count()
}

use std::collections::HashSet;

fn solve_line(line: &str) -> i32 {
    let mut patterns = line.split_whitespace();
    let unique_patterns: Vec<HashSet<_>> = patterns
        .by_ref()
        .take_while(|s| *s != "|")
        .map(|s| s.chars().collect())
        .collect();
    let numbers: Vec<HashSet<_>> = patterns.map(|s| s.chars().collect()).collect();

    let one = unique_patterns.iter().find(|p| p.len() == 2).unwrap();
    let four = unique_patterns.iter().find(|p| p.len() == 4).unwrap();

    let mut result = 0;
    for number in &numbers {
        result *= 10;

        result += match number.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            5 => {
                if (number - one).len() == 3 {
                    3
                } else if (number - four).len() == 2 {
                    5
                } else {
                    2
                }
            }
            6 => {
                if (one - number).len() == 1 {
                    6
                } else if (four - number).len() == 0 {
                    9
                } else {
                    0
                }
            }
            _ => panic!("that's not a valid 7 segment number!"),
        }
    }

    result
}

fn puzzle2(note_entries: &str) -> i32 {
    note_entries.lines().map(|line| solve_line(line)).sum()
}

pub fn day08() {
    println!("\nDay 8:");

    println!("Puzzle 1:");
    let num_easy_digits = puzzle1(&INPUT);
    println!(
        "In the output values, the digits 1, 4, 7 and 8 appear {} times",
        num_easy_digits
    );

    println!("Puzzle 2:");
    let sum_of_output_values = puzzle2(&INPUT);
    println!("The sum of all output values is {}", sum_of_output_values);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn puzzle1_example() {
        let num_easy_digits = puzzle1(&TEST_INPUT);
        assert_eq!(num_easy_digits, 26);
    }

    #[test]
    fn puzzle2_example() {
        let mut lines = TEST_INPUT.lines();
        assert_eq!(solve_line(lines.next().unwrap()), 8394);
        assert_eq!(solve_line(lines.next().unwrap()), 9781);
        assert_eq!(solve_line(lines.next().unwrap()), 1197);
        assert_eq!(solve_line(lines.next().unwrap()), 9361);
        assert_eq!(solve_line(lines.next().unwrap()), 4873);
        assert_eq!(solve_line(lines.next().unwrap()), 8418);
        assert_eq!(solve_line(lines.next().unwrap()), 4548);
        assert_eq!(solve_line(lines.next().unwrap()), 1625);
        assert_eq!(solve_line(lines.next().unwrap()), 8717);
        assert_eq!(solve_line(lines.next().unwrap()), 4315);

        let sum_of_output_values = puzzle2(&TEST_INPUT);
        assert_eq!(sum_of_output_values, 61229);
    }
}
