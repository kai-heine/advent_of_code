const INPUT: &str = include_str!("../inputs/day09");

struct Location {
    height: i32,
    visited: bool,
}

struct Position {
    x: usize,
    y: usize,
}

fn read_input(input: &str) -> Vec<Vec<Location>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Location {
                    height: c.to_digit(10).unwrap() as i32,
                    visited: false,
                })
                .collect()
        })
        .collect()
}

fn find_low_points(heightmap: &[Vec<Location>]) -> Vec<Position> {
    let mut low_points: Vec<Position> = Vec::new();

    let map_width = heightmap[0].len();
    let map_height = heightmap.len();

    for y in 0..map_height {
        for x in 0..map_width {
            let height = heightmap[y][x].height;
            let is_low_point = (x == 0 || height < heightmap[y][x - 1].height)
                && (y == 0 || height < heightmap[y - 1][x].height)
                && (x == (map_width - 1) || height < heightmap[y][x + 1].height)
                && (y == (map_height - 1) || height < heightmap[y + 1][x].height);

            if is_low_point {
                low_points.push(Position { x, y });
            }
        }
    }

    low_points
}

fn puzzle1(input: &str) -> i32 {
    let heightmap = read_input(&input);
    let low_points = find_low_points(&heightmap);
    low_points
        .iter()
        .map(|pos| heightmap[pos.y][pos.x].height + 1)
        .sum()
}

fn visit_location(heightmap: &mut [Vec<Location>], pos: Position) -> usize {
    let mut locations_visited = 1;
    let mut adjacent_cells: Vec<Position> = Vec::new();
    heightmap[pos.y][pos.x].visited = true;

    if pos.x > 0 {
        adjacent_cells.push(Position {
            x: pos.x - 1,
            y: pos.y,
        });
    }
    if pos.x < heightmap[0].len() - 1 {
        adjacent_cells.push(Position {
            x: pos.x + 1,
            y: pos.y,
        });
    }
    if pos.y > 0 {
        adjacent_cells.push(Position {
            x: pos.x,
            y: pos.y - 1,
        });
    }
    if pos.y < heightmap.len() - 1 {
        adjacent_cells.push(Position {
            x: pos.x,
            y: pos.y + 1,
        });
    }

    for cell in adjacent_cells {
        if heightmap[cell.y][cell.x].height != 9 && !heightmap[cell.y][cell.x].visited {
            locations_visited += visit_location(heightmap, cell);
        }
    }

    return locations_visited;
}

fn puzzle2(input: &str) -> usize {
    let mut heightmap = read_input(&input);
    let low_points = find_low_points(&heightmap);
    let mut basin_sizes: Vec<usize> = Vec::new();

    for low_point in low_points {
        basin_sizes.push(visit_location(&mut heightmap, low_point));
    }

    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).product()
}

pub fn day09() {
    println!("\nDay 9:");

    println!("Puzzle 1:");
    let risk_level = puzzle1(&INPUT);
    println!("The sum of all risk levels is {}", risk_level);

    println!("Puzzle 2:");
    let result = puzzle2(&INPUT);
    println!(
        "The product of the sizes of the three largest basins is {}",
        result
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn puzzle1_example() {
        let risk_level = puzzle1(&TEST_INPUT);
        assert_eq!(risk_level, 15);
    }

    #[test]
    fn puzzle2_example() {
        let product = puzzle2(&TEST_INPUT);
        assert_eq!(product, 1134);
    }
}
