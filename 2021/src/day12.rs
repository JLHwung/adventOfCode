use std::collections::{HashMap, HashSet};

macro_rules! DATA_PATH {
    () => {
        "../data/day12.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

/// Map cave label to u8
const START_CAVE_LABEL: u8 = 0;
const END_CAVE_LABEL: u8 = 1;

/// The input is modeled by a HashMap from small cave to their merged neighbors
/// For example, if we have an adjacent list [start-A, start-end, A-end]
/// and denote start by 0, A by 2, end by 1, we will forward A's children to A's parent
/// so the merged neighbor records are
/// HashMap {
///   0: HashMap { 1: 2 }
///   1: HashMap { 0: 2 }
/// }
type Input = HashMap<u8, HashMap<u8, usize>>;

fn process(raw: &str) -> Input {
    let mut labels = HashMap::from([("start", START_CAVE_LABEL), ("end", END_CAVE_LABEL)]);
    let mut raw_map = HashMap::new();
    let mut big_cave_ids = HashSet::new();
    let mut register_index = |a| {
        let index = labels.len() as u8;
        *labels.entry(a).or_insert_with(|| {
            if a.as_bytes()[0] <= b'Z' {
                big_cave_ids.insert(index);
            }
            index
        })
    };
    let mut register_edge =
        |from_id, to_id| raw_map.entry(from_id).or_insert_with(Vec::new).push(to_id);

    for line in raw.lines() {
        let (from, to) = line.split_once('-').unwrap();
        let (from_id, to_id) = (register_index(from), register_index(to));
        register_edge(from_id, to_id);
        register_edge(to_id, from_id);
    }

    // forward big cave's children (must be small caves) to its parent (must be small caves)
    raw_map
        .iter()
        .filter_map(|(key, children)| {
            if big_cave_ids.contains(key) {
                return None;
            }
            let mut children_count_map = HashMap::<u8, usize>::new();
            for child in children {
                if big_cave_ids.contains(child) {
                    let big_cave_children = raw_map.get(child).unwrap();
                    for child in big_cave_children.iter() {
                        let count = children_count_map.entry(*child).or_insert(0);
                        *count += 1;
                    }
                } else {
                    let count = children_count_map.entry(*child).or_insert(0);
                    *count += 1;
                }
            }
            Some((*key, children_count_map))
        })
        .collect()
}

struct P1State {
    current_cave: u8,
    // The distance to the END_CAVE
    distance: usize,
    // The number of valid paths from current_cave to END_CAVE,
    // when current_cave is END_CAVE, it is defined as 1
    sum: usize,
}

fn p1(input: &Input) -> usize {
    let initial_state = P1State {
        current_cave: END_CAVE_LABEL,
        distance: 0,
        sum: 1,
    };
    let mut solution_stack = vec![initial_state];
    let mut count = 0;
    // A memory for visited small caves in reversing order, visited[0] is always END_CAVE
    // its distance can not be greater than the number of small caves minus 1, otherwise
    // there must be two duplicate small caves
    let mut visited = vec![END_CAVE_LABEL; input.len() - 1];
    while let Some(P1State {
        current_cave,
        distance,
        sum,
    }) = solution_stack.pop()
    {
        visited[distance] = current_cave;
        for (prev_cave, connection_count) in &input[&current_cave] {
            match *prev_cave {
                START_CAVE_LABEL => count += connection_count * sum,
                END_CAVE_LABEL => {}
                cave => {
                    // invariant: visited[0] is always END_CAVE and thus must not
                    // contain other small cave. Here we can skip visited[0]
                    if !visited[1..=distance].contains(&cave) {
                        solution_stack.push(P1State {
                            current_cave: cave,
                            distance: distance + 1,
                            sum: sum * connection_count,
                        });
                    }
                }
            }
        }
    }
    count
}

// See P1State for definition of `distance` and `sum`
struct P2State {
    current_cave: u8,
    distance: usize,
    sum: usize,
    visited_cave_twice: bool,
}

fn p2(input: &Input) -> usize {
    let initial_state = P2State {
        current_cave: END_CAVE_LABEL,
        distance: 0,
        sum: 1,
        visited_cave_twice: false,
    };
    let mut solution_stack = vec![initial_state];
    let mut count = 0;
    // A memory for visited small caves in reversing order
    // its distance can not be greater than the number of small caves, otherwise
    // there must be more than one duplicate small caves
    let mut visited = vec![END_CAVE_LABEL; input.len()];
    while let Some(P2State {
        current_cave,
        distance,
        sum,
        visited_cave_twice,
    }) = solution_stack.pop()
    {
        visited[distance] = current_cave;
        for (prev_cave, connection_count) in &input[&current_cave] {
            match *prev_cave {
                START_CAVE_LABEL => count += connection_count * sum,
                END_CAVE_LABEL => {}
                cave => {
                    if !visited[1..=distance].contains(&cave) {
                        solution_stack.push(P2State {
                            current_cave: cave,
                            distance: distance + 1,
                            sum: sum * connection_count,
                            visited_cave_twice,
                        });
                    } else if !visited_cave_twice {
                        solution_stack.push(P2State {
                            current_cave: cave,
                            distance: distance + 1,
                            sum: sum * connection_count,
                            visited_cave_twice: true,
                        });
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 3856);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 116692);
    }
}
