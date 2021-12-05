#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

fn parse_lines(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let mut points = line.split(" -> ").map(|point| {
                let mut nums = point.split(',').map(|num| num.parse().unwrap());

                Point {
                    x: nums.next().unwrap(),
                    y: nums.next().unwrap(),
                }
            });

            Line {
                start: points.next().unwrap(),
                end: points.next().unwrap(),
            }
        })
        .collect()
}

type Grid = Vec<Vec<i32>>;

fn make_grid(width: usize, height: usize) -> Grid {
    vec![vec![0; width]; height]
}

fn draw_line(line: &Line, grid: &mut Grid) {
    let move_to = |p: &mut Point, end: &Point| {
        p.x += (end.x - p.x).signum();
        p.y += (end.y - p.y).signum();
    };

    let mut point = line.start;

    grid[point.y as usize][point.x as usize] += 1;
    while point != line.end {
        move_to(&mut point, &line.end);
        grid[point.y as usize][point.x as usize] += 1;
    }
}

fn puzzle1(width: usize, height: usize, lines: &[Line]) -> usize {
    let mut grid = make_grid(width, height);

    for line in lines
        .iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
    {
        draw_line(line, &mut grid);
    }

    grid.iter().fold(0, |sum, row| {
        sum + row
            .iter()
            .filter(|&num_overlaps| *num_overlaps >= 2)
            .count()
    })
}

fn puzzle2(width: usize, height: usize, lines: &[Line]) -> usize {
    let mut grid = make_grid(width, height);
    for line in lines {
        draw_line(line, &mut grid);
    }

    grid.iter().fold(0, |sum, row| {
        sum + row
            .iter()
            .filter(|&num_overlaps| *num_overlaps >= 2)
            .count()
    })
}

pub fn day05() {
    println!("\nDay 5:");
    let input =
        parse_lines(&std::fs::read_to_string("inputs/day05").expect("could not read input"));

    println!("Puzzle 1:");
    let p1_result = puzzle1(1000, 1000, &input);
    println!(
        "The number of points where at least two lines overlap is {}",
        p1_result
    );

    println!("Puzzle 2:");
    let p2_result = puzzle2(1000, 1000, &input);
    println!(
        "The number of points where at least two lines overlap (including diagonals) is {}",
        p2_result
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn puzzle1_example() {
        let input = parse_lines(&EXAMPLE_INPUT);
        let two_or_more_overlaps = puzzle1(10, 10, &input);
        assert_eq!(two_or_more_overlaps, 5);
    }

    #[test]
    fn puzzle2_example() {
        let input = parse_lines(&EXAMPLE_INPUT);
        let two_or_more_overlaps = puzzle2(10, 10, &input);
        assert_eq!(two_or_more_overlaps, 12);
    }
}
