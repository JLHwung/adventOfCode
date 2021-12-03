use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day6.txt")?)?;
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

fn process(raw: &str) -> Vec<Vec<u32>> {
    let mut result: Vec<_> = vec![];
    for line in raw.split("\n\n") {
        if line.is_empty() {
            continue;
        }
        // "abc\nad" => vec![0x111, 0x1001]
        result.push(
            line.split("\n")
                .filter(|x| !x.is_empty())
                .map(|x| {
                    x.chars()
                        .fold(0, |acc, c| acc | (1 << ((c as u32 - 'a' as u32) + 1)))
                })
                .collect::<Vec<u32>>(),
        );
    }
    result
}

fn p1(input: &Vec<Vec<u32>>) -> u32 {
    input
        .iter()
        // Concat each answer with bitwise OR
        // abc(0x111) | ad(0x1001) => abcd (0x1111)
        .map(|card| card.iter().fold(0, |acc, x| acc | x).count_ones())
        .fold(0, |acc, x| acc + x)
}

fn p2(input: &Vec<Vec<u32>>) -> u32 {
    input
        .iter()
        // Concat each answer with bitwise AND
        // abc(0x111) & ad(0x1001) => a (0x1)
        .map(|card| card.iter().fold(!0, |acc, x| acc & x).count_ones())
        .fold(0, |acc, x| acc + x)
}
