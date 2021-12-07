use std::cmp;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day7.txt")?)?;
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

type Input = Vec<usize>;

fn process(raw: &str) -> Input {
    let mut result = vec![];
    for i in raw.split(',') {
        let int: usize = i.parse().unwrap();
        result.push(int)
    }
    result.sort();
    result
}

fn p1(input: &Input) -> usize {
    let median = {
        let len = input.len();
        if len % 2 == 0 {
            (input[len / 2] + input[len / 2 - 1]) / 2
        } else {
            input[len / 2]
        }
    };
    let distance_sum = input
        .iter()
        .map(|x| {
            if x > &median {
                x - &median
            } else {
                &median - x
            }
        })
        .fold(0, |acc, x| acc + x);
    distance_sum
}

fn distance_sum(input: &Input, mean: usize) -> usize {
    input
        .iter()
        .map(|x| {
            let euclid_distance = if x > &mean { x - &mean } else { &mean - x };
            euclid_distance * (euclid_distance + 1) / 2
        })
        .fold(0, |acc, x| acc + x)
}

fn p2(input: &Input) -> usize {
    let ceiling_mean = {
        let len = input.len();
        let sum = input.iter().fold(0, |acc, x| acc + x);
        sum / len
    };
    let distance_sum = cmp::min(
        distance_sum(input, ceiling_mean),
        distance_sum(input, ceiling_mean + 1),
    );
    distance_sum
}
