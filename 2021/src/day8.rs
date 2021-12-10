use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day8.txt")?)?;
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

const SEGMENT_SIZE: usize = 7;

#[derive(Debug)]
struct Observation<'a> {
    signals: Vec<&'a str>,
    outputs: Vec<&'a str>,
}

type Input<'a> = Observation<'a>;

// A transform is a map from (offset to 'a') to a one-segment mask.usize
// for example, if a transform t maps 'a' to 'e', 'e' to 'a'
// then
// t[0] = 0b0010000, t[4] = 0b0000001

type Transform = Vec<u8>;

fn process(raw: &str) -> Vec<Input> {
    let mut result = vec![];
    for line in raw.split('\n') {
        let mut iter = line.split(" | ");
        let signals: Vec<_> = iter.next().unwrap().split(' ').collect();
        let outputs: Vec<_> = iter.next().unwrap().split(' ').collect();
        result.push(Observation { signals, outputs })
    }
    result
}

fn p1(input: &[Input]) -> usize {
    let mut sum = 0;
    for observation in input {
        sum += observation
            .outputs
            .iter()
            .filter(|x| x.len() != 5 && x.len() != 6)
            .count();
    }
    sum
}

fn decode_bits(input: u8) -> usize {
    match input {
        0b1110111 => 0,
        0b0100100 => 1,
        0b1011101 => 2,
        0b1101101 => 3,
        0b0101110 => 4,
        0b1101011 => 5,
        0b1111011 => 6,
        0b0100101 => 7,
        0b1111111 => 8,
        0b1101111 => 9,
        _ => unreachable!(),
    }
}

/// Pack a signal to bits.

fn pack_signal_to_bits(input: &str) -> u8 {
    let mut activated_segments = 0x0;
    for s in input.chars() {
        activated_segments |= 1 << (s as u8 - b'a');
    }
    activated_segments
}

fn pack_signal_to_bits_with_transform(input: &str, transform: &[u8]) -> usize {
    let mut activated_segments = 0x0;
    for s in input.chars() {
        activated_segments |= transform[(s as u8 - b'a') as usize]
    }
    decode_bits(activated_segments)
}

/// Decode output signal with a given transform
fn decode_output(input: &[&str], transform: &[u8]) -> usize {
    let outputs: Vec<_> = input
        .iter()
        .map(|&x| pack_signal_to_bits_with_transform(x, transform))
        .collect();
    outputs[0] * 1000 + outputs[1] * 100 + outputs[2] * 10 + outputs[3]
}

