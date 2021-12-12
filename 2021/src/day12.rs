use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;

const DATA_PATH: &str = "./data/day12.txt";

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize(&DATA_PATH)?)?;
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

/// The input graph is modeled by a HashMap from node to their neighbors
type Input<'a> = HashMap<&'a str, Vec<&'a str>>;

fn process(raw: &str) -> Input {
    let mut result: Input = HashMap::new();
    for line in raw.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut iter = line.split('-');
        let from = iter.next().unwrap();
        let to = iter.next().unwrap();
        let from_neighbors = result.entry(from).or_insert_with(Vec::new);
        from_neighbors.push(to);
        let to_neighbors = result.entry(to).or_insert_with(Vec::new);
        to_neighbors.push(from);
    }
    result
}

fn is_big_cave(name: &str) -> bool {
    !name.chars().next().unwrap().is_ascii_lowercase()
}

struct P1State<'a> {
    current_cave: &'a str,
    visited_small_caves: HashSet<&'a str>,
}

fn p1(input: &Input) -> usize {
    let initial_state = P1State {
        current_cave: "start",
        visited_small_caves: ["start"].into_iter().collect::<HashSet<_>>(),
    };
    let mut path_stack = vec![initial_state];
    let mut count = 0;
    while let Some(P1State {
        current_cave,
        visited_small_caves,
    }) = path_stack.pop()
    {
        if current_cave == "end" {
            count += 1;
        } else {
            let candidates = input.get(current_cave).unwrap().iter().filter_map(|&cave| {
                if is_big_cave(cave) {
                    Some(P1State {
                        current_cave: cave,
                        visited_small_caves: visited_small_caves.clone(),
                    })
                } else if !visited_small_caves.contains(cave) {
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

struct P2State<'a> {
    current_cave: &'a str,
    visited_small_caves: HashSet<&'a str>,
    has_visited_small_cave_twice: bool,
}

fn p2(input: &Input) -> usize {
    let initial_state = P2State {
        current_cave: "start",
        visited_small_caves: ["start"].into_iter().collect::<HashSet<_>>(),
        has_visited_small_cave_twice: false,
    };
    let mut path_stack = vec![initial_state];
    let mut count = 0;
    while let Some(P2State {
        current_cave,
        visited_small_caves,
        has_visited_small_cave_twice,
    }) = path_stack.pop()
    {
        if current_cave == "end" {
            count += 1;
        } else {
            let candidates = input.get(current_cave).unwrap().iter().filter_map(|&cave| {
                if is_big_cave(cave) {
                    Some(P2State {
                        current_cave: cave,
                        visited_small_caves: visited_small_caves.clone(),
                        has_visited_small_cave_twice,
                    })
                } else if cave == "start" {
                    None
                } else if !visited_small_caves.contains(cave) {
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
    fn test_p1() -> io::Result<()> {
        let raw = fs::read_to_string(fs::canonicalize(&DATA_PATH)?)?;
        let input = process(&raw);
        assert_eq!(p1(&input), 3856);
        Ok(())
    }

    #[test]
    fn test_p2() -> io::Result<()> {
        let raw = fs::read_to_string(fs::canonicalize(&DATA_PATH)?)?;
        let input = process(&raw);
        assert_eq!(p2(&input), 116692);
        Ok(())
    }
}
