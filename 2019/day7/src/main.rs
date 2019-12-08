use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;

fn main() -> io::Result<()> {
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);

    let memory: Vec<i32> = reader
        .split(b',')
        .map(|x| {
            str::from_utf8(&(x.unwrap()))
                .unwrap()
                .trim_end()
                .parse()
                .unwrap()
        })
        .collect();

    let mut stdout: Vec<i32> = Vec::new();
    let mut max_thruster_signal = std::i32::MIN;

    for p1 in 5..10 {
        for p2 in 5..10 {
            if p2 == p1 {
                continue;
            }
            for p3 in 5..10 {
                if [p1, p2].iter().any(|p| *p == p3) {
                    continue;
                }
                for p4 in 5..10 {
                    if [p1, p2, p3].iter().any(|p| *p == p4) {
                        continue;
                    }
                    for p5 in 5..10 {
                        if [p1, p2, p3, p4].iter().any(|p| *p == p5) {
                            continue;
                        }
                        let mut amp1_input = 0;
                        let amp5_output: i32;
                        let mut is_feedback = false;
                        let mut memories = [memory.clone(), memory.clone(), memory.clone(), memory.clone(), memory.clone()];
                        let mut pcs = [0,0,0,0,0];
                        loop {
                            let mut stdin = [amp1_input].to_vec();
                            if !is_feedback {
                                stdin.push(p1);
                            }
                            let mut output = intcode_interpreter(&mut memories[0], &mut stdin, &mut stdout, &mut pcs[0]).0;

                            stdin = [output].to_vec();
                            if !is_feedback {
                                stdin.push(p2);
                            }
                            output = intcode_interpreter(&mut memories[1], &mut stdin, &mut stdout, &mut pcs[1]).0;
                            stdin = [output].to_vec();
                            if !is_feedback {
                                stdin.push(p3);
                            }
                            output = intcode_interpreter(&mut memories[2], &mut stdin, &mut stdout, &mut pcs[2]).0;
                            stdin = [output].to_vec();
                            if !is_feedback {
                                stdin.push(p4);
                            }
                            output = intcode_interpreter(&mut memories[3], &mut stdin, &mut stdout, &mut pcs[3]).0;
                            stdin = [output].to_vec();
                            if !is_feedback {
                                stdin.push(p5);
                            }
                            let (output, finished) = intcode_interpreter(&mut memories[4], &mut stdin, &mut stdout, &mut pcs[4]);
                            is_feedback = true;
                            if finished == true {
                                amp5_output = output;
                                break;
                            } else {
                                amp1_input = output;
                            }
                        }

                        if amp5_output > max_thruster_signal {
                            max_thruster_signal = amp5_output;
                        }
                    }
                }
            }
        }
    }
    println!("{}", max_thruster_signal);
    Ok(())
}

fn parse_opcode(opcode: i32) -> [i32; 4] {
    [
        opcode % 100,
        opcode % 1000 / 100,
        opcode % 10000 / 1000,
        opcode / 10000,
    ]
}

fn resolve_value(memory: &Vec<i32>, value: i32, mode: i32) -> i32 {
    match mode {
        0 => memory[value as usize],
        1 => value,
        _ => panic!("ILLEGAL INSTRUCTION FORMAT!"),
    }
}

fn intcode_interpreter(memory: &mut Vec<i32>, stdin: &mut Vec<i32>, stdout: &mut Vec<i32>, entry: &mut usize) -> (i32, bool) {
    let mut pc = *entry;
    loop {
        let parsed = parse_opcode(memory[pc]);
        match parsed[0] {
            // add, mul
            1 | 2 | 7 | 8 => {
                let input1 = resolve_value(&memory, memory[pc + 1], parsed[1]);
                let input2 = resolve_value(&memory, memory[pc + 2], parsed[2]);
                let operand = memory[pc + 3] as usize;
                match parsed[0] {
                    1 => memory[operand] = input1 + input2,
                    2 => memory[operand] = input1 * input2,
                    7 => memory[operand] = if input1 < input2 { 1 } else { 0 },
                    8 => memory[operand] = if input1 == input2 { 1 } else { 0 },
                    _ => unreachable!(),
                }
                pc += 4;
            }
            // stdin
            3 => {
                let operand = memory[pc + 1] as usize;
                memory[operand] = stdin.pop().unwrap();
                pc += 2;
            }
            // stdout
            4 => {
                let operand = memory[pc + 1] as usize;
                stdout.push(memory[operand]);
                pc += 2;
                // interrupt when output is emitted
                *entry = pc;
                return (*stdout.last().unwrap(), false)
            }
            // jnz, jz
            5 | 6 => {
                let input1 = resolve_value(&memory, memory[pc + 1], parsed[1]);
                let input2 = resolve_value(&memory, memory[pc + 2], parsed[2]);
                match parsed[0] {
                    5 => {
                        if input1 != 0 {
                            pc = input2 as usize
                        } else {
                            pc += 3
                        }
                    }
                    6 => {
                        if input1 == 0 {
                            pc = input2 as usize
                        } else {
                            pc += 3
                        }
                    }
                    _ => unreachable!(),
                }
            }
            99 => {
                break;
            }
            _ => {
                panic!("ILLEGAL OPCODE: {}", parsed[0]);
            }
        }
    }

    *entry = pc;
    (*stdout.last().unwrap(), true)
}
