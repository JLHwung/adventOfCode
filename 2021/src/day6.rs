use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day6.txt")?)?;
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

fn process(raw: &str) -> Vec<usize> {
    let mut result: Vec<_> = vec![];
    for n in raw.split(",") {
        if n.is_empty() {
            continue;
        }
        let timer = n.parse().unwrap();
        result.push(timer)
    }
    result
}

fn count_fish(x: &[usize; 9], days: isize) -> [usize; 9] {
    if days == 0 {
        *x
    } else {
        count_fish(
            &[x[1], x[2], x[3], x[4], x[5], x[6], x[7] + x[0], x[8], x[0]],
            days - 1,
        )
    }
}

fn sum_fish_count(input: &Vec<usize>, days: isize) -> usize {
    let mut frequency = [0; 9];
    for i in input {
        match i {
            0 => frequency[0] += 1,
            1 => frequency[1] += 1,
            2 => frequency[2] += 1,
            3 => frequency[3] += 1,
            4 => frequency[4] += 1,
            5 => frequency[5] += 1,
            6 => frequency[6] += 1,
            _ => unreachable!(),
        }
    }
    let mut sum = 0;
    for v in count_fish(&frequency, days) {
        sum += v;
    }
    sum
}

fn p1(input: &Vec<usize>) -> usize {
    sum_fish_count(input, 80)
}

fn p2(input: &Vec<usize>) -> usize {
    sum_fish_count(input, 256)
}
