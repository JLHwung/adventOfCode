use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;
use std::io;

const DATA_PATH: &str = "./data/day15.txt";

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize(&DATA_PATH)?)?;
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

const RISK_MAX: u8 = 9;

struct Input {
    risk: Vec<u8>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn process(raw: &str) -> Input {
    let mut width = 0;
    let mut height = 0;
    let mut risk = vec![];
    for line in raw.split('\n') {
        if line.is_empty() {
            break;
        }
        for ch in line.chars() {
            risk.push(ch as u8 - b'0');
        }
        if width == 0 {
            width = risk.len();
        }
        height += 1;
    }
    Input {
        risk,
        width,
        height,
    }
}

fn get_neighbors(p: usize, width: &usize, height: &usize) -> Vec<usize> {
    let (y, x) = (p / width, p % width);
    let result: Vec<_> = vec![
        (y.overflowing_sub(1).0, x),
        (y, x + 1),
        (y + 1, x),
        (y, x.overflowing_sub(1).0),
    ]
    .into_iter()
    .filter_map(|(y, x)| {
        if y < *height && x < *width {
            Some(y * height + x)
        } else {
            None
        }
    })
    .collect();
    result
}

fn get_node_risk(risk: &[u8], p: usize, width: &usize, height: &usize, width_scaled: &usize) -> u8 {
    let (y, x) = (p / width_scaled, p % width_scaled);
    let (y_chunk, y_orig) = (y / height, y % height);
    let (x_chunk, x_orig) = (x / width, x % width);
    (risk[y_orig * height + x_orig] - 1 + x_chunk as u8 + y_chunk as u8) % RISK_MAX + 1
}

fn shortest_path(input: &Input, map_scaling: &usize) -> Option<usize> {
    let Input {
        risk,
        width,
        height,
    } = input;
    let width_scaled = width * map_scaling;
    let height_scaled = height * map_scaling;
    let mut dist: Vec<_> = (0..width_scaled * height_scaled)
        .map(|_| usize::MAX)
        .collect();
    let mut heap = BinaryHeap::new();
    let goal = width_scaled * height_scaled - 1;

    dist[0] = 0;
    heap.push(State {
        cost: 0,
        position: 0,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for node in get_neighbors(position, &width_scaled, &height_scaled) {
            let next = State {
                cost: cost + (get_node_risk(risk, node, width, height, &width_scaled) as usize),
                position: node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn p1(input: &Input) -> usize {
    shortest_path(input, &1).unwrap()
}

fn p2(input: &Input) -> usize {
    shortest_path(input, &5).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() -> io::Result<()> {
        let raw = fs::read_to_string(fs::canonicalize(&DATA_PATH)?)?;
        let input = process(&raw);
        assert_eq!(p1(&input), 537);
        Ok(())
    }

    #[test]
    fn test_p2() -> io::Result<()> {
        let raw = fs::read_to_string(fs::canonicalize(&DATA_PATH)?)?;
        let input = process(&raw);
        assert_eq!(p2(&input), 2881);
        Ok(())
    }
}
