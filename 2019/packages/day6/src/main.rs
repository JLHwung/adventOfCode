use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./packages/day6/data/input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut map = HashMap::new();

    for line in &lines {
        let orbitee = line[0..3].to_string();
        let orbitor = line[4..7].to_string();
        map.insert(orbitor, orbitee);
    }

    let mut you_ancestors = find_ancestors(&map, &"YOU".to_string());
    you_ancestors.reverse();
    let mut san_ancestors = find_ancestors(&map, &"SAN".to_string());
    san_ancestors.reverse();

    println!(
        "{}",
        you_ancestors.len() + san_ancestors.len()
            - 2 * find_common_starts(&you_ancestors, &san_ancestors)
    );
    Ok(())
}

fn find_ancestors(map: &HashMap<String, String>, node: &String) -> Vec<String> {
    let mut p = map.get(node);
    let mut result = Vec::new();
    loop {
        match p {
            Some(parent) => {
                result.push(parent.to_string());
                p = map.get(parent);
            }
            None => {
                break;
            }
        }
    }
    result
}

fn find_common_starts(ancestor1: &Vec<String>, ancestor2: &Vec<String>) -> usize {
    let mut r = 0;
    for i in 0..cmp::min(ancestor1.len(), ancestor2.len()) {
        if ancestor1[i] == ancestor2[i] {
            r += 1;
        } else {
            break;
        }
    }
    r
}
