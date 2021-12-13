use std::cmp::max;

const INPUT: &str = include_str!("../inputs/day13");

enum Fold {
    Vertical(usize),
    Horizontal(usize),
}

fn read_input(input: &str) -> (Vec<Vec<bool>>, Vec<Fold>) {
    let (dots_str, folds_str) = input.split_once("\n\n").unwrap();
    let dots: Vec<(usize, usize)> = dots_str
        .lines()
        .map(|l| {
            let mut i = l.split(',').map(|n| n.parse().unwrap());
            (i.next().unwrap(), i.next().unwrap())
        })
        .collect();

    let folds: Vec<Fold> = folds_str
        .lines()
        .map(|l| {
            let (axis, index) = l
                .split_whitespace()
                .last()
                .unwrap()
                .split_once('=')
                .unwrap();
            match axis {
                "x" => Fold::Vertical(index.parse().unwrap()),
                "y" => Fold::Horizontal(index.parse().unwrap()),
                _ => panic!(),
            }
        })
        .collect();

    let (cols, rows) = dots.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
        (max(max_x, *x + 1), max(max_y, *y + 1))
    });

    let mut paper = vec![vec![false; cols]; rows];

    for (x, y) in dots {
        paper[y][x] = true;
    }

    (paper, folds)
}

fn fold_vertically(paper: &mut Vec<Vec<bool>>, column: usize) {
    let rightmost = paper[0].len() - 1;
    for y in 0..paper.len() {
        for x in 0..column {
            paper[y][x] |= paper[y][rightmost - x];
        }
        paper[y].resize(column, false);
    }
}

fn fold_horizontally(paper: &mut Vec<Vec<bool>>, row: usize) {
    let lowest = paper.len() - 1;
    for y in 0..row {
        for x in 0..paper[y].len() {
            paper[y][x] |= paper[lowest - y][x];
        }
    }
    paper.resize(row, Vec::new());
}

fn dot_count(paper: &Vec<Vec<bool>>) -> usize {
    paper
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&cell| *cell)
        .count()
}

fn puzzle1(input: &str) -> usize {
    let (mut paper, folds) = read_input(input);
    let fold = folds.first().unwrap();
    match *fold {
        Fold::Vertical(x) => fold_vertically(&mut paper, x),
        Fold::Horizontal(y) => fold_horizontally(&mut paper, y),
    }

    dot_count(&paper)
}

fn puzzle2(input: &str) {
    let (mut paper, folds) = read_input(input);
    for fold in folds {
        match fold {
            Fold::Vertical(x) => fold_vertically(&mut paper, x),
            Fold::Horizontal(y) => fold_horizontally(&mut paper, y),
        }
    }

    for y in 0..paper.len() {
        for x in 0..paper[y].len() {
            print!("{}", if paper[y][x] { '#' } else { '.' });
        }
        print!("\n");
    }
}

pub fn day13() {
    println!("Day 13");

    let visible_dots = puzzle1(&INPUT);
    println!("Visible dots after first fold: {}", visible_dots);

    println!("Paper after all folds:");
    puzzle2(&INPUT);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn p1_test() {
        assert_eq!(puzzle1(&TEST_INPUT), 17);

        let (mut paper, _) = read_input(TEST_INPUT);
        fold_horizontally(&mut paper, 7);
        assert_eq!(dot_count(&paper), 17);
        fold_vertically(&mut paper, 5);
        assert_eq!(dot_count(&paper), 16);
    }
}
