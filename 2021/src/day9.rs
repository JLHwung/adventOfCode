use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day9.txt")?)?;
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

type Input = Vec<Vec<u8>>;
type Location = (usize, usize);

const INPUT_VALUE_MAX: u8 = 9;
const BASIN_SIZE_TOP_K: usize = 3;

fn process(raw: &str) -> Input {
    let mut result = vec![];
    for line in raw.split('\n') {
        if line.is_empty() {
            continue;
        }
        let line_vec: Vec<u8> = line.chars().map(|x| (x as u8 - '0' as u8)).collect();
        result.push(line_vec);
    }
    result
}

fn get_neighbors(y: usize, x: usize, width: usize, height: usize) -> Vec<Location> {
    let mut result = vec![];
    if x > 0 {
        result.push((y, x - 1)); // West
    }
    if x < width - 1 {
        result.push((y, x + 1)); // East
    }
    if y > 0 {
        result.push((y - 1, x)); // South
    }
    if y < height - 1 {
        result.push((y + 1, x)); // North
    }
    result
}

fn p1(input: &Input) -> usize {
    let mut sum: usize = 0;
    let width = input[0].len();
    let height = input.len();
    for y in 0..height {
        for x in 0..width {
            let risk = input[y][x] + 1;
            if get_neighbors(y, x, width, height)
                .iter()
                .all(|&(y, x)| input[y][x] >= risk)
            {
                sum += usize::from(risk);
            }
        }
    }
    sum
}

fn should_visit(input: &Input, visited: &HashSet<Location>, y: usize, x: usize) -> bool {
    input[y][x] < INPUT_VALUE_MAX && !visited.contains(&(y, x))
}

/// Traverse all connected locations whose value is less than INPUT_VALUE_MAX
///
/// A depth-first traversal of connected basin locations. The `basin_set` records all the
/// current-known basic locations. The function returns the size of the basin
fn traverse_basin(
    input: &Input,
    basin_set: &mut HashSet<Location>,
    y: usize,
    x: usize,
    width: usize,
    height: usize,
) -> usize {
    let mut size = 0;
    let mut queue: VecDeque<Location> = VecDeque::new();
    if should_visit(input, basin_set, y, x) {
        size += 1;
        queue.push_back((y, x));
        basin_set.insert((y, x));
        while let Some((y, x)) = queue.pop_front() {
            let neighbors = get_neighbors(y, x, width, height);
            for (y, x) in neighbors {
                if should_visit(input, basin_set, y, x) {
                    size += 1;
                    queue.push_back((y, x));
                    basin_set.insert((y, x));
                }
            }
        }
    }
    size
}

fn p2(input: &Input) -> usize {
    let width = input[0].len();
    let height = input.len();
    let mut basin_set = HashSet::<Location>::new();
    let mut basin_size_heap = BinaryHeap::<Reverse<usize>>::with_capacity(BASIN_SIZE_TOP_K + 1);
    for y in 0..height {
        for x in 0..width {
            if basin_set.contains(&(y, x)) {
                continue;
            }
            let basin_size = traverse_basin(input, &mut basin_set, y, x, width, height);
            // Store basin size to a min-max heap so that we can keep references to the top-3 largest basin
            if basin_size > 0 {
                basin_size_heap.push(Reverse(basin_size));
                if basin_size_heap.len() > BASIN_SIZE_TOP_K {
                    basin_size_heap.pop();
                }
            }
        }
    }
    basin_size_heap.into_iter().fold(1, |acc, x| acc * x.0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() -> io::Result<()> {
        let raw = fs::read_to_string(fs::canonicalize("./data/day9.txt")?)?;
        let input = process(&raw);
        assert_eq!(p1(&input), 532);
        Ok(())
    }

    #[test]
    fn test_p2() -> io::Result<()> {
        let raw = fs::read_to_string(fs::canonicalize("./data/day9.txt")?)?;
        let input = process(&raw);
        assert_eq!(p2(&input), 1110780);
        Ok(())
    }
}
