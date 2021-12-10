use std::collections::HashSet;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day1/input.txt")?)?;
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

fn process(raw: &str) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for n in raw.split('\n') {
        if n.is_empty() {
            continue;
        }
        let int: i32 = n.parse().unwrap();
        result.push(int);
    }
    result
}

fn p1(input: &Vec<i32>) -> i32 {
    let mut diff_set = HashSet::new();
    for n in input {
        let diff = 2020 - *n;
        if diff_set.contains(n) {
            return diff * *n;
        }
        diff_set.insert(diff);
    }
    unreachable!()
}

fn p2(input: &Vec<i32>) -> i32 {
    let mut diff_set = HashSet::new();
    for n in input {
        let diff = 2020 - *n;
        for m in input {
            let two_sum = *m + *n;
            if diff_set.contains(&two_sum) {
                return (2020 - two_sum) * *m * *n;
            }
        }
        diff_set.insert(diff);
    }
    unreachable!()
}
