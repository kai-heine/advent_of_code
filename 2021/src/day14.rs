use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day14");

fn read_input(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let (t, r) = input.split_once("\n\n").unwrap();

    let template = t.chars().collect();

    let rules = r
        .lines()
        .map(|l| {
            let (pair, insert) = l.split_once(" -> ").unwrap();
            (
                (pair.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap()),
                insert.chars().nth(0).unwrap(),
            )
        })
        .collect();

    (template, rules)
}

fn step(template: &[char], rules: &HashMap<(char, char), char>) -> Vec<char> {
    let mut new_template = Vec::new();
    for (left, right) in template.iter().zip(template.iter().skip(1)) {
        new_template.push(*left);
        if let Some(insert) = rules.get(&(*left, *right)) {
            new_template.push(*insert);
        }
    }
    new_template.push(*template.last().unwrap());
    new_template
}

fn puzzle(input: &str, steps: usize) -> usize {
    let (mut template, rules) = read_input(input);

    for _ in 0..steps {
        template = step(&template, &rules);
    }

    let char_counts = template.iter().fold(HashMap::new(), |mut counts, &c| {
        if let Some(x) = counts.get_mut(&c) {
            *x += 1;
        } else {
            counts.insert(c, 1);
        }
        counts
    });

    let (_, min) = char_counts.iter().min_by_key(|&(_, v)| v).unwrap();
    let (_, max) = char_counts.iter().max_by_key(|&(_, v)| v).unwrap();

    max - min
}

pub fn day14() {
    let p1 = puzzle(&INPUT, 10);
    println!("d14 p1: {}", p1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn p1_test() {
        let res = puzzle(&TEST_INPUT, 10);
        assert_eq!(res, 1588);
    }
}
