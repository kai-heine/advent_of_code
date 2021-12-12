use std::cmp::{max, min};

const INPUT: &str = include_str!("../inputs/day11");

#[derive(Debug, PartialEq)]
struct Octopus {
    level: u32,
    flashing: bool,
}

impl Octopus {
    fn new() -> Octopus {
        Octopus {
            level: 0,
            flashing: false,
        }
    }
    fn with_level(level: u32) -> Octopus {
        Octopus {
            level,
            flashing: false,
        }
    }
}

fn load_octopuses(input: &str) -> Vec<Vec<Octopus>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Octopus::with_level(c.to_digit(10).unwrap()))
                .collect()
        })
        .collect()
}

fn step(octopuses: &mut Vec<Vec<Octopus>>) -> usize {
    let rows = octopuses.len();
    let cols = octopuses[0].len();

    for octopus in octopuses.iter_mut().flatten() {
        octopus.level += 1;
    }

    let mut total_flashes = 0;

    loop {
        let mut new_flashing = 0;
        for y in 0..rows {
            for x in 0..cols {
                if octopuses[y][x].flashing || octopuses[y][x].level <= 9 {
                    continue;
                }

                new_flashing += 1;
                octopuses[y][x].flashing = true;

                for ya in max(y as i32 - 1, 0)..min(y as i32 + 2, rows as i32) {
                    for xa in max(x as i32 - 1, 0)..min(x as i32 + 2, cols as i32) {
                        octopuses[ya as usize][xa as usize].level += 1;
                    }
                }
            }
        }

        total_flashes += new_flashing;

        if new_flashing == 0 {
            break;
        }
    }

    for octopus in octopuses.iter_mut().flatten() {
        if octopus.flashing {
            *octopus = Octopus::new();
        }
    }

    total_flashes
}

fn puzzle1(input: &str, steps: u32) -> usize {
    // TIL: octopuses is actually correct, octopodes is less common and octopi is wrong
    let mut octopuses = load_octopuses(input);

    let mut total_flashes = 0;

    for _ in 0..steps {
        total_flashes += step(&mut octopuses);
    }

    total_flashes
}

fn puzzle2(input: &str) -> usize {
    let mut octopuses = load_octopuses(input);
    let num_octopuses = octopuses.len() * octopuses[0].len();

    let mut steps = 0;
    loop {
        steps += 1;
        let flashes = step(&mut octopuses);
        if flashes == num_octopuses {
            return steps;
        }
    }
}

pub fn day11() {
    println!("\nDay 11:");

    println!("Puzzle 1:");
    let total_flashes = puzzle1(INPUT, 100);
    println!(
        "The total number of flashes after 100 steps is {}",
        total_flashes
    );

    println!("Puzzle 2:");
    let steps = puzzle2(INPUT);
    println!(
        "The first step during which all octopuses flash is {}",
        steps
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn puzzle1_example() {
        let total_flashes = puzzle1(TEST_INPUT, 100);
        assert_eq!(total_flashes, 1656);
    }

    #[test]
    fn puzzle2_example() {
        let steps = puzzle2(TEST_INPUT);
        assert_eq!(steps, 195);
    }
}
