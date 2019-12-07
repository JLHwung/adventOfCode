use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut map = HashMap::new();

    let mut last_sum = 0;
    loop {
        for line in &lines {
            let orbitee = line[0..3].to_string();
            let orbitor = line[4..7].to_string();
            let orbitee_depth: usize = *map.entry(orbitee).or_insert(0);
            map.insert(orbitor, orbitee_depth + 1);
        }
        let sum: usize = map.values().sum();
        if sum == last_sum {
            break;
        } else {
            last_sum = sum;
        }
    }

    println!("{}", last_sum);
    Ok(())
}

