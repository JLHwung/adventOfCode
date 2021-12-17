use std::cell::RefCell;

macro_rules! DATA_PATH {
    () => {
        "../data/day15.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

const RISK_MAX: usize = 9;

struct Input {
    risk: Vec<u8>,
    width: usize,
    height: usize,
}

fn process(raw: &str) -> Input {
    let mut width = 0;
    let mut height = 0;
    let mut risk = vec![];
    for line in raw.lines() {
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

/// Dial's algorithm with improved buckets size upper bound
fn shortest_path_from_top_left_to_bottom_right(input: &Input, map_scaling: &usize) -> usize {
    let Input {
        risk,
        width,
        height,
    } = input;
    let width_scaled = width * map_scaling;
    let height_scaled = height * map_scaling;
    let size_scaled = width_scaled * height_scaled;

    let get_node_risk = |p: usize| -> usize {
        let (y, x) = (p / width_scaled, p % width_scaled);
        let (y_chunk, y_orig) = (y / height, y % height);
        let (x_chunk, x_orig) = (x / width, x % width);
        (risk[y_orig * height + x_orig] as usize - 1 + x_chunk + y_chunk) % RISK_MAX + 1
    };

    let get_neighbors = |p: usize| -> Vec<usize> {
        let (y, x) = (p / width_scaled, p % width_scaled);
        vec![
            (y.overflowing_sub(1).0, x),
            (y, x + 1),
            (y + 1, x),
            (y, x.overflowing_sub(1).0),
        ]
        .into_iter()
        .filter_map(|(y, x)| (y < height_scaled && x < width_scaled).then(|| y * height_scaled + x))
        .collect::<Vec<_>>()
    };

    // Pre compute the risk of a path from top-left to bottom-right
    // Use the risk as the upper bound of buckets
    let bucket_size_upper_bound: usize = {
        let risk_top_left_right: usize = (1..width_scaled).map(get_node_risk).sum();
        let risk_top_right_bottom: usize = (2 * width_scaled - 1..size_scaled)
            .step_by(width_scaled)
            .map(get_node_risk)
            .sum();
        risk_top_left_right + risk_top_right_bottom
    };

    let mut dist = vec![bucket_size_upper_bound; size_scaled];
    let buckets = vec![RefCell::new(vec![]); bucket_size_upper_bound];
    let goal = size_scaled - 1;

    dist[0] = 0;
    buckets[0].borrow_mut().push(0);

    for cost in 0..buckets.len() {
        let nodes = buckets[cost].borrow();
        for node in nodes.iter() {
            if *node == goal {
                return cost;
            }

            // Important as we may have already found a better way
            if cost > dist[*node] {
                continue;
            }
            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for neighbor in get_neighbors(*node) {
                let new_cost = cost + get_node_risk(neighbor);

                // If so, add it to the frontier and continue
                if new_cost < dist[neighbor] {
                    buckets[new_cost].borrow_mut().push(neighbor);
                    // Relaxation, we have now found a better way
                    dist[neighbor] = new_cost;
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
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 537);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 2881);
    }
}
