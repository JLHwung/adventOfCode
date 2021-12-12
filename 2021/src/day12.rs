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

fn simulate<'a, F>(input: &'a Input, mut criteria: F) -> usize
where
    F: FnMut(&&'a str, &Vec<&'a str>) -> Option<Vec<&'a str>>,
{
    let path = vec!["start"];
    let mut path_stack = vec![path];
    let mut count = 0;
    while let Some(path) = path_stack.pop() {
        let current_cave = path[path.len() - 1];
        if current_cave == "end" {
            count += 1;
        } else {
            let candidates = input
                .get(current_cave)
                .unwrap()
                .iter()
                .filter_map(|cave| criteria(cave, &path));
            for path in candidates {
                path_stack.push(path);
            }
        }
    }
    count
}

fn p1(input: &Input) -> usize {
    simulate(input, |cave, path| {
        if is_big_cave(cave) || !path.contains(cave) {
            let mut new_path = path.clone();
            new_path.push(cave);
            Some(new_path)
        } else {
            None
        }
    })
}

fn path_has_only_single_cave(path: &Vec<&str>) -> bool {
    let small_caves: Vec<_> = path
        .into_iter()
        .clone()
        .filter_map(|&cave| if is_big_cave(cave) { None } else { Some(cave) })
        .collect();
    small_caves.len() == small_caves.iter().cloned().collect::<HashSet<&str>>().len()
}

fn p2(input: &Input) -> usize {
    simulate(input, |cave, path| {
        if is_big_cave(cave) || !path.contains(cave) {
            let mut new_path = path.clone();
            new_path.push(cave);
            Some(new_path)
        } else if *cave != "start" {
            if path_has_only_single_cave(path) {
                let mut new_path = path.clone();
                new_path.push(cave);
                Some(new_path)
            } else {
                None
            }
        } else {
            None
        }
    })
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
