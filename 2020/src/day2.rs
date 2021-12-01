use std::fs;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    matcher: char,
}

impl FromStr for Policy {
    type Err = ParseIntError;

    // 1-6 x
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut space_iter = s.rsplit(" ");
        let matcher = space_iter.next().unwrap().chars().next().unwrap();
        let mut hyphen_iter = space_iter.next().unwrap().split("-");
        let min = hyphen_iter.next().unwrap().parse::<usize>()?;
        let max = hyphen_iter.next().unwrap().parse::<usize>()?;
        Ok(Policy {
            min: min,
            max: max,
            matcher: matcher,
        })
    }
}

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day2.txt")?)?;
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

fn process(raw: &str) -> Vec<(Policy, &str)> {
    let mut result: Vec<(Policy, &str)> = vec![];
    for line in raw.split("\n") {
        if line.is_empty() {
            continue;
        }
        let mut split_iter = line.split(": ");
        let policy: Policy = split_iter.next().unwrap().parse().unwrap();
        let str = split_iter.next().unwrap();
        result.push((policy, &str));
    }
    result
}

fn p1(input: &Vec<(Policy, &str)>) -> usize {
    input
        .into_iter()
        .filter(|(policy, pw)| {
            let matched = pw.matches(policy.matcher).count();
            return matched >= policy.min && matched <= policy.max;
        })
        .count()
}

fn p2(input: &Vec<(Policy, &str)>) -> usize {
    input
        .into_iter()
        .filter(|(policy, pw)| {
            let matcher = policy.matcher;
            let mut chars = pw
                .chars()
                .skip(policy.min - 1)
                .step_by(policy.max - policy.min);
            return ((chars.next().unwrap() == matcher) as i32
                ^ (chars.next().unwrap() == matcher) as i32)
                != 0;
        })
        .count()
}
