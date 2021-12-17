use std::str::FromStr;

macro_rules! DATA_PATH {
    () => {
        "../data/day2.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
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
        let (op, val) = s.split_once(' ').unwrap();
        Ok(Move {
            value: val.parse().map_err(|_| ParseMoveError)?,
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
    raw.lines().map(|n| n.parse().unwrap()).collect()
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
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 1690020);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 1408487760);
    }
}
