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
    for n in raw.split("\n") {
        if n.is_empty() {
            continue;
        }
        let int: i32 = n.parse().unwrap();
        result.push(int)
    }
    result
}

fn p1(input: &Vec<i32>) -> i32 {
    let mut prev = i32::MAX;
    let mut result = 0;
    for n in input {
        if *n > prev {
            result += 1;
        }
        prev = *n;
    }
    result
}

fn p2(input: &Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..input.len() {
        if i >= 3 && input[i] > input[i - 3] {
            result += 1
        }
    }
    result
}
