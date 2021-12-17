use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

macro_rules! DATA_PATH {
    () => {
        "../data/day9.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

type Input<'a> = Vec<Vec<u8>>;
type Location = (usize, usize);

const INPUT_VALUE_MAX: u8 = 9;
const BASIN_SIZE_TOP_K: usize = 3;

fn process(raw: &str) -> Input {
    raw.lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect()
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

fn p1(input: &[Vec<u8>]) -> usize {
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

fn should_visit(input: &[Vec<u8>], visited: &HashSet<Location>, y: usize, x: usize) -> bool {
    input[y][x] < INPUT_VALUE_MAX && !visited.contains(&(y, x))
}

/// Traverse all connected locations whose value is less than INPUT_VALUE_MAX
///
/// A depth-first traversal of connected basin locations. The `basin_set` records all the
/// current-known basic locations. The function returns the size of the basin
fn traverse_basin(
    input: &[Vec<u8>],
    basin_set: &mut HashSet<Location>,
    y: usize,
    x: usize,
    width: usize,
    height: usize,
) -> usize {
    let mut size = 0;
    let mut stack: Vec<_> = vec![(y, x)];
    while let Some((y, x)) = stack.pop() {
        if should_visit(input, basin_set, y, x) {
            size += 1;
            basin_set.insert((y, x));
            let neighbors = get_neighbors(y, x, width, height);
            for (y, x) in neighbors {
                stack.push((y, x));
            }
        }
    }
    size
}

fn p2(input: &[Vec<u8>]) -> usize {
    let width = input[0].len();
    let height = input.len();
    let mut basin_set = HashSet::<Location>::with_capacity(width * height);
    let mut basin_size_heap = BinaryHeap::<Reverse<usize>>::with_capacity(BASIN_SIZE_TOP_K + 1);
    for y in 0..height {
        for x in 0..width {
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
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 532);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 1110780);
    }
}
