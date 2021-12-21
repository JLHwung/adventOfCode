use std::collections::HashSet;

macro_rules! DATA_PATH {
    () => {
        "../data/day20.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

type Number = i16;

type Point = [Number; 2];

type Algorithm = Vec<bool>;

const DELTA: [Point; 9] = [
    [1, 1],
    [0, 1],
    [-1, 1],
    [1, 0],
    [0, 0],
    [-1, 0],
    [1, -1],
    [0, -1],
    [-1, -1],
];

const PADDING: i16 = 2;

fn add(lhs: &Point, rhs: &Point) -> Point {
    [lhs[0] + rhs[0], lhs[1] + rhs[1]]
}

fn enhance_at_is_light(
    point: &Point,
    light_pixels: &HashSet<Point>,
    algorithm: &[bool],
) -> bool {
    let mut index = 0;
    for (i, delta) in DELTA.iter().enumerate() {
        let neighbor = add(point, delta);
        if light_pixels.contains(&neighbor) {
            index |= 1 << i
        }
    }
    algorithm[index]
}

fn enhance_twice_at_is_light(
    point: &Point,
    light_pixels: &HashSet<Point>,
    algorithm: &[bool],
) -> bool {
    let mut index = 0;
    for (i, delta) in DELTA.iter().enumerate() {
        if enhance_at_is_light(&add(point, delta), light_pixels, algorithm) {
            index |= 1 << i
        }
    }
    algorithm[index]
}

#[derive(Clone, Debug)]
struct Image {
    x_min: Number,
    x_max: Number,
    y_min: Number,
    y_max: Number,
    light_pixels: HashSet<Point>,
}

struct Input {
    algorithm: Algorithm,
    image: Image,
}

fn process(raw: &str) -> Input {
    let (algorithm_text, image_rep) = raw.split_once("\n\n").unwrap();
    let algorithm = algorithm_text
        .chars()
        .filter_map(|ch| match ch {
            '#' => Some(true),
            '.' => Some(false),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(algorithm.len(), 512);

    let mut light_pixels = HashSet::new();
    let (mut x_max, mut y_max) = (0, -1);
    for (j, line) in image_rep.lines().enumerate() {
        x_max = line.len() - 1;
        for (i, ch) in line.chars().enumerate() {
            if ch == '#' {
                light_pixels.insert([i as Number, j as Number]);
            }
        }
        y_max += 1;
    }

    Input {
        algorithm,
        image: Image {
            x_min: 0,
            x_max: x_max as Number,
            y_min: 0,
            y_max,
            light_pixels,
        },
    }
}

fn enhance_twice(image: &Image, algorithm: &[bool]) -> Image {
    let Image {
        mut x_min,
        mut x_max,
        mut y_min,
        mut y_max,
        light_pixels: orig_light_pixels,
    } = image;
    x_min -= PADDING;
    x_max += PADDING;
    y_min -= PADDING;
    y_max += PADDING;

    let mut light_pixels = HashSet::new();
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            if enhance_twice_at_is_light(&[x, y], orig_light_pixels, algorithm) {
                light_pixels.insert([x, y]);
            }
        }
    }
    Image {
        x_min,
        x_max,
        y_min,
        y_max,
        light_pixels,
    }
}

fn p1(input: &Input) -> usize {
    let enhanced = enhance_twice(&input.image, &input.algorithm);
    enhanced.light_pixels.len()
}

const TIME: usize = 50;

fn p2(input: &Input) -> usize {
    let mut enhanced = input.image.clone();
    for _ in 0..TIME / 2 {
        enhanced = enhance_twice(&enhanced, &input.algorithm);
    }
    enhanced.light_pixels.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 5306);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 17497);
    }
}
