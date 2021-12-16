macro_rules! DATA_PATH {
    () => {
        "../data/day11.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    println!("Answer of p1: {}", p1(raw));
    println!("Answer of p2: {}", p2(raw));
}

const FLASH_THRESHOLD: u8 = 10;

type Input = Vec<Vec<u8>>;
type Location = (usize, usize);

fn process(raw: &str) -> Input {
    let mut result = vec![];
    for line in raw.split('\n') {
        if line.is_empty() {
            continue;
        }
        let tokens: Vec<_> = line.chars().map(|c| c as u8 - b'0').collect();
        result.push(tokens);
    }
    result
}

fn get_neighbors(y: usize, x: usize, width: &usize, height: &usize) -> Vec<Location> {
    let result: Vec<_> = vec![
        (y.overflowing_sub(1).0, x),
        (y.overflowing_sub(1).0, x + 1),
        (y, x + 1),
        (y + 1, x + 1),
        (y + 1, x),
        (y + 1, x.overflowing_sub(1).0),
        (y, x.overflowing_sub(1).0),
        (y.overflowing_sub(1).0, x.overflowing_sub(1).0),
    ]
    .into_iter()
    .filter(|(y, x)| y < height && x < width)
    .collect();
    result
}

/// Simulate the octupus flashing, returns a sum of flash counts
fn simulate(input: &mut [Vec<u8>], width: &usize, height: &usize) -> usize {
    // first pass: increment energy level by one
    for line in input.iter_mut() {
        for level in line.iter_mut() {
            *level += 1;
        }
    }

    let mut flashed_count = 0;
    // second pass: process flashing
    loop {
        let last_flashed_count = flashed_count;
        for y in 0..*height {
            for x in 0..*width {
                let level = &mut input[y][x];
                if *level >= FLASH_THRESHOLD {
                    flashed_count += 1;
                    *level = 0;
                    for (y, x) in get_neighbors(y, x, width, height) {
                        let level = &mut input[y][x];
                        if *level > 0 {
                            *level += 1;
                        }
                    }
                }
            }
        }
        if last_flashed_count == flashed_count {
            break;
        }
    }
    flashed_count
}

fn p1(raw: &str) -> usize {
    let mut input = process(raw);
    let width = input[0].len();
    let height = input.len();

    let mut flashed_count = 0;
    for _ in 0..100 {
        flashed_count += simulate(&mut input, &width, &height);
    }
    flashed_count
}

fn p2(raw: &str) -> usize {
    let mut input = process(raw);
    let width = input[0].len();
    let height = input.len();
    let total = width * height;

    let mut step = 0;
    loop {
        step += 1;
        if simulate(&mut input, &width, &height) == total {
            break;
        }
    }
    step
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        assert_eq!(p1(raw), 1667);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        assert_eq!(p2(raw), 488);
    }
}
