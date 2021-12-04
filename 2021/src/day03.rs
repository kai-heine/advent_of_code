use std::{io, path::Path, vec};

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

fn to_uints(lines: &[String]) -> Vec<u32> {
    lines
        .iter()
        .map(|s| {
            s.chars().fold(0, |acc, c| match c {
                '0' => acc << 1,
                '1' => (acc << 1) | 1,
                _ => panic!("unexpected bit!"),
            })
        })
        .collect()
}

fn partition_least_most_common(report: &[u32], bit_index: usize) -> (Vec<u32>, Vec<u32>) {
    let (ones, zeros): (Vec<_>, Vec<_>) = report
        .iter()
        .partition(|&value| (value & (1 << bit_index)) != 0);

    if ones.len() >= zeros.len() {
        (zeros, ones)
    } else {
        (ones, zeros)
    }
}

fn filter_most_common(report: &[u32], bit_index: usize) -> Vec<u32> {
    let (_, most_common) = partition_least_most_common(&report, bit_index);
    most_common
}

fn filter_least_common(report: &[u32], bit_index: usize) -> Vec<u32> {
    let (least_common, _) = partition_least_most_common(&report, bit_index);
    least_common
}

fn puzzle2(diagnostic_report: &[u32], bit_width: usize) -> u32 {
    // i'm stuck trying to work with Vec<String>, so let's use actual bits (why didn't i do that in the first place?)

    let mut least_common = diagnostic_report.to_vec();
    let mut most_common = diagnostic_report.to_vec();

    for bit_index in (0..bit_width).rev() {
        most_common = filter_most_common(&most_common, bit_index);
        if most_common.len() == 1 {
            break;
        }
    }

    for bit_index in (0..bit_width).rev() {
        least_common = filter_least_common(&least_common, bit_index);
        if least_common.len() == 1 {
            break;
        }
    }

    let oxygen_generator_rating = most_common.first().unwrap();
    let co2_scrubber_rating = least_common.first().unwrap();

    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;

    life_support_rating
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

    println!("Puzzle 1:");
    let life_support_rating = puzzle2(&to_uints(&lines), lines.first().unwrap().len());
    println!(
        "The life support rating of the submarine is {}",
        life_support_rating
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

    #[test]
    fn puzzle2_example() {
        let example_input_str: Vec<String> = EXAMPLE_INPUT.iter().map(|&s| s.into()).collect();
        let example_input = to_uints(&example_input_str);

        let life_support_rating = puzzle2(&example_input, 5);
        assert_eq!(life_support_rating, 230);
    }
}
