use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    fn new(line: &str) -> Self {
        let (cmd_str, num_str) = line.split_once(' ').unwrap();
        let number = num_str.parse::<i32>().unwrap();
        match cmd_str {
            "forward" => Self::Forward(number),
            "down" => Self::Down(number),
            "up" => Self::Up(number),
            _ => panic!(),
        }
    }
}

fn read_commands<P>(path: P) -> io::Result<Vec<Command>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let commands: Vec<Command> = reader
        .lines()
        .map(|line| Command::new(&line.unwrap()))
        .collect();

    Ok(commands)
}

#[derive(Default, PartialEq, Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Command {
    fn update_position(&self, mut pos: Position) -> Position {
        match self {
            Command::Forward(x) => {
                pos.horizontal += x;
                pos
            }
            Command::Down(x) => {
                pos.depth += x;
                pos
            }
            Command::Up(x) => {
                pos.depth -= x;
                pos
            }
        }
    }

    fn update_position_with_aim(&self, mut pos: Position) -> Position {
        match self {
            Command::Down(x) => {
                pos.aim += x;
                pos
            }
            Command::Up(x) => {
                pos.aim -= x;
                pos
            }
            Command::Forward(x) => {
                pos.horizontal += x;
                pos.depth += pos.aim * x;
                pos
            }
        }
    }
}

fn accumulate_position<F>(init: Position, commands: &[Command], update_func: F) -> Position
where
    F: Fn(&Command, Position) -> Position,
{
    commands.iter().fold(init, |pos, cmd| update_func(cmd, pos))
}

fn puzzle1(input: &[Command]) -> i32 {
    let final_position = accumulate_position(Position::default(), input, Command::update_position);

    final_position.horizontal * final_position.depth
}

fn puzzle2(input: &[Command]) -> i32 {
    let final_position = accumulate_position(
        Position::default(),
        input,
        Command::update_position_with_aim,
    );

    final_position.horizontal * final_position.depth
}

pub fn day02() {
    println!("\nDay 2:");
    let commands = read_commands("./inputs/day02").expect("could not read input for day 2!");

    println!("Puzzle 1:");
    let puzzle1_result = puzzle1(&commands);
    println!(
        "The product of the final horizontal position and the final depth is {}.",
        puzzle1_result
    );

    println!("Puzzle 2:");
    let puzzle2_result = puzzle2(&commands);
    println!(
        "The product of the final horizontal position and the final depth (considering aim) is {}.\n",
        puzzle2_result
    );
}

#[cfg(test)]
mod tests {
    use crate::day02::*;

    const EXAMPLE_INPUT: [Command; 6] = [
        Command::Forward(5),
        Command::Down(5),
        Command::Forward(8),
        Command::Up(3),
        Command::Down(8),
        Command::Forward(2),
    ];

    #[test]
    fn puzzle1_example() {
        let final_pos = accumulate_position(
            Position::default(),
            &EXAMPLE_INPUT,
            Command::update_position,
        );
        assert_eq!(final_pos.horizontal, 15);
        assert_eq!(final_pos.depth, 10);

        let puzzle_result = puzzle1(&EXAMPLE_INPUT);
        assert_eq!(puzzle_result, 150);
    }

    #[test]
    fn puzzle2_example() {
        let final_pos = accumulate_position(
            Position::default(),
            &EXAMPLE_INPUT,
            Command::update_position_with_aim,
        );
        assert_eq!(final_pos.horizontal, 15);
        assert_eq!(final_pos.depth, 60);

        let puzzle_result = puzzle2(&EXAMPLE_INPUT);
        assert_eq!(puzzle_result, 900);
    }
}
