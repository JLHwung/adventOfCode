macro_rules! DATA_PATH {
    () => {
        "../data/day6.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

fn process(raw: &str) -> [usize; 9] {
    let mut frequency = [0; 9];
    for i in raw.chars() {
        match i {
            '0' => frequency[0] += 1,
            '1' => frequency[1] += 1,
            '2' => frequency[2] += 1,
            '3' => frequency[3] += 1,
            '4' => frequency[4] += 1,
            '5' => frequency[5] += 1,
            '6' => frequency[6] += 1,
            _ => {}
        }
    }
    frequency
}

fn count_fish(x: &[usize; 9], days: isize) -> [usize; 9] {
    if days == 0 {
        *x
    } else {
        count_fish(
            &[x[1], x[2], x[3], x[4], x[5], x[6], x[7] + x[0], x[8], x[0]],
            days - 1,
        )
    }
}

fn sum_fish_count(input: &[usize; 9], days: isize) -> usize {
    let mut sum = 0;
    for v in count_fish(input, days) {
        sum += v;
    }
    sum
}

fn p1(input: &[usize; 9]) -> usize {
    sum_fish_count(input, 80)
}

fn p2(input: &[usize; 9]) -> usize {
    sum_fish_count(input, 256)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 360761);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 1632779838045);
    }
}
