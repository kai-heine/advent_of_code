const INPUT: &str = include_str!("../inputs/day07");

fn read_input(input: &str) -> Vec<i32> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

fn median(data: &mut [i32]) -> i32 {
    let even = data.len() % 2 == 0;
    let (_, median, above) = data.select_nth_unstable(data.len() / 2 - 1);
    if even {
        let median_upper = above.iter().min().unwrap();
        (*median + *median_upper) / 2
    } else {
        *median
    }
}

fn puzzle1(input: &[i32]) -> i32 {
    let mut crab_positions = input.to_vec();
    let median = median(&mut crab_positions);
    let fuel_needed: i32 = crab_positions
        .iter()
        .map(|position| (median - position).abs())
        .sum();
    fuel_needed
}

fn partial_sums(len: usize) -> Vec<i32> {
    let mut accumulator = 0;
    (0..len as i32)
        .map(|index| {
            accumulator += index;
            accumulator
        })
        .collect()
}

fn puzzle2(crab_positions: &[i32]) -> i32 {
    let len = *crab_positions.iter().max().unwrap() as usize + 1;
    let mut fuel_consumptions = vec![0; len];
    let partial_sums = partial_sums(len);

    for crab_position in crab_positions {
        for position in 0..fuel_consumptions.len() {
            fuel_consumptions[position] +=
                partial_sums[(crab_position - position as i32).abs() as usize];
        }
    }

    *fuel_consumptions.iter().min().unwrap()
}

pub fn day07() {
    println!("\nDay 7:");
    let crab_positions = read_input(&INPUT);

    println!("Puzzle 1:");
    let fuel_needed = puzzle1(&crab_positions);
    println!("The amount of fuel needed is {}", fuel_needed);

    println!("Puzzle 2:");
    let fuel_needed_2 = puzzle2(&crab_positions);
    println!(
        "The amount of fuel needed (real crab engineering) is {}",
        fuel_needed_2
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn median_test() {
        let mut crab_positions = read_input(&EXAMPLE_INPUT);
        let median = median(&mut crab_positions);
        assert_eq!(median, 2);
    }

    #[test]
    fn puzzle1_example() {
        let mut crab_positions = read_input(&EXAMPLE_INPUT);
        let fuel_needed = puzzle1(&mut crab_positions);
        assert_eq!(fuel_needed, 37);
    }

    #[test]
    fn puzzle2_example() {
        let crab_positions = read_input(&EXAMPLE_INPUT);
        let fuel_needed = puzzle2(&crab_positions);
        assert_eq!(fuel_needed, 168);
    }
}
