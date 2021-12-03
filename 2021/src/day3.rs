use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day3.txt")?)?;
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

fn process(raw: &str) -> Vec<Vec<usize>> {
    let mut result: Vec<_> = vec![];
    for n in raw.split("\n") {
        if n.is_empty() {
            continue;
        }
        result.push(
            n.chars()
                .map(|x| match x {
                    '0' => 0,
                    '1' => 1,
                    _ => unreachable!(),
                })
                .collect::<Vec<usize>>(),
        )
    }
    result
}

fn p1(input: &Vec<Vec<usize>>) -> usize {
    let height = input.len();
    let width = input[0].len();
    let mut mcb: Vec<usize> = vec![];
    for i in 0..width {
        let sum = input.iter().map(|x| x[i]).fold(0, |sum, x| sum + x);
        if sum > height / 2 {
            mcb.push(1)
        } else {
            mcb.push(0)
        }
    }
    let gamma_rate = usize::from_str_radix(
        &mcb.iter()
            .map(|x| match x {
                0 => '0',
                1 => '1',
                _ => unreachable!(),
            })
            .collect::<String>(),
        2,
    )
    .unwrap();
    let epsilon_rate = (1 << width) - 1 - gamma_rate;
    gamma_rate * epsilon_rate
}

fn get_nested_common_bits(input: &Vec<Vec<usize>>, mcb: bool) -> usize {
    let width = input[0].len();
    let mut bits: Vec<usize> = vec![];
    let mut filtered: Vec<&Vec<usize>> = input.into_iter().collect();
    for i in 0..width {
        let sum = filtered.iter().map(|x| x[i]).fold(0, |sum, x| sum + x);
        let filtered_len = filtered.len();
        // If 0 and 1 are equally common, keep values with a 1 in the position being considered.
        let mut bit = if sum + sum >= filtered_len { 1 } else { 0 };
        if mcb == false {
            bit ^= 1
        }
        // When there is only one element, the bit is both mcb/lcb
        if filtered_len == 1 {
            bit = filtered[0][i]
        }
        bits.push(bit);
        filtered = filtered.into_iter().filter(|x| x[i] == bit).collect();
    }
    usize::from_str_radix(
        &bits
            .iter()
            .map(|x| match x {
                0 => '0',
                1 => '1',
                _ => unreachable!(),
            })
            .collect::<String>(),
        2,
    )
    .unwrap()
}
fn p2(input: &Vec<Vec<usize>>) -> usize {
    let oxygen_generator_rating = get_nested_common_bits(input, true);
    let co2_scrubber_rating = get_nested_common_bits(input, false);

    oxygen_generator_rating * co2_scrubber_rating
}
