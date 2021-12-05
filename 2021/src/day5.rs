use std::fs;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day5.txt")?)?;
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
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
        let points: Vec<&str> = s.split(" -> ").collect();
        let mut start_iter = points[0].split(",");
        let mut end_iter = points[1].split(",");
        let mut start = (
            start_iter.next().unwrap().parse::<usize>()?,
            start_iter.next().unwrap().parse::<usize>()?,
        );
        let mut end = (
            end_iter.next().unwrap().parse::<usize>()?,
            end_iter.next().unwrap().parse::<usize>()?,
        );
        // invariant: start.0 <= end.0 || start.1 <= end.1
        let mut temp = start;
        if start.0 > end.0 {
            temp = start;
            start = end;
            end = temp;
        } else if start.1 > end.1 {
            temp = start;
            start = end;
            end = temp;
        }
        Ok(Self {
            start: start,
            end: end,
        })
    }
}

fn process(raw: &str) -> Vec<Line> {
    let mut result: Vec<_> = vec![];
    for n in raw.split("\n") {
        if n.is_empty() {
            continue;
        }
        let line: Line = n.parse().unwrap();
        result.push(line)
    }
    result
}

fn print_map(map: &Vec<i32>) {
    for i in 0..HEIGHT {
        let mut str = "".to_string();
        for j in 0..WIDTH {
            let point = map[j * HEIGHT + i];
            str.push(if point > 1 {
                'x'
            } else if point == 1 {
                '1'
            } else {
                '.'
            });
        }
        println!("{}", str);
    }
}

fn draw_and_sum(input: &Vec<Line>, consider_horizontal_vertical_only: bool) -> u32 {
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

fn p1(input: &Vec<Line>) -> u32 {
    draw_and_sum(input, true)
}

fn p2(input: &Vec<Line>) -> u32 {
    draw_and_sum(input, false)
}
