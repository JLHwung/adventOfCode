use std::str::FromStr;

macro_rules! DATA_PATH {
    () => {
        "../data/day24.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

fn process(raw: &str) -> Vec<Instruction> {
    let result: Vec<_> = raw.lines().map(|l| l.parse().unwrap()).collect();
    result
}

// The solver does not rely on the ALU, but it is fun to implement it
// plus we can double check the solver results
type Register = i32;
type Immediate = Register;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum RegisterLabel {
    W = 0,
    X = 1,
    Y = 2,
    Z = 3,
}

#[derive(Debug)]
struct ParseInstructionError {}

impl FromStr for RegisterLabel {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Self::W),
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "z" => Ok(Self::Z),
            _ => Err(Self::Err {}),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Inp(RegisterLabel),
    Mul(RegisterLabel, RegisterLabel),
    // Instructions with immediate
    Muli(RegisterLabel, Immediate),
    Add(RegisterLabel, RegisterLabel),
    Addi(RegisterLabel, Immediate),
    Divi(RegisterLabel, Immediate),
    Modi(RegisterLabel, Immediate),
    Eql(RegisterLabel, RegisterLabel),
    Eqli(RegisterLabel, Immediate),
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let texts: Vec<_> = s.split(' ').collect();
        let try_immediate = if texts.len() > 2 {
            texts[2].parse::<Immediate>().ok()
        } else {
            None
        };
        let operand_0: RegisterLabel = texts[1].parse()?;
        match texts[0] {
            "inp" => Ok(Self::Inp(operand_0)),
            "add" => {
                if let Some(immediate) = try_immediate {
                    Ok(Self::Addi(operand_0, immediate))
                } else {
                    Ok(Self::Add(operand_0, texts[2].parse()?))
                }
            }
            "mul" => {
                if let Some(immediate) = try_immediate {
                    Ok(Self::Muli(operand_0, immediate))
                } else {
                    Ok(Self::Mul(operand_0, texts[2].parse()?))
                }
            }
            "div" => {
                if let Some(immediate) = try_immediate {
                    Ok(Self::Divi(operand_0, immediate))
                } else {
                    unimplemented!();
                }
            }
            "mod" => {
                if let Some(immediate) = try_immediate {
                    Ok(Self::Modi(operand_0, immediate))
                } else {
                    unimplemented!();
                }
            }
            "eql" => {
                if let Some(immediate) = try_immediate {
                    Ok(Self::Eqli(operand_0, immediate))
                } else {
                    Ok(Self::Eql(operand_0, texts[2].parse()?))
                }
            }
            _ => Err(Self::Err {}),
        }
    }
}

fn evaluate(registers: &mut [Immediate; 4], instructions: &[Instruction], queue: &[Immediate]) {
    let mut stack: Vec<_> = queue.iter().rev().collect();
    for inst in instructions {
        match inst {
            Instruction::Inp(r1) => registers[*r1 as usize] = *stack.pop().unwrap(),
            Instruction::Add(r1, r2) => registers[*r1 as usize] += registers[*r2 as usize],
            Instruction::Addi(r1, imm) => registers[*r1 as usize] += imm,
            Instruction::Divi(r1, imm) => registers[*r1 as usize] /= imm,
            Instruction::Eql(r1, r2) => {
                registers[*r1 as usize] =
                    (registers[*r1 as usize] == registers[*r2 as usize]) as Immediate
            }
            Instruction::Eqli(r1, imm) => {
                registers[*r1 as usize] = (registers[*r1 as usize] == *imm) as Immediate
            }
            Instruction::Modi(r1, imm) => registers[*r1 as usize] %= imm,
            Instruction::Mul(r1, r2) => registers[*r1 as usize] *= registers[*r2 as usize],
            Instruction::Muli(r1, imm) => registers[*r1 as usize] *= imm,
        }
    }
}

