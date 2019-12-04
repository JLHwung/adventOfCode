use std::cmp;
use std::fs::File;
use std::i32;
use std::io::{self, prelude::*, BufReader};
use std::vec;

fn main() -> io::Result<()> {
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);

    let wires: vec::Vec<String> = reader
        .lines()
        .map(|x| String::from(x.unwrap().trim_end()))
        .collect();

    let w1 = wire_to_segments(&wires[0]);
    let w2 = wire_to_segments(&wires[1]);

    let intersections = find_intersections(&w1, &w2);
    let p = find_closest(&intersections);
    println!("{}", p[0].abs() + p[1].abs());
    Ok(())
}

type Point = [i32; 2];
type Segment = [[i32; 2]; 2];

fn is_vertical(seg: &Segment) -> bool {
    seg[0][0] == seg[1][0]
}

fn find_intersections(wire1: &Vec<Segment>, wire2: &Vec<Segment>) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    for s1 in wire1 {
        if is_vertical(&s1) {
            // vertical
            for s2 in wire2 {
                if is_vertical(&s2) {
                    if s1[0][0] != s2[0][0] {
                        continue;
                    } else {
                        if cmp::min(s1[0][1], s1[1][1]) > cmp::min(s2[0][1], s2[1][1])
                            || cmp::max(s1[0][1], s1[1][1]) < cmp::max(s2[0][1], s2[1][1])
                        {
                            continue;
                        }
                        panic!("You need to write more")
                    }
                } else {
                    if cmp::max(s2[0][0], s2[1][0]) < s1[0][0]
                        || cmp::min(s2[0][0], s2[1][0]) > s1[0][0]
                        || cmp::max(s1[0][1], s1[1][1]) < s2[0][1]
                        || cmp::min(s1[0][1], s1[1][1]) > s2[0][1]
                    {
                        continue;
                    }
                    result.push([s1[0][0], s2[0][1]])
                }
            }
        } else {
            for s2 in wire2 {
                if !is_vertical(&s2) {
                    if s1[0][1] != s2[0][1] {
                        continue;
                    } else {
                        if cmp::min(s1[0][0], s1[1][0]) > cmp::min(s2[0][0], s2[1][0])
                            || cmp::max(s1[0][0], s1[1][0]) < cmp::max(s2[0][0], s2[1][0])
                        {
                            continue;
                        }
                        panic!("You need to write more")
                    }
                } else {
                    if cmp::max(s1[0][0], s1[1][0]) < s2[0][0]
                        || cmp::min(s1[0][0], s1[1][0]) > s2[0][0]
                        || cmp::max(s2[0][1], s2[1][1]) < s1[0][1]
                        || cmp::min(s2[0][1], s2[1][1]) > s1[0][1]
                    {
                        continue;
                    }
                    result.push([s2[0][0], s1[0][1]])
                }
            }
        }
    }
    result
}

fn find_closest(points: &Vec<Point>) -> &Point {
    let mut min_distance = i32::MAX;
    let mut result = &[i32::MAX, i32::MAX];
    for p in points {
        let d = p[0].abs() + p[1].abs();
        if d < min_distance {
            result = p;
            min_distance = d;
        }
    }
    result
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
        if start[0] != 0 && start[0] != 0 {
            result.push([start.clone(), new_start.clone()]);
        }
        start = new_start;
    }
    return result;
}
