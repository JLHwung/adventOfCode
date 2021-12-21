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

// Delta array ordererd as
// 8 7 6
// 5 4 3
// 2 1 0
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

const PADDING: i16 = 1;
const ALGORITHM_SIZE: usize = 512;

// An image is a HashSet of lit pixels
type Image = HashSet<Point>;

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
    assert_eq!(algorithm.len(), ALGORITHM_SIZE);

    let mut image = HashSet::new();
    for (j, line) in image_rep.lines().enumerate() {
        for (i, ch) in line.chars().enumerate() {
            if ch == '#' {
                image.insert([i as Number, j as Number]);
            }
        }
    }

    Input { algorithm, image }
}

fn enhanced_pixel_is_light(
    [x0, y0]: &Point,
    image: &Image,
    algorithm: &[bool],
    [x_min, y_min]: &Point,
    [x_max, y_max]: &Point,
    border_is_light: bool,
) -> bool {
    let mut index = 0;
    for (i, [dx, dy]) in DELTA.iter().enumerate() {
        let neighbor = [x0 + dx, y0 + dy];
        let [x, y] = &neighbor;
        if (border_is_light && (x < x_min || x > x_max || y < y_min || y > y_max))
            || image.contains(&neighbor)
        {
            index |= 1 << i
        }
    }
    algorithm[index]
}

fn enhance(image: &Image, algorithm: &[bool], border_is_light: &mut bool) -> Image {
    let (mut x_min, mut y_min, mut x_max, mut y_max) =
        (Number::MAX, Number::MAX, Number::MIN, Number::MIN);

    for p in image.iter() {
        x_min = Number::min(x_min, p[0]);
        y_min = Number::min(y_min, p[1]);
        x_max = Number::max(x_max, p[0]);
        y_max = Number::max(y_max, p[1]);
    }

    let mut output = Image::new();
    for x in x_min - PADDING..=x_max + PADDING {
        for y in y_min - PADDING..=y_max + PADDING {
            if enhanced_pixel_is_light(
                &[x, y],
                image,
                algorithm,
                &[x_min, y_min],
                &[x_max, y_max],
                *border_is_light,
            ) {
                output.insert([x, y]);
            }
        }
    }

    // flip border pixels accoriding to algorithms
    *border_is_light =
        (algorithm[0] && !*border_is_light) || (algorithm[ALGORITHM_SIZE - 1] && *border_is_light);
    output
}

fn count_enhanced_light_pixels(input: &Input, round: usize) -> usize {
    let mut border_is_light = false;
    let mut enhanced = input.image.clone();
    for _ in 0..round {
        enhanced = enhance(&enhanced, &input.algorithm, &mut border_is_light);
    }
    enhanced.len()
}

fn p1(input: &Input) -> usize {
    count_enhanced_light_pixels(input, 2)
}

fn p2(input: &Input) -> usize {
    count_enhanced_light_pixels(input, 50)
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
