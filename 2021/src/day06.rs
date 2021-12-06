fn read_input(input: &str) -> Vec<usize> {
    input
        .trim_end()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn age_fishies(fishies: &mut [usize]) -> usize {
    let mut new_fishies = 0;

    for fishie in fishies {
        if *fishie == 0 {
            *fishie = 6;
            new_fishies += 1;
        } else {
            *fishie -= 1;
        }
    }

    new_fishies
}

// naive approach
fn puzzle1(input: &[usize], days: usize) -> usize {
    let mut fishies = input.to_vec();

    for _ in 0..days {
        let new_fishies = age_fishies(&mut fishies);
        fishies.resize_with(fishies.len() + new_fishies, || 8);
    }

    fishies.len()
}

// thats a rotate!
fn puzzle2(input: &[usize], days: usize) -> usize {
    // array of fish counts where the index represents the days until the fish reproduces
    let mut fish_pipeline = [0; 9];
    for remaining_days in input {
        fish_pipeline[*remaining_days] += 1;
    }

    for _ in 0..days {
        fish_pipeline.rotate_left(1); // fish with a value of 0 produce new fish with a value of 8
        fish_pipeline[6] += fish_pipeline[8]; // but also start over with a value of 6 themselves
    }

    fish_pipeline.iter().sum()
}

pub fn day06() {
    println!("\nDay 6:");
    let input = read_input(
        &std::fs::read_to_string("inputs/day06").expect("could not read input for day06"),
    );

    println!("Puzzle 1:");
    let fishie_count = puzzle1(&input, 80);
    println!("After 80 days, there are {} fishies", fishie_count);

    println!("Puzzle 2:");
    let fishie_count256 = puzzle2(&input, 256);
    println!("After 256 days, there are {} fishies", fishie_count256);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn input_test() {
        let input = read_input(&EXAMPLE_INPUT);
        assert_eq!(input, vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn puzzle1_example() {
        let input = read_input(&EXAMPLE_INPUT);

        let mut fishies = input.clone();
        let mut new_fishies = age_fishies(&mut fishies);
        assert_eq!(new_fishies, 0);
        assert_eq!(fishies, vec![2, 3, 2, 0, 1]);

        new_fishies = age_fishies(&mut fishies);
        assert_eq!(new_fishies, 1);
        assert_eq!(fishies, vec![1, 2, 1, 6, 0]);

        let fishie_count_18 = puzzle1(&input, 18);
        assert_eq!(fishie_count_18, 26);

        let fishie_count_80 = puzzle1(&input, 80);
        assert_eq!(fishie_count_80, 5934);
    }

    #[test]
    fn puzzle2_example() {
        let input = read_input(&EXAMPLE_INPUT);
        let fishie_count_256 = puzzle2(&input, 256);
        assert_eq!(fishie_count_256, 26984457539);
    }
}
