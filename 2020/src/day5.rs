use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day5.txt")?)?;
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

fn process(raw: &str) -> Vec<usize> {
    let mut result: Vec<_> = vec![];
    for line in raw.split("\n") {
        if line.is_empty() {
            continue;
        }
        let len = line.len();
        let row = usize::from_str_radix(
            &line[0..len - 3]
                .chars()
                .map(|x| match x {
                    'B' => '1',
                    'F' => '0',
                    _ => unreachable!(),
                })
                .collect::<String>(),
            2,
        )
        .unwrap();
        let column = usize::from_str_radix(
            &line[len - 3..len]
                .chars()
                .map(|x| match x {
                    'R' => '1',
                    'L' => '0',
                    _ => unreachable!(),
                })
                .collect::<String>(),
            2,
        )
        .unwrap();
        result.push(row * 8 + column);
    }
    result.sort();
    result
}

fn p1(input: &Vec<usize>) -> usize {
    input[input.len() - 1]
}

fn p2(input: &Vec<usize>) -> usize {
    for i in 0..input.len() - 1 {
        let seat = input[i];
        if input[i + 1] == seat + 2 {
            return seat + 1;
        }
    }
    unreachable!();
}
