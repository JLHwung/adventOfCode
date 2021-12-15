use std::cell::RefCell;
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

fn get_neighbors(p: &usize, width: &usize, height: &usize) -> Vec<usize> {
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

fn risk_top_left_top_right_bottom_right(
    input: &Input,
    width_scaled: usize,
    height_scaled: usize,
) -> usize {
    let Input {
        risk,
        width,
        height,
    } = input;
    let risk_top_left_right: usize = (1..width_scaled)
        .map(|p| get_node_risk(risk, p, width, height, &width_scaled) as usize)
        .sum();
    let risk_top_right_bottom: usize = (1..height_scaled)
        .map(|p| {
            get_node_risk(
                risk,
                (width_scaled - 1) + p * width_scaled,
                width,
                height,
                &width_scaled,
            ) as usize
        })
        .sum();
    risk_top_left_right + risk_top_right_bottom
}

/// Dial's algorithm with improved buckets size upper bound
fn shortest_path_from_top_left_to_bottom_right(input: &Input, map_scaling: &usize) -> usize {
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

    // Pre compute the risk of a path from top-left to bottom-right
    // Use the risk as the upper bound of buckets
    let bucket_size_upper_bound: usize = risk_top_left_top_right_bottom_right(input, width_scaled, height_scaled);

    let buckets = vec![RefCell::new(vec![]); bucket_size_upper_bound];
    let goal = width_scaled * height_scaled - 1;

    dist[0] = 0;
    buckets[0].borrow_mut().push(0);

    for cost in 0..buckets.len() {
        let nodes = buckets[cost].borrow();
        for position in nodes.iter() {
            if *position == goal {
                return cost;
            }

            // Important as we may have already found a better way
            if cost > dist[*position] {
                continue;
            }
            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for node in get_neighbors(position, &width_scaled, &height_scaled) {
                let new_cost =
                    cost + (get_node_risk(risk, node, width, height, &width_scaled) as usize);

                // If so, add it to the frontier and continue
                if new_cost < dist[node] {
                    buckets[new_cost].borrow_mut().push(node);
                    // Relaxation, we have now found a better way
                    dist[node] = new_cost;
                }
            }
        }
    }

    // Goal not reachable
    unreachable!();
}

fn p1(input: &Input) -> usize {
    shortest_path_from_top_left_to_bottom_right(input, &1)
}

fn p2(input: &Input) -> usize {
    shortest_path_from_top_left_to_bottom_right(input, &5)
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
