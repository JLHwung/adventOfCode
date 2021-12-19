macro_rules! DATA_PATH {
    () => {
        "../data/day18.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

const SN_SIZE: usize = 64;
const SPLIT_THRESHOLD: u16 = 10;

/// A Snailfish number is represented by a packed array
///
/// The array index encodes the branch information of number in a given pair.
/// Example
///
/// parse_sn("[1,2]")
/// returns an array with { 2: Some(1), 3: Some(2) } and other locations None
///
/// parse_sn("[9,[8,7]]")
/// { 2: Some(9), 6: Some(8), 7: Some(7) } and other locations None
///
/// Generally, the layout of SN is
///
/// | 0 1      | 2 3     | 4 .. 7  | 8 .. 15 | 16 .. 31 | 32 .. 63 |
/// | Not used | Depth 0 | Depth 1 | Depth 2 | Depth 3  | Depth 4  |
///
/// Location 0 and 1 are not used: they are always None
///
/// To get the parent slot of any given index, we can divide the index by 2
/// To check all need-to-be-exploded pairs, we can iterate slice 32 .. 63
///
type SN = [Option<u16>; SN_SIZE];

fn parse_sn(input: &str) -> SN {
    let mut sn = new_sn();
    let mut pos = 0;
    parse_pair(input, &mut pos, 1, &mut sn);
    sn
}

fn parse_pair(input: &str, pos: &mut usize, i: usize, sn: &mut SN) {
    let start = *pos;
    *pos += 1; // eat [ or number
    match &input[start..*pos] {
        "[" => {
            parse_pair(input, pos, i << 1, sn);
            *pos += 1; // eat ,
            parse_pair(input, pos, i << 1 | 1, sn);
            *pos += 1; // eat ]
        }
        c => sn[i] = Some(c.parse().unwrap()),
    };
}

fn new_sn() -> SN {
    [None; 64]
}

fn get_depth(i: usize) -> u32 {
    usize::BITS - i.leading_zeros() - 2
}

fn get_left_regular_number(sn: &SN, i: usize) -> Option<usize> {
    // search regular number in first-left candidates
    // we start by finding the last 1 in bit path i,
    // toggle it to 0, and then search by adding 1 after
    // 0b110100 -> 0b1100, 0b11001, 0b110011
    // 0b100000 -> 0b0 -> None
    let mut candidate = (i >> (i.trailing_zeros())) - 1;
    if candidate < 2 {
        None
    } else {
        while candidate < SN_SIZE {
            if sn[candidate].is_some() {
                return Some(candidate);
            }
            candidate = candidate * 2 + 1;
        }
        unreachable!();
    }
}

fn get_right_regular_number(sn: &SN, i: usize) -> Option<usize> {
    // search regular number in first-right candidates:
    // 0b111011 -> 0b1111, 0b11110, 0b111100
    // 0b111111 -> 0b1 -> None
    let mut candidate = (i >> (i.trailing_ones())) + 1;
    if candidate < 2 {
        None
    } else {
        while candidate < SN_SIZE {
            if sn[candidate].is_some() {
                return Some(candidate);
            }
            candidate *= 2;
        }
        unreachable!();
    }
}

/// Explode SN at given index i
fn explode_at(sn: &mut SN, i: usize) {
    let left = &sn[i].unwrap();
    let right = &sn[i + 1].unwrap();
    let parent = i / 2;
    // replace pair to 0
    sn[parent] = Some(0);
    let result = (
        get_left_regular_number(sn, i),
        get_right_regular_number(sn, i + 1),
    );
    if result.0.is_some() {
        let left_i = result.0.unwrap();
        sn[left_i] = Some(sn[left_i].unwrap() + left);
    }
    if result.1.is_some() {
        let right_i = result.1.unwrap();
        sn[right_i] = Some(sn[right_i].unwrap() + right);
    }
    // purge the pair
    sn[i] = None;
    sn[i + 1] = None;
}

/// Get the leftmost split index
fn get_first_split(sn: &SN) -> Option<usize> {
    let mut state = (SN_SIZE / 2, SN_SIZE / 2);
    for (i, value_option) in sn.iter().enumerate().take(SN_SIZE / 2).skip(2) {
        if let Some(v) = value_option {
            if *v >= SPLIT_THRESHOLD {
                let depth = get_depth(i);
                // Virtual left is the index of the left-most depth-3 child of given index
                // (should it existed)
                let left_most_depth_3_index = i << (3 - depth);
                if left_most_depth_3_index < state.1 {
                    state.0 = i;
                    state.1 = left_most_depth_3_index;
                }
            }
        }
    }
    if state.1 < SN_SIZE / 2 {
        Some(state.0)
    } else {
        None
    }
}

fn add(left: &SN, right: &SN) -> SN {
    let mut sn = new_sn();
    // merge two SN
    for depth in 0..4 {
        sn[0b100 << depth..0b110 << depth].clone_from_slice(&left[0b10 << depth..0b100 << depth]);
        sn[0b110 << depth..0b1000 << depth].clone_from_slice(&right[0b10 << depth..0b100 << depth]);
    }
    // explode
    for i in (SN_SIZE / 2..SN_SIZE).step_by(2) {
        if sn[i].is_some() {
            explode_at(&mut sn, i);
        }
    }
    // split
    while let Some(i) = get_first_split(&sn) {
        let v = sn[i].unwrap();
        sn[i] = None;
        sn[2 * i] = Some(v / 2);
        sn[2 * i + 1] = Some(v - v / 2);
        if 2 * i >= SN_SIZE / 2 {
            explode_at(&mut sn, 2 * i);
        }
    }
    sn
}

fn magnitude(sn: &SN) -> u16 {
    let mut sum = 0;
    for (i, value_option) in sn.iter().enumerate() {
        if let Some(v) = value_option {
            let depth = get_depth(i);
            let right_counts = i.count_ones() - 1;
            let left_counts = depth + 1 - right_counts;
            sum += 3_u16.pow(left_counts) * 2_u16.pow(right_counts) * v
        }
    }
    sum
}

type Input = Vec<SN>;

fn process(raw: &str) -> Input {
    raw.lines().map(parse_sn).collect()
}

fn p1(input: &[SN]) -> u16 {
    let mut sum = input[0];
    for sn in input.iter().skip(1) {
        sum = add(&sum, sn)
    }
    magnitude(&sum)
}

fn p2(input: &[SN]) -> u16 {
    let mut max = u16::MIN;
    for x in input {
        for y in input {
            max = u16::max(max, magnitude(&add(x, y)))
        }
    }
    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 3987);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 4500);
    }

    fn debug_print(sn: &SN) -> String {
        debug_print_base(sn, 1)
    }

    fn debug_print_base(sn: &SN, bit_depth: usize) -> String {
        let mut result = "".to_owned();
        if let Some(v) = sn[bit_depth] {
            result += &format!("{}", v);
        } else {
            result += "[";
            result += &debug_print_base(sn, bit_depth * 2);
            result += ",";
            result += &debug_print_base(sn, bit_depth * 2 + 1);
            result += "]";
        }
        result
    }

    #[test]
    fn test_sum() {
        let input = [
            parse_sn("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"),
            parse_sn("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
        ];
        let mut sum = input[0];
        for i in 1..input.len() {
            sum = add(&sum, &input[i]);
        }
        assert_eq!(
            debug_print(&sum),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        );
    }

    #[test]
    fn test_sum_2() {
        let input = [
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ];
        let mut sum = parse_sn(input[0]);
        for i in 1..input.len() {
            sum = add(&sum, &parse_sn(input[i]));
        }
        assert_eq!(
            debug_print(&sum),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
    }
}
