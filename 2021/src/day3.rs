macro_rules! DATA_PATH {
    () => {
        "../data/day3.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

fn process(raw: &str) -> Vec<Vec<char>> {
    raw.lines().map(|n| n.chars().collect()).collect()
}

fn p1(input: &[Vec<char>]) -> usize {
    let height = input.len();
    let width = input[0].len();
    let mut mcb: String = "".to_string();
    for i in 0..width {
        let sum = input.iter().filter(|x| x[i] == '1').count();
        if sum > height / 2 {
            mcb.push('1')
        } else {
            mcb.push('0')
        }
    }
    let gamma_rate = usize::from_str_radix(&mcb, 2).unwrap();
    let epsilon_rate = (1 << width) - 1 - gamma_rate;
    gamma_rate * epsilon_rate
}

fn get_nested_common_bits(input: &[Vec<char>], mcb: bool) -> usize {
    let width = input[0].len();
    let mut bits: String = "".to_string();
    let mut filtered: Vec<&Vec<char>> = input.iter().collect();
    for i in 0..width {
        let sum = filtered.iter().filter(|x| x[i] == '1').count();
        let filtered_len = filtered.len();
        // If 0 and 1 are equally common, keep values with a 1 in the position being considered.
        let selection = if mcb {
            sum + sum >= filtered_len
        } else {
            sum + sum < filtered_len
        };
        let mut bit = if selection { '1' } else { '0' };
        // When there is only one element, the bit is both mcb/lcb
        if filtered_len == 1 {
            bit = filtered[0][i]
        }
        bits.push(bit);
        filtered = filtered.into_iter().filter(|x| x[i] == bit).collect();
    }
    usize::from_str_radix(&bits, 2).unwrap()
}
fn p2(input: &[Vec<char>]) -> usize {
    let oxygen_generator_rating = get_nested_common_bits(input, true);
    let co2_scrubber_rating = get_nested_common_bits(input, false);

    oxygen_generator_rating * co2_scrubber_rating
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 741950);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 903810);
    }
}
