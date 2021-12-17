macro_rules! DATA_PATH {
    () => {
        "../data/day1.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

fn process(raw: &str) -> Vec<u16> {
    raw.lines().map(|n| n.parse().unwrap()).collect()
}

fn p1(input: &[u16]) -> usize {
    (1..input.len())
        .into_iter()
        .filter(|&i| input[i] > input[i - 1])
        .count()
}

fn p2(input: &[u16]) -> usize {
    (3..input.len())
        .into_iter()
        .filter(|&i| input[i] > input[i - 3])
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 1162);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 1190);
    }
}
