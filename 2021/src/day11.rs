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

fn process(raw: &str) -> Input {
    raw.lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect()
}

/// Simulate the octupus flashing, returns a sum of flash counts
fn simulate(input: &mut [Vec<u8>], width: &usize, height: &usize) -> usize {
    let get_neighbors = |y: usize, x: usize| {
        vec![
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
    };

    let mut flash_stack = vec![];
    // first pass: increment energy level by one
    // and build initial flash stack
    for (y, line) in input.iter_mut().enumerate() {
        for (x, level) in line.iter_mut().enumerate() {
            *level = (*level + 1) % FLASH_THRESHOLD;
            if *level == 0 {
                flash_stack.push((y, x));
            }
        }
    }

    let mut flashed_count = 0;
    // second pass: consume flash stack and regiser new
    // flashing octopus
    while let Some((y, x)) = flash_stack.pop() {
        flashed_count += 1;
        for (y, x) in get_neighbors(y, x) {
            let level = &mut input[y][x];
            if *level > 0 {
                *level = (*level + 1) % FLASH_THRESHOLD;
                if *level == 0 {
                    flash_stack.push((y, x));
                }
            }
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
