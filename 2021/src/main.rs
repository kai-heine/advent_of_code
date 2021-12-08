use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() {
    println!("Advent of Code 2021\n");

    let days = [
        day01::day01,
        day02::day02,
        day03::day03,
        day04::day04,
        day05::day05,
        day06::day06,
        day07::day07,
        day08::day08,
    ];

    let mut days_to_run: Vec<usize> = env::args()
        .skip(1)
        .map(|d| d.parse::<usize>().expect("could not parse cli argument") - 1)
        .collect();

    if days_to_run.is_empty() {
        days_to_run = (0..days.len()).collect();
    }

    for day in days_to_run {
        days[day]();
    }
}
