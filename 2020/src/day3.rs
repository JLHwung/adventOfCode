use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = process();
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

fn process() -> Vec<Vec<u8>> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day3.txt").unwrap()).unwrap();
    let mut result: Vec<Vec<u8>> = vec![];
    for line in raw.split('\n') {
        if line.is_empty() {
            continue;
        }
        let vector: Vec<u8> = line
            .chars()
            .map(|c| match c {
                '#' => 1,
                '.' => 0,
                _ => unreachable!(),
            })
            .collect();
        result.push(vector);
    }
    result
}

fn traverse_trees(input: &Vec<Vec<u8>>, right: usize, down: usize) -> usize {
    let mut result: usize = 0;
    let width = input[0].len();
    let mut j = right;
    for i in (down..input.len()).step_by(down) {
        j = j % width;
        if input[i][j] == 1 {
            result += 1;
        }
        j += right;
    }
    result
}

fn p1(input: &Vec<Vec<u8>>) -> usize {
    traverse_trees(input, 3, 1)
}

fn p2(input: &Vec<Vec<u8>>) -> usize {
    let data = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    data.iter()
        .map(|(right, down)| traverse_trees(input, *right, *down))
        .fold(1, |acc, x| acc * x)
}
