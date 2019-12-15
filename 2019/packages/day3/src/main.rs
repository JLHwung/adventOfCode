use std::cmp;
use std::fs::File;
use std::i32;
use std::io::{self, prelude::*, BufReader};
use std::vec;

fn main() -> io::Result<()> {
    let file = File::open("./packages/day3/data/input.txt")?;
    let reader = BufReader::new(file);

    let wires: vec::Vec<String> = reader
        .lines()
        .map(|x| String::from(x.unwrap().trim_end()))
        .collect();

    let w1 = wire_to_segments(&wires[0]);
    let w2 = wire_to_segments(&wires[1]);

    let r = find_intersection_lowest_delay(&w1, &w2);
    println!("{}", r);
    Ok(())
}

type Segment = [[i32; 2]; 2];

fn is_vertical(seg: &Segment) -> bool {
    seg[0][0] == seg[1][0]
}

fn find_intersection_lowest_delay(wire1: &Vec<Segment>, wire2: &Vec<Segment>) -> i32 {
    let mut s1_cost = 0;
    let mut min_cost = i32::MAX;
    'outer: for s1 in wire1 {
        let mut s2_cost = 0;
        if is_vertical(&s1) {
            // vertical
            for s2 in wire2 {
                if is_vertical(&s2) {
                    if s1[0][0] != s2[0][0] {
                    } else {
                        if cmp::min(s1[0][1], s1[1][1]) > cmp::min(s2[0][1], s2[1][1])
                            || cmp::max(s1[0][1], s1[1][1]) < cmp::max(s2[0][1], s2[1][1])
                        {
                        } else {
                            panic!("You need to write more")
                        }
                    }
                } else {
                    if cmp::max(s2[0][0], s2[1][0]) < s1[0][0]
                        || cmp::min(s2[0][0], s2[1][0]) > s1[0][0]
                        || cmp::max(s1[0][1], s1[1][1]) < s2[0][1]
                        || cmp::min(s1[0][1], s1[1][1]) > s2[0][1]
                    {
                    } else {
                        let cost = s1_cost
                            + s2_cost
                            + (s1[0][1] - s2[0][1]).abs()
                            + (s2[0][0] - s1[0][0]).abs();
                        if cost < min_cost && cost != 0 {
                            min_cost = cost;
                        }
                        break;
                    }
                }
                s2_cost += cmp::max((s2[0][1] - s2[1][1]).abs(), (s2[0][0] - s2[1][0]).abs());
            }
        } else {
            for s2 in wire2 {
                if !is_vertical(&s2) {
                    if s1[0][1] != s2[0][1] {
                    } else {
                        if cmp::min(s1[0][0], s1[1][0]) > cmp::min(s2[0][0], s2[1][0])
                            || cmp::max(s1[0][0], s1[1][0]) < cmp::max(s2[0][0], s2[1][0])
                        {
                        } else {
                            panic!("You need to write more")
                        }
                    }
                } else {
                    if cmp::max(s1[0][0], s1[1][0]) < s2[0][0]
                        || cmp::min(s1[0][0], s1[1][0]) > s2[0][0]
                        || cmp::max(s2[0][1], s2[1][1]) < s1[0][1]
                        || cmp::min(s2[0][1], s2[1][1]) > s1[0][1]
                    {
                    } else {
                        let cost = s1_cost
                            + s2_cost
                            + (s1[0][1] - s2[0][1]).abs()
                            + (s2[0][0] - s1[0][0]).abs();
                        if cost < min_cost && cost != 0 {
                            min_cost = cost;
                        }
                        break;
                    }
                }
                s2_cost += cmp::max((s2[0][1] - s2[1][1]).abs(), (s2[0][0] - s2[1][0]).abs());
            }
        }
        s1_cost += cmp::max((s1[0][1] - s1[1][1]).abs(), (s1[0][0] - s1[1][0]).abs());
    }
    min_cost
}

fn wire_to_segments(wire: &str) -> Vec<Segment> {
    let mut start: [i32; 2] = [0, 0];
    let mut result: Vec<[[i32; 2]; 2]> = Vec::new();
    for seg in wire.split(",") {
        let direction: &str = &seg[0..1];
        let length: i32 = seg[1..].parse().unwrap();
        let mut new_start = start;
        match direction {
            "U" => {
                new_start[1] += length;
            }
            "D" => {
                new_start[1] -= length;
            }
            "L" => {
                new_start[0] -= length;
            }
            "R" => {
                new_start[0] += length;
            }
            _ => {
                panic!();
            }
        }
        result.push([start.clone(), new_start.clone()]);
        start = new_start;
    }
    return result;
}
