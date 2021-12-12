use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../inputs/day12");

#[derive(Default)]
struct Cave {
    is_small: bool,
    neighbours: HashSet<String>,
}

// because try_insert is a nightly feature
fn insert_or_update_with_neighbour(caves: &mut HashMap<String, Cave>, key: &str, neighbour: &str) {
    if !caves.contains_key(key) {
        caves.insert(
            key.to_string(),
            Cave {
                is_small: key.chars().next().unwrap().is_lowercase(),
                ..Default::default()
            },
        );
    }
    let cave = caves.get_mut(key).unwrap();
    cave.neighbours.insert(neighbour.to_string());
}

fn visit_cave(
    caves: &HashMap<String, Cave>,
    mut visited_caves: Vec<String>,
    name: &str,
) -> Vec<Vec<String>> {
    let cave = &caves[name];

    if cave.is_small && visited_caves.contains(&name.to_string()) {
        return Vec::new();
    }

    visited_caves.push(name.to_string());

    let mut completed_paths = Vec::new();

    if name == "end" {
        completed_paths.push(visited_caves);
        return completed_paths;
    }

    for neighbour in &cave.neighbours {
        completed_paths.append(&mut visit_cave(caves, visited_caves.clone(), neighbour));
    }

    completed_paths
    // return: vec of vec of visited caves? optional? none if no valid path?
}

fn puzzle1(input: &str) -> usize {
    // create a list of caves as nodes
    // every node has links to other nodes
    // every node has a status (big, small, already visited)
    // start at the start node
    // for every linked node:
    // go to that node
    // if small and already visited: stop, invalid path
    // else: repeat until end node reached
    // keep track of visited nodes on path

    let mut caves: HashMap<String, Cave> = HashMap::new();
    for line in input.lines() {
        let (start, end) = line.split_once('-').unwrap();
        insert_or_update_with_neighbour(&mut caves, start, end);
        insert_or_update_with_neighbour(&mut caves, end, start);
    }

    let completed_paths = visit_cave(&caves, Vec::new(), "start");

    completed_paths.len()
}

pub fn day12() {
    println!("\nDay 12:");

    println!("Puzzle 1:");
    let num_paths = puzzle1(&INPUT);
    println!("The number of paths is {}", num_paths);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const TEST_INPUT2: &str = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const TEST_INPUT3: &str = r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn puzzle1_example() {
        assert_eq!(puzzle1(&TEST_INPUT1), 10);
        assert_eq!(puzzle1(&TEST_INPUT2), 19);
        assert_eq!(puzzle1(&TEST_INPUT3), 226);
    }
}