/// Compute the inverse transform from given signals
///
/// The inverse transform will map signals to a one-bit mask, see definition of Transform
fn compute_inverse_transform(signals: &[&str]) -> Transform {
    let signal_bits: Vec<_> = signals
        .iter()
        .map(|signal| pack_signal_to_bits(signal))
        .collect();

    // compute unique-length segment index
    let (digit_one_idx, digit_four_idx, digit_seven_idx, digit_eight_idx, len_6_indices) = {
        let mut digit_one_idx = 0;
        let mut digit_four_idx = 0;
        let mut digit_seven_idx = 0;
        let mut digit_eight_idx = 0;
        let mut len_6_indices: Vec<usize> = vec![];
        for (i, &signal) in signals.iter().enumerate() {
            match signal.len() {
                2 => digit_one_idx = i,
                3 => digit_seven_idx = i,
                4 => digit_four_idx = i,
                6 => len_6_indices.push(i),
                7 => digit_eight_idx = i,
                _ => {}
            }
        }
        (
            digit_one_idx,
            digit_four_idx,
            digit_seven_idx,
            digit_eight_idx,
            len_6_indices,
        )
    };

    // 'a' = (7 ^ 1)
    let result_a = (signal_bits[digit_seven_idx] ^ signal_bits[digit_one_idx]).trailing_zeros();

    // 'g' = (9 ^ (4 | 'a'))
    let (result_g, digit_nine_idx) = {
        // 9 = find element in signal_bits[len_6_indices] s.t. element ^ (4 | 'a') contains only one 1-bit
        let four_and_a = signal_bits[digit_four_idx] | (1 << result_a);
        let mut mask = 0;
        let mut idx = 0;
        for i in &len_6_indices {
            mask = signal_bits[*i] ^ four_and_a;
            if mask.count_ones() == 1 {
                idx = *i;
                break;
            }
        }
        (mask.trailing_zeros(), idx)
    };

    // 'e' = (8 ^ 9)
    let result_e = (signal_bits[digit_eight_idx] ^ signal_bits[digit_nine_idx]).trailing_zeros();

    // 'b' = (0 ^ (7 | 'e' | 'g'))
    let (result_b, digit_zero_idx) = {
        // 0 = find element in signal_bits[len_6_indices] s.t. element ^ (7 | 'e' | 'g') contains only one 1-bit
        let seven_and_e_and_g = signal_bits[digit_seven_idx] | (1 << result_e) | (1 << result_g);
        let mut mask = 0;
        let mut idx = 0;
        for i in &len_6_indices {
            mask = signal_bits[*i] ^ seven_and_e_and_g;
            if mask.count_ones() == 1 {
                idx = *i;
                break;
            }
        }
        (mask.trailing_zeros(), idx)
    };

    // 'd' = (8 ^ 0)
    let result_d = (signal_bits[digit_eight_idx] ^ signal_bits[digit_zero_idx]).trailing_zeros();

    // 6 = find element in len_6_indices s.t. element is neither digit_nine_idx nor digit_zero_idx
    let digit_six_idx = *(len_6_indices
        .iter()
        .find(|&x| *x != digit_nine_idx && *x != digit_zero_idx)
        .unwrap());
    // 'c' = (8 ^ 6)
    let result_c = (signal_bits[digit_eight_idx] ^ signal_bits[digit_six_idx]).trailing_zeros();
    // 'f' = (1 ^ 'c')
    let result_f = (signal_bits[digit_one_idx] ^ (1 << result_c)).trailing_zeros();

    let mut transform = vec![0; SEGMENT_SIZE];
    transform[result_a as usize] = 0x1;
    transform[result_b as usize] = 0x2;
    transform[result_c as usize] = 0x4;
    transform[result_d as usize] = 0x8;
    transform[result_e as usize] = 0x10;
    transform[result_f as usize] = 0x20;
    transform[result_g as usize] = 0x40;
    transform
}

fn p2(input: &[Input]) -> usize {
    let mut result = 0;
    for observation in input {
        let transform = compute_inverse_transform(&observation.signals);
        result += decode_output(&observation.outputs, &transform);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pack_signal_to_bits() {
        assert_eq!(pack_signal_to_bits("cf"), 0b0100100);
        assert_eq!(pack_signal_to_bits("fc"), 0b0100100);
    }

    #[test]
    fn test_compute_inverse_transform() {
        let signals: Vec<_> = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab"
            .split(' ')
            .collect();
        assert_eq!(
            compute_inverse_transform(&signals)[..],
            vec![
                0x4,  // 'a' => 'c'
                0x20, // 'b' => 'f'
                0x40, // 'c' => 'g'
                0x1,  // 'd' => 'a'
                0x2,  // 'e' => 'b'
                0x8,  // 'f' => 'd'
                0x10, // 'g' => 'e'
            ][..]
        )
    }

    #[test]
    fn test_p1() -> io::Result<()> {
        let raw = fs::read_to_string(fs::canonicalize("./data/day8.txt")?)?;
        let input = process(&raw);
        assert_eq!(p1(&input), 310);
        Ok(())
    }

    #[test]
    fn test_p2() -> io::Result<()> {
        let raw = fs::read_to_string(fs::canonicalize("./data/day8.txt")?)?;
        let input = process(&raw);
        assert_eq!(p2(&input), 915941);
        Ok(())
    }
}
