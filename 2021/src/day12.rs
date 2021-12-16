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

struct Input<'a> {
    labels: HashMap<&'a str, u8>,
    big_cave_ids: HashSet<u8>,
    /// The map is modeled by a HashMap from node to their neighbors
    map: HashMap<u8, Vec<u8>>,
}

const START_CAVE_LABEL: u8 = 0;
const END_CAVE_LABEL: u8 = 1;

fn process(raw: &str) -> Input {
    let mut labels = HashMap::from([("start", START_CAVE_LABEL), ("end", END_CAVE_LABEL)]);
    let mut map = HashMap::new();
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
        |from_id, to_id| map.entry(from_id).or_insert_with(Vec::new).push(to_id);

    for line in raw.split('\n') {
        if line.is_empty() {
            continue;
        }
        let (from, to) = line.split_once('-').unwrap();

        let (from_id, to_id) = (register_index(from), register_index(to));
        register_edge(from_id, to_id);
        register_edge(to_id, from_id);
    }
    Input {
        labels,
        big_cave_ids,
        map,
    }
}

struct P1State {
    current_cave: u8,
    visited_small_caves: HashSet<u8>,
}

fn p1(input: &Input) -> usize {
    let Input {
        map, big_cave_ids, ..
    } = input;
    let initial_state = P1State {
        current_cave: START_CAVE_LABEL,
        visited_small_caves: HashSet::from([START_CAVE_LABEL]),
    };
    let mut path_stack = vec![initial_state];
    let mut count = 0;
    while let Some(P1State {
        current_cave,
        visited_small_caves,
    }) = path_stack.pop()
    {
        if current_cave == END_CAVE_LABEL {
            count += 1;
        } else {
            let candidates = map.get(&current_cave).unwrap().iter().filter_map(|&cave| {
                if big_cave_ids.contains(&cave) {
                    Some(P1State {
                        current_cave: cave,
                        visited_small_caves: visited_small_caves.clone(),
                    })
                } else if !visited_small_caves.contains(&cave) {
                    let mut new_visited_small_caves = visited_small_caves.clone();
                    new_visited_small_caves.insert(cave);
                    Some(P1State {
                        current_cave: cave,
                        visited_small_caves: new_visited_small_caves,
                    })
                } else {
                    None
                }
            });
            for candidate in candidates {
                path_stack.push(candidate);
            }
        }
    }
    count
}

struct P2State {
    current_cave: u8,
    visited_small_caves: HashSet<u8>,
    has_visited_small_cave_twice: bool,
}

fn p2(input: &Input) -> usize {
    let initial_state = P2State {
        current_cave: START_CAVE_LABEL,
        visited_small_caves: HashSet::from([START_CAVE_LABEL]),
        has_visited_small_cave_twice: false,
    };
    let Input {
        map, big_cave_ids, ..
    } = input;
    let mut path_stack = vec![initial_state];
    let mut count = 0;
    while let Some(P2State {
        current_cave,
        visited_small_caves,
        has_visited_small_cave_twice,
    }) = path_stack.pop()
    {
        if current_cave == END_CAVE_LABEL {
            count += 1;
        } else {
            let candidates = map.get(&current_cave).unwrap().iter().filter_map(|&cave| {
                if big_cave_ids.contains(&cave) {
                    Some(P2State {
                        current_cave: cave,
                        visited_small_caves: visited_small_caves.clone(),
                        has_visited_small_cave_twice,
                    })
                } else if cave == START_CAVE_LABEL {
                    None
                } else if !visited_small_caves.contains(&cave) {
                    let mut new_visited_small_caves = visited_small_caves.clone();
                    new_visited_small_caves.insert(cave);
                    Some(P2State {
                        current_cave: cave,
                        visited_small_caves: new_visited_small_caves,
                        has_visited_small_cave_twice,
                    })
                } else if !has_visited_small_cave_twice {
                    Some(P2State {
                        current_cave: cave,
                        visited_small_caves: visited_small_caves.clone(),
                        has_visited_small_cave_twice: true,
                    })
                } else {
                    None
                }
            });
            for candidate in candidates {
                path_stack.push(candidate);
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
