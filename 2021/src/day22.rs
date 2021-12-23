use std::num::ParseIntError;
use std::str::FromStr;

macro_rules! DATA_PATH {
    () => {
        "../data/day22.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

type Number = isize;
const DIMENSION: usize = 3;
type Point = [Number; DIMENSION];

#[derive(Clone)]
struct Cube {
    /// Assumption: start[0] < end[0], start[1] < end[1], start[2] < end[2]
    start: Point,
    end: Point,
}

impl Cube {
    fn intersect(self: &Cube, other: &Cube) -> Option<Self> {
        let mut intersect_cube = Cube {
            start: [0; 3],
            end: [0; 3],
        };
        for i in 0..DIMENSION {
            if let Some([start, end]) = interval_intersect(
                &[self.start[i], self.end[i]],
                &[other.start[i], other.end[i]],
            ) {
                intersect_cube.start[i] = start;
                intersect_cube.end[i] = end;
            } else {
                return None;
            }
        }
        Some(intersect_cube)
    }

    fn volume(self: &Cube) -> usize {
        (0..DIMENSION)
            .into_iter()
            .map(|i| (self.end[i] - self.start[i]) as usize + 1)
            .product()
    }
}

impl FromStr for Cube {
    type Err = ParseIntError;
    // example x=0..5,y=-1..12,z=-104834..13456
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cube = Self {
            start: [0; 3],
            end: [0; 3],
        };
        for (i, coordinate_text) in s.split(',').enumerate() {
            let (start_text, end_text) = coordinate_text[2..].split_once("..").unwrap();
            let (start, end) = (start_text.parse()?, end_text.parse()?);
            cube.start[i] = start;
            cube.end[i] = end;
        }
        Ok(cube)
    }
}

/// compute the intersect interval between left [a, b] and right [c, d]
fn interval_intersect(left: &[Number; 2], right: &[Number; 2]) -> Option<[Number; 2]> {
    if left[1] < right[0] || right[1] < left[0] {
        None
    } else {
        Some([left[0].max(right[0]), left[1].min(right[1])])
    }
}

struct Step {
    on: bool,
    cube: Cube,
}

/// Building block of the intersection forest.
///
/// An intersection forest is a list of intersection tree, which satisfies
/// 1. Every root node hosts a cube with 'on' = true
/// 2. For every node, the child node hosts a cube such that the cube is the
///    intersection between parent cube and _some_ cube applied later than the root
///
/// ## Example
///
/// Let's say we have four cuboids C1, C2, C3, C4.
/// For every pair, they share an intersection cube, except C3 and C4.
/// Assuming C1 is on, C2 is off, C3 is on, C4 is on, the intersection forest is
///
/// ```
/// C1 ─ C1 ∩ C2 ─ C1 ∩ C2 ∩ C3
///             └─ C1 ∩ C2 ∩ C4
///   └─ C1 ∩ C3
/// C3 ─ C3 ∩ C4
/// C4
/// ```
///
/// Note that the class labels are for demonstration purpose only, we don't track which
/// cube intersects which in the forest.
struct Node {
    cube: Cube,
    children: Vec<Node>,
}

/// Recursively apply intersection with a given cube to a tree
fn apply_intersection(node: &mut Node, cube: &Cube) {
    if let Some(intersect_cube) = node.cube.intersect(cube) {
        for child in node.children.iter_mut() {
            apply_intersection(child, cube);
        }
        node.children.push(Node {
            cube: intersect_cube,
            children: vec![],
        });
    }
}

fn build_intersection_forest(reboot_steps: &[&Step]) -> Vec<Node> {
    let mut result = vec![Node {
        cube: reboot_steps[0].cube.clone(),
        children: vec![],
    }];

    for &step in reboot_steps.iter().skip(1) {
        for node in result.iter_mut() {
            apply_intersection(node, &step.cube);
        }
        if step.on {
            result.push(Node {
                cube: step.cube.clone(),
                children: vec![],
            })
        }
    }
    result
}

fn count_enabled_grids(forest: &[Node]) -> usize {
    struct State<'a> {
        node: &'a Node,
        should_add: bool,
    }

    let mut sum = 0;
    let mut stack: Vec<_> = forest
        .iter()
        .map(|node| State {
            node,
            should_add: true,
        })
        .collect();
    while let Some(State { node, should_add }) = stack.pop() {
        if should_add {
            // A, A∩B∩C, A∩B∩C∩D∩E, ...
            sum += node.cube.volume();
        } else {
            // A∩B, A∩B∩C∩D, ...
            sum -= node.cube.volume();
        }

        for child in &node.children {
            stack.push(State {
                node: child,
                should_add: !should_add,
            });
        }
    }
    sum
}

type Input = Vec<Step>;

fn process(raw: &str) -> Input {
    let result: Vec<_> = raw
        .lines()
        .map(|line| {
            let (on_text, cuboid_text) = line.split_once(' ').unwrap();
            let on = match on_text {
                "on" => true,
                "off" => false,
                _ => unreachable!(),
            };
            Step {
                on,
                cube: cuboid_text.parse().unwrap(),
            }
        })
        .collect();
    // otherwise we have to discard the leading off instructions
    assert!(result[0].on);
    result
}

// L∞ norm
fn norm_linf(point: &Point) -> isize {
    point.iter().map(|x| x.abs()).max().unwrap()
}

fn p1(input: &[Step]) -> usize {
    const P1_BOUND: isize = 50;

    let p1_input: Vec<_> = input
        .iter()
        .filter(|&step| {
            norm_linf(&step.cube.start) <= P1_BOUND && norm_linf(&step.cube.end) <= P1_BOUND
        })
        .collect();
    count_enabled_grids(&build_intersection_forest(&p1_input))
}

fn p2(input: &[Step]) -> usize {
    let p2_input: Vec<_> = input.iter().collect();
    count_enabled_grids(&build_intersection_forest(&p2_input))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 650099);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 1254011191104293);
    }
}
