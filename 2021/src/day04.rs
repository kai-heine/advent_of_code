use std::{io, path::Path};

#[derive(Debug, Clone)]
struct BingoNumber {
    value: i32,
    marked: bool,
}

type BingoBoard = Vec<Vec<BingoNumber>>;

const ROWS: usize = 5;
const COLS: usize = 5;

fn read_bingo_input<P: AsRef<Path>>(path: P) -> io::Result<(Vec<i32>, Vec<BingoBoard>)> {
    let input = std::fs::read_to_string(path)?;
    let mut chunks = input.split("\n\n");

    let drawn_numbers: Vec<i32> = chunks
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let grids: Vec<BingoBoard> = chunks
        .map(|chunk| {
            chunk
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| BingoNumber {
                            value: num.parse().unwrap(),
                            marked: false,
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    Ok((drawn_numbers, grids))
}

fn draw_number(drawn_number: i32, board: &mut BingoBoard) {
    for row in board {
        for num in row {
            if num.value == drawn_number {
                num.marked = true;
            }
        }
    }
}

fn check_board(board: &BingoBoard) -> Option<i32> {
    let mut row_sums = [0; ROWS];
    let mut col_sums = [0; COLS];
    let mut unmarked_sum = 0;

    for (row, numbers) in board.iter().enumerate() {
        for (col, number) in numbers.iter().enumerate() {
            if number.marked {
                row_sums[row] += 1;
                col_sums[col] += 1;
            } else {
                unmarked_sum += number.value;
            }
        }
    }

    if row_sums.contains(&COLS) || col_sums.contains(&ROWS) {
        Some(unmarked_sum)
    } else {
        None
    }
}

fn puzzle1(drawn_numbers: &[i32], boards: &Vec<BingoBoard>) -> i32 {
    let mut boards = boards.clone();
    for number in drawn_numbers {
        for board in &mut boards {
            draw_number(*number, board);
            if let Some(unmarked_sum) = check_board(board) {
                let final_score = unmarked_sum * number;
                return final_score;
            }
        }
    }
    panic!("could not find a winning board!");
}

fn puzzle2(drawn_numbers: &[i32], boards: &Vec<BingoBoard>) -> i32 {
    let mut boards = boards.clone();
    let mut last_winning_score = None;

    for number in drawn_numbers {
        for board in &mut boards {
            draw_number(*number, board);
        }
        // iterate a second time so that numbers are marked for all boards
        boards.retain(|board| {
            if let Some(unmarked_sum) = check_board(&board) {
                last_winning_score = Some(unmarked_sum * number);
                false
            } else {
                true
            }
        });
    }

    last_winning_score.expect("could not find a winning board")
}

pub fn day04() {
    println!("\nDay 4:");
    let (drawn_numbers, boards) = read_bingo_input("inputs/day04").expect("could not read input");

    println!("Puzzle 1:");
    let final_score1 = puzzle1(&drawn_numbers, &boards);
    println!(
        "The final score when choosing the first winning board is {}",
        final_score1
    );

    println!("Puzzle 2:");
    let final_score2 = puzzle2(&drawn_numbers, &boards);
    println!(
        "The final score when choosing the last winning board is {}",
        final_score2
    );
}

#[test]
fn puzzle1_example() {
    let (drawn_numbers, boards) =
        read_bingo_input("inputs/day04_example").expect("could not read input");
    let final_score = puzzle1(&drawn_numbers, &boards);
    assert_eq!(final_score, 4512);
}

#[test]
fn puzzle2_example() {
    let (drawn_numbers, boards) =
        read_bingo_input("inputs/day04_example").expect("could not read input");
    let final_score = puzzle2(&drawn_numbers, &boards);
    assert_eq!(final_score, 1924);
}
