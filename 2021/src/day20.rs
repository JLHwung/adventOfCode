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

type Number = usize;

type Point = [Number; 2];

type Algorithm = Vec<u8>;

// Delta array ordererd as
// 8 7 6
// 5 4 3
// 2 1 0
const DELTA: [[isize; 2]; 9] = [
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

const PADDING: Number = 1;
const ALGORITHM_SIZE: usize = 512;

// An image is a HashSet of lit pixels
type Image = Vec<Vec<u8>>;

struct Input {
    algorithm: Algorithm,
    image: Image,
}

fn process(raw: &str) -> Input {
    let (algorithm_text, image_rep) = raw.split_once("\n\n").unwrap();
    let algorithm = algorithm_text
        .chars()
        .filter_map(|ch| match ch {
            '#' => Some(1),
            '.' => Some(0),
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
    let image = image_rep
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '#' => 1,
                    '.' => 0,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    Input { algorithm, image }
}

fn enhanced_pixel_is_light(
    [x0, y0]: Point,
    image: &[Vec<u8>],
    algorithm: &[u8],
    width: usize,
    height: usize,
    border_is_light: bool,
) -> u8 {
    let mut index = 0;
    for (i, [dx, dy]) in DELTA.iter().enumerate() {
        let neighbor = [
            x0 as isize + dx - PADDING as isize,
            y0 as isize + dy - PADDING as isize,
        ];
        let [x, y] = neighbor;
        if x < 0 || x >= width as isize || y < 0 || y >= height as isize {
            if border_is_light {
                index |= 1 << i
            }
        } else if image[y as usize][x as usize] == 1 {
            index |= 1 << i
        }
    }
    algorithm[index]
}

fn enhance(image: &[Vec<u8>], algorithm: &[u8], border_is_light: &mut bool) -> Image {
    let height = image.len();
    let width = image[0].len();

    let mut output = vec![vec![0; width + 2 * PADDING]; height + 2 * PADDING];
    for (y, line) in output.iter_mut().enumerate() {
        for (x, pixel) in line.iter_mut().enumerate() {
            *pixel =
                enhanced_pixel_is_light([x, y], image, algorithm, width, height, *border_is_light);
        }
    }

    // flip border pixels accoriding to algorithms
    *border_is_light = (algorithm[0] == 1 && !*border_is_light)
        || (algorithm[ALGORITHM_SIZE - 1] == 1 && *border_is_light);
    output
}

fn count_enhanced_light_pixels(input: &Input, round: usize) -> usize {
    let mut border_is_light = false;
    let mut enhanced = input.image.clone();
    for _ in 0..round {
        enhanced = enhance(&enhanced, &input.algorithm, &mut border_is_light);
    }
    enhanced
        .iter()
        .flat_map(|line| line.iter())
        .filter(|&&ch| ch == 1)
        .count()
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
