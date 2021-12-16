macro_rules! DATA_PATH {
    () => {
        "../data/day16.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

#[derive(Debug)]
enum PacketContent {
    Literal {
        value: usize,
    },
    Expression {
        operator: u16,
        arguments: Vec<Packet>,
    },
}

#[derive(Debug)]
struct Packet {
    version: u16,
    content: PacketContent,
}

type Input = Packet;

fn parse_packet(input: &str, pos: &mut usize) -> Packet {
    let advance_by = |pos: &mut usize, size: usize| -> &str {
        let start = *pos;
        *pos += size;
        &input[start..*pos]
    };

    let parse_fixed_length =
        |pos: &mut usize, size| -> u16 { u16::from_str_radix(advance_by(pos, size), 2).unwrap() };

    let parse_variable_length = |pos: &mut usize| -> usize {
        let mut result = 0;
        loop {
            let should_break = advance_by(pos, 1);
            result = result << 4 | parse_fixed_length(pos, 4) as usize;
            if should_break == "0" {
                break;
            }
        }
        result
    };

    let version = parse_fixed_length(pos, 3);
    let type_id = parse_fixed_length(pos, 3);
    let content = match type_id {
        4 => PacketContent::Literal {
            value: parse_variable_length(pos),
        },
        operator => {
            let length_type_id = advance_by(pos, 1);
            let arguments: Vec<_> = match length_type_id {
                "0" => {
                    let bit_length = parse_fixed_length(pos, 15) as usize;
                    let arguments_end = *pos + bit_length;
                    let mut arguments = vec![];
                    while *pos < arguments_end {
                        arguments.push(parse_packet(input, pos));
                    }
                    arguments
                }
                "1" => {
                    let packet_length = parse_fixed_length(pos, 11) as usize;
                    let mut arguments = Vec::with_capacity(packet_length);
                    for _ in 0..packet_length {
                        arguments.push(parse_packet(input, pos));
                    }
                    arguments
                }
                _ => unreachable!(),
            };
            PacketContent::Expression {
                operator,
                arguments,
            }
        }
    };
    Packet { version, content }
}

fn process(raw: &str) -> Input {
    let mut pos = 0;
    let bits: String = raw
        .chars()
        .map(|c| {
            let digit = match c {
                '0'..='9' => c as u8 - b'0',
                'A'..='F' => c as u8 - b'A' + 10,
                _ => unreachable!(),
            };
            format!("{:04b}", digit)
        })
        .collect();
    parse_packet(&bits, &mut pos)
}

fn p1(input: &Input) -> usize {
    let mut sum = 0;
    let mut packet_stack = vec![input];
    while let Some(packet) = packet_stack.pop() {
        sum += packet.version as usize;
        if let PacketContent::Expression {
            operator: _,
            arguments,
        } = &packet.content
        {
            for packet in arguments {
                packet_stack.push(packet);
            }
        }
    }
    sum
}

fn evaluate(input: &Input) -> usize {
    match &input.content {
        PacketContent::Literal { value } => *value as usize,
        PacketContent::Expression {
            operator,
            arguments,
        } => {
            let mut results_iter = arguments.iter().map(evaluate);
            match operator {
                0 => results_iter.sum(),
                1 => results_iter.product(),
                2 => results_iter.min().unwrap(),
                3 => results_iter.max().unwrap(),
                5 => (results_iter.next() > results_iter.next()) as usize,
                6 => (results_iter.next() < results_iter.next()) as usize,
                7 => (results_iter.next() == results_iter.next()) as usize,
                _ => unreachable!(),
            }
        }
    }
}
fn p2(input: &Input) -> usize {
    evaluate(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 977);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 101501020883);
    }
}
