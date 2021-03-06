macro_rules! DATA_PATH {
    () => {
        "../data/day7.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

type Input = usize;

fn process(raw: &str) -> Vec<Input> {
    let mut result: Vec<_> = raw.split(',').map(|i| i.parse().unwrap()).collect();
    result.sort_unstable();
    result
}

fn p1(input: &[Input]) -> usize {
    let median = {
        let len = input.len();
        if len % 2 == 0 {
            (input[len / 2] + input[len / 2 - 1]) / 2
        } else {
            input[len / 2]
        }
    };
    let distance_sum = input
        .iter()
        .map(|x| if x > &median { x - median } else { median - x })
        .sum();
    distance_sum
}

fn distance_sum(input: &[Input], mean: usize) -> usize {
    input
        .iter()
        .map(|x| {
            let euclid_distance = if x > &mean { x - mean } else { mean - x };
            euclid_distance * (euclid_distance + 1) / 2
        })
        .sum()
}

fn p2(input: &[Input]) -> usize {
    let ceiling_mean = {
        let len = input.len();
        let sum: usize = input.iter().sum();
        sum / len
    };
    usize::min(
        distance_sum(input, ceiling_mean),
        distance_sum(input, ceiling_mean + 1),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 328262);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 90040997);
    }
}
