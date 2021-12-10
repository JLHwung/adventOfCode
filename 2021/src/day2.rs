use std::fs;
use std::io;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day2.txt")?)?;
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

#[derive(Debug)]
enum MoveOperation {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct Move {
    value: i32,
    op: MoveOperation,
}

#[derive(Debug, Clone)]
struct ParseMoveError;

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let op = iter.next().unwrap();
        let val = iter
            .next()
            .unwrap()
            .parse::<i32>()
            .map_err(|_| ParseMoveError)?;
        Ok(Move {
            value: val,
            op: match op {
                "forward" => MoveOperation::Forward,
                "down" => MoveOperation::Down,
                "up" => MoveOperation::Up,
                _ => unreachable!(),
            },
        })
    }
}

fn process(raw: &str) -> Vec<Move> {
    let mut result: Vec<Move> = vec![];
    for n in raw.split('\n') {
        if n.is_empty() {
            continue;
        }
        let mov: Move = n.parse().unwrap();
        result.push(mov)
    }
    result
}

fn p1(input: &[Move]) -> i32 {
    let mut hoz = 0;
    let mut depth = 0;
    for mov in input {
        let value = mov.value;
        match mov.op {
            MoveOperation::Down => depth += value,
            MoveOperation::Forward => hoz += value,
            MoveOperation::Up => depth -= value,
        }
    }
    hoz * depth
}

fn p2(input: &[Move]) -> i32 {
    let mut hoz = 0;
    let mut depth = 0;
    let mut aim = 0;
    for mov in input {
        let value = mov.value;
        match mov.op {
            MoveOperation::Down => aim += value,
            MoveOperation::Forward => {
                hoz += value;
                depth += aim * value
            }
            MoveOperation::Up => aim -= value,
        }
    }
    hoz * depth
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() -> io::Result<()> {
        let raw = fs::read_to_string(fs::canonicalize("./data/day2.txt")?)?;
        let input = process(&raw);
        assert_eq!(p1(&input), 1690020);
        Ok(())
    }

    #[test]
    fn test_p2() -> io::Result<()> {
        let raw = fs::read_to_string(fs::canonicalize("./data/day2.txt")?)?;
        let input = process(&raw);
        assert_eq!(p2(&input), 1408487760);
        Ok(())
    }
}
