macro_rules! DATA_PATH {
    () => {
        "../data/day17.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

struct Input {
    x_min: i16,
    x_max: i16,
    y_min: i16,
    y_max: i16,
}
type Point = (i16, i16);

const START: Point = (0, 0);

fn process(raw: &str) -> Input {
    // target area: x=57..116, y=-198..-148
    let (x_range, y_range) = raw.lines().next().unwrap()[13..].split_once(", ").unwrap();
    let (x_min_text, x_max_text) = x_range[2..].split_once("..").unwrap();
    let (y_min_text, y_max_text) = y_range[2..].split_once("..").unwrap();
    Input {
        x_min: x_min_text.parse().unwrap(),
        x_max: x_max_text.parse().unwrap(),
        y_min: y_min_text.parse().unwrap(),
        y_max: y_max_text.parse().unwrap(),
    }
}

fn is_point_within(x: &i16, y: &i16, input: &Input) -> bool {
    (input.x_min..=input.x_max).contains(x) && (input.y_min..=input.y_max).contains(y)
}

/// Simulate the trajectory
///
/// returns Some(highest y) when it hits the target area, otherwise returns None
fn simulate(input: &Input, vx0: i16, vy0: i16) -> Option<i16> {
    let (mut x, mut y) = START;
    let mut y_max = y;
    let (mut vx, mut vy) = (vx0, vy0);
    loop {
        x += vx;
        y += vy;
        // y reaches maximum when vy = 0
        if vy == 0 {
            y_max = y;
        }
        if is_point_within(&x, &y, input) {
            return Some(y_max);
        }
        if y < input.y_min || x > input.x_max {
            return None;
        }
        vy -= 1;
        vx -= i16::signum(vx);
    }
}
fn p1(input: &Input) -> i16 {
    // Search vx in [0, x_max]
    // when vx > x_max, the trajectory passes x_max after the first step
    // regardless of vy
    (0..=input.x_max)
        .filter_map(|vx| {
            // Search vy in [0, -y_min]
            // when vy < 0, the hightest y is starting point 0
            // when vy > -y_min, it will overshoot the target area regardless of vx
            // because when it returns to (x, 0), the next step will be (x + vx, -vy)
            (0..=(-input.y_min))
                .filter_map(|vy| simulate(input, vx, vy))
                .max()
        })
        .max()
        .unwrap()
}

fn p2(input: &Input) -> usize {
    (0..=input.x_max)
        .map(|vx| {
            // Search vy in [y_min, -y_min]
            // when vy < y_min, the trajectory passes y_min after the first step
            (input.y_min..=(-input.y_min))
                .filter(|&vy| simulate(input, vx, vy).is_some())
                .count()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 19503);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 5200);
    }
}
