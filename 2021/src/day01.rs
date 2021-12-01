use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines_as_numbers<P>(path: P) -> io::Result<Vec<i32>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let depths: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    Ok(depths)
}

fn count_increases(depths: &[i32]) -> i32 {
    depths
        .iter()
        .zip(depths.iter().skip(1))
        .map(|(first, second)| (second > first) as i32)
        .sum()
}

fn sliding_window(depths: &[i32]) -> Vec<i32> {
    depths
        .windows(3)
        .map(|window| window.iter().sum())
        .collect()
}

fn puzzle1(depths: &[i32]) -> i32 {
    count_increases(&depths)
}

fn puzzle2(depths: &[i32]) -> i32 {
    let windows = sliding_window(depths);
    count_increases(&windows)
}

pub fn day01() {
    let depths = read_lines_as_numbers("./inputs/day01").expect("could not read input file!");

    println!("Day 1:");

    let puzzle1_result = puzzle1(&depths);
    println!(
        "Puzzle 1:\nThe number of times a depth measurement increases is {}",
        puzzle1_result
    );

    let puzzle2_result = puzzle2(&depths);
    println!(
        "Puzzle 2:\nThe number of times the sum of measurements in the sliding window increases is {}",
        puzzle2_result
    );
}

#[cfg(test)]
mod tests {
    use crate::day01::*;

    const EXAMPLE_INPUT: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn puzzle1_example() {
        assert_eq!(puzzle1(&EXAMPLE_INPUT), 7);
    }

    #[test]
    fn puzzle2_example() {
        assert_eq!(
            sliding_window(&EXAMPLE_INPUT),
            [607, 618, 618, 617, 647, 716, 769, 792]
        );
        assert_eq!(puzzle2(&EXAMPLE_INPUT), 5);
    }
}
