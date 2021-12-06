use std::fs;
use std::io;
use std::thread;

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

fn count_fish(timer: isize, days: isize) -> usize {
    if days <= timer {
        1
    } else {
        count_fish(6, days - timer - 1) + count_fish(8, days - timer - 1)
    }
}

fn sum_fish_count(input: &Vec<usize>, days: isize) -> usize {
    let cache = {
        let mut handlers = vec![];
        for i in 0..6 {
            handlers.push(thread::spawn(move || {
                count_fish(i.try_into().unwrap(), days)
            }))
        }
        let mut cache = vec![];
        for handler in handlers {
            cache.push(handler.join().unwrap());
        }
        cache
    };
    let mut sum = 0;
    for i in input {
        sum += cache[*i];
    }
    sum
}

fn p1(input: &Vec<usize>) -> usize {
    sum_fish_count(input, 80)
}

fn p2(input: &Vec<usize>) -> usize {
    sum_fish_count(input, 256)
}
