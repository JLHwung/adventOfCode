use std::cmp::{max, min};
use std::collections::HashMap;

macro_rules! DATA_PATH {
    () => {
        "../data/day14.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

/// A pair is any adjacent pair, every char, except the end and start,
/// in polymer template belongs to exactly two pairs.
type Pair = (char, char);

struct Input {
    /// A virtural pair (end, start) used to counter the fact that
    /// the start and end can only belongs to one pair.
    /// In other words, if we concat the end pair with pairs, then every
    /// char in a polymer template belongs to exactly two pairs.
    end: Pair,
    /// Frequency of pairs
    pairs: HashMap<Pair, usize>,
    rules: HashMap<Pair, (Pair, Pair)>,
}

fn process(raw: &str) -> Input {
    let chunks: Vec<_> = raw.split("\n\n").collect();
    let mut pairs = HashMap::<Pair, usize>::new();
    let chars: Vec<_> = chunks[0].chars().collect();
    for i in 0..chars.len() - 1 {
        let count = pairs.entry((chars[i], chars[i + 1])).or_insert(0);
        *count += 1
    }
    let end = (chars[chars.len() - 1], chars[0]);

    let mut rules = HashMap::<Pair, (Pair, Pair)>::new();
    for line in chunks[1].split('\n') {
        if line.is_empty() {
            continue;
        }
        // MM -> N
        let chars: Vec<_> = line.chars().collect();
        rules.insert(
            (chars[0], chars[1]),
            ((chars[0], chars[6]), (chars[6], chars[1])),
        );
    }
    Input { end, pairs, rules }
}

fn simulate(
    pairs: &mut HashMap<Pair, usize>,
    rules: &HashMap<Pair, (Pair, Pair)>,
) -> HashMap<Pair, usize> {
    let mut new_pairs = HashMap::<Pair, usize>::new();
    // apply insertion rules
    for (from, (to0, to1)) in rules {
        if let Some(from_count) = pairs.get(from) {
            let to0_count = new_pairs.entry(*to0).or_insert(0);
            *to0_count += from_count;
            let to1_count = new_pairs.entry(*to1).or_insert(0);
            *to1_count += from_count;
            pairs.remove(from);
        }
    }
    // inherit remaning pairs
    for (pair, count) in pairs {
        new_pairs.insert(*pair, *count);
    }
    new_pairs
}

/// Compute char frequency from pair frequency and the end virtual pair
fn summarize(pairs: &HashMap<Pair, usize>, end: &Pair) -> HashMap<char, usize> {
    let mut result = HashMap::<char, usize>::new();
    for ((left, right), count) in pairs {
        let left_count = result.entry(*left).or_insert(0);
        *left_count += count;
        let right_count = result.entry(*right).or_insert(0);
        *right_count += count;
    }
    let (left, right) = end;
    let left_count = result.entry(*left).or_insert(0);
    *left_count += 1;
    let right_count = result.entry(*right).or_insert(0);
    *right_count += 1;
    for (_, count) in result.iter_mut() {
        *count /= 2;
    }
    result
}

fn stat(input: &Input, step: usize) -> usize {
    let mut pairs = input.pairs.clone();
    for _ in 0..step {
        pairs = simulate(&mut pairs, &input.rules);
    }
    let (mut min_count, mut max_count) = (usize::MAX, usize::MIN);
    for (_, v) in summarize(&pairs, &input.end) {
        min_count = min(min_count, v);
        max_count = max(max_count, v);
    }
    max_count - min_count
}

fn p1(input: &Input) -> usize {
    stat(input, 10)
}

fn p2(input: &Input) -> usize {
    stat(input, 40)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 2194);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 2360298895777);
    }
}
