use std::{io, path::Path};

fn read_lines<P>(path: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let lines: Vec<String> = std::fs::read_to_string(path)?
        .lines()
        .map(String::from)
        .collect();
    Ok(lines)
}

// what the hell is even that
fn accumulate_bits(lines: &[String]) -> Vec<u32> {
    let init = vec![0; lines.first().unwrap().len()];
    lines.iter().fold(init, |sum, s| {
        assert_eq!(sum.len(), s.len());
        sum.iter()
            .zip(s.chars())
            .map(|(bit_sum, bit)| match bit {
                '0' => *bit_sum,
                '1' => bit_sum + 1,
                _ => panic!("unexpected bit!"),
            })
            .collect()
    })
}

fn get_rate<F>(bit_counts: &[u32], predicate: F) -> u32
where
    F: Fn(u32) -> bool,
{
    bit_counts.iter().fold(0, |result, bit_count| {
        (result << 1) | predicate(*bit_count) as u32
    })
}

fn puzzle1(lines: &[String]) -> u32 {
    let bit_counts = accumulate_bits(lines);

    let gamma_rate = get_rate(&bit_counts, |bit_count| {
        bit_count > (lines.len() as u32 / 2)
    });

    let epsilon_rate = get_rate(&bit_counts, |bit_count| {
        bit_count < (lines.len() as u32 / 2)
    });

    let power_consumption = gamma_rate * epsilon_rate;

    power_consumption
}

pub fn day03() {
    println!("\nDay 3:");
    let lines = read_lines("./inputs/day03").expect("could not read input!");

    println!("Puzzle 1:");
    let power_consumption = puzzle1(&lines);
    println!(
        "The power consumption of the submarine is {}",
        power_consumption
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // is there a way to have global constants that are initialized at runtime? (like static const in c++...)
    const EXAMPLE_INPUT: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn puzzle1_example() {
        // the better way would be to define functions that don't care what the string type is
        let example_input: Vec<String> = EXAMPLE_INPUT.iter().map(|&s| s.into()).collect();
        let power_consumption = puzzle1(&example_input);
        assert_eq!(power_consumption, 198);
    }
}