/// The input ALU instructions can be splitted into 14 sections.
/// Each section is either an Increase transform or and Decrease transform
///
/// An increase transform with parameter a, b is
/// function increase(a, b, w) {
///   if (z % 26 + a != w) {
///     z = 26 * z + w + b
///   }
/// }
///
/// where a > 0 and b > 0. An increase transform may increases the degree of z on base 26.
///
/// For example, here is an increase transform with parameter a = 11 and b = 14.
/// ```
/// inp w
/// mul x 0
/// add x z
/// mod x 26
/// div z 1
/// add x 11
/// eql x w
/// eql x 0
/// mul y 0
/// add y 25
/// mul y x
/// add y 1
/// mul z y
/// mul y 0
/// add y w
/// add y 14
/// mul y x
/// add z y
/// ```
///
/// A decrease transform with parameter a, b is
/// function decrease(a, b, w) {
///   if (z % 26 + a != w) {
///     z = z / 26 * 26 + w + b
///   } else {
///     z = z / 26
///   }
/// }
///
/// where a < 0 and b > 0. A decrease transform may increases the degree of z on base 26.
///
/// For example, here is a decrease transform with parameter a = -9 and b = 7
///
/// ```
/// inp w
/// mul x 0
/// add x z
/// mod x 26
/// div z 26
/// add x -9
/// eql x w
/// eql x 0
/// mul y 0
/// add y 25
/// mul y x
/// add y 1
/// mul z y
/// mul y 0
/// add y w
/// add y 7
/// mul y x
/// add z y
/// ```
///
enum Transform {
    Increase(Immediate, Immediate),
    Decrease(Immediate, Immediate),
}

fn extract_transform_paremeters(input: &[Instruction]) -> Vec<Transform> {
    input
        .chunks(input.len() / DIGIT_LEN)
        .map(|instructions| {
            let a = match instructions[5] {
                Instruction::Addi(_, imm) => imm,
                _ => unreachable!(),
            };
            let b = match instructions[15] {
                Instruction::Addi(_, imm) => imm,
                _ => unreachable!(),
            };
            if a > 0 {
                Transform::Increase(a, b)
            } else {
                Transform::Decrease(a, b)
            }
        })
        .collect()
}

/// A digit begins from most significant to least significant
/// A constraint is a linear equation between digit[from_index] and
/// digit[to_index]:
///
/// digit[from_index] + offset = digit[to_index]
///
/// under the condition final z = 0.
/// For z = 0, each increase transform must be matched with the first
/// decrease transform. For example, if transform[8] is an increase transform,
/// transform[9] is a decrease transform, the following equation must holds:
///
/// w8 + b8 + a9 == w9
struct Constraint {
    from_index: usize,
    to_index: usize,
    offset: Immediate,
}

fn get_digit_constraints(transforms: &[Transform]) -> Vec<Constraint> {
    let mut results = vec![];
    let mut stack = vec![];
    for (index, transform) in transforms.iter().enumerate() {
        match transform {
            Transform::Increase(_, b) => {
                stack.push((index, b));
            }
            Transform::Decrease(a, _) => {
                let (last_increase_transform_index, parameter_b) = stack.pop().unwrap();
                results.push(Constraint {
                    from_index: last_increase_transform_index,
                    to_index: index,
                    offset: parameter_b + a,
                });
            }
        }
    }
    results
}

const DIGIT_LEN: usize = 14;
const DIGIT_MAX: Immediate = 9;
const DIGIT_MIN: Immediate = 1;

fn p1(input: &[Instruction]) -> String {
    let digit_constraints = get_digit_constraints(&extract_transform_paremeters(input));
    let mut digit = vec![0; DIGIT_LEN];
    for Constraint {
        from_index,
        to_index,
        offset,
    } in digit_constraints
    {
        digit[from_index] = if offset > 0 {
            DIGIT_MAX - offset
        } else {
            DIGIT_MAX
        };
        digit[to_index] = digit[from_index] + offset;
    }
    // Verify that MONAD does accept the digits
    let mut registers = [0; 4];
    evaluate(&mut registers, input, &digit);
    assert_eq!(registers[RegisterLabel::Z as usize], 0);
    digit.iter().map(|d| (*d as u8 + b'0') as char).collect()
}

fn p2(input: &[Instruction]) -> String {
    let digit_constraints = get_digit_constraints(&extract_transform_paremeters(input));
    let mut digit = vec![0; DIGIT_LEN];
    for Constraint {
        from_index,
        to_index,
        offset,
    } in digit_constraints
    {
        digit[from_index] = if offset > 0 {
            DIGIT_MIN
        } else {
            DIGIT_MIN - offset
        };
        digit[to_index] = digit[from_index] + offset;
    }
    // Verify that MONAD does accept the digits
    let mut registers = [0; 4];
    evaluate(&mut registers, input, &digit);
    assert_eq!(registers[RegisterLabel::Z as usize], 0);
    digit.iter().map(|d| (*d as u8 + b'0') as char).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), "45989929946199");
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), "11912814611156");
    }
}
