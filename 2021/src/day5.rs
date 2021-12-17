use std::num::ParseIntError;
use std::str::FromStr;

macro_rules! DATA_PATH {
    () => {
        "../data/day5.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

const HEIGHT: usize = 1000;
const WIDTH: usize = 1000;

type Point = (usize, usize);

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_text, end_text) = s.split_once(" -> ").unwrap();
        let (x_s, y_s) = start_text.split_once(',').unwrap();
        let (x_e, y_e) = end_text.split_once(',').unwrap();
        let mut start = (
            x_s.parse()?,
            y_s.parse()?,
        );
        let mut end = (
            x_e.parse()?,
            y_e.parse()?,
        );
        // invariant: start.0 <= end.0 || start.1 <= end.1
        let temp = start;
        if start.0 > end.0 || start.1 > end.1 {
            start = end;
            end = temp;
        }
        Ok(Self { start, end })
    }
}

fn process(raw: &str) -> Vec<Line> {
    raw.lines().map(|line| line.parse().unwrap()).collect()
}

fn draw_and_sum(input: &[Line], consider_horizontal_vertical_only: bool) -> u32 {
    let mut map = vec![0; HEIGHT * WIDTH];
    // draw lines
    for line in input {
        let (start, end) = (line.start, line.end);
        if start.0 == end.0 {
            for y in start.1..end.1 + 1 {
                map[start.0 * HEIGHT + y] += 1;
            }
        } else if start.1 == end.1 {
            for x in start.0..end.0 + 1 {
                map[x * HEIGHT + start.1] += 1;
            }
        } else if consider_horizontal_vertical_only {
            continue;
        } else if start.0 < end.0 {
            if start.1 < end.1 {
                for i in 0..end.0 - start.0 + 1 {
                    map[(start.0 + i) * HEIGHT + start.1 + i] += 1;
                }
            } else {
                for i in 0..end.0 - start.0 + 1 {
                    map[(start.0 + i) * HEIGHT + start.1 - i] += 1;
                }
            };
        } else if start.1 < end.1 {
            // start.0 > end.0
            for i in 0..end.1 - start.1 + 1 {
                map[(start.0 - i) * HEIGHT + start.1 + i] += 1;
            }
        } else {
            unreachable!();
        }
    }
    let mut sum = 0;
    for point in map {
        if point > 1 {
            sum += 1;
        }
    }
    sum
}

fn p1(input: &[Line]) -> u32 {
    draw_and_sum(input, true)
}

fn p2(input: &[Line]) -> u32 {
    draw_and_sum(input, false)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 5774);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 18423);
    }
}
