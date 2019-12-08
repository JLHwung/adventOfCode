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

    for p1 in 0..5 {
        for p2 in 0..5 {
            if p2 == p1 {
                continue;
            }
            for p3 in 0..5 {
                if [p1, p2].iter().any(|p| *p == p3) {
                    continue;
                }
                for p4 in 0..5 {
                    if [p1, p2, p3].iter().any(|p| *p == p4) {
                        continue;
                    }
                    for p5 in 0..5 {
                        if [p1, p2, p3, p4].iter().any(|p| *p == p5) {
                            continue;
                        }
                        let mut stdin = [0, p1].to_vec();
                        let mut output = intcode_interpreter(&memory, &mut stdin, &mut stdout);

                        stdin = [output, p2].to_vec();
                        output = intcode_interpreter(&memory, &mut stdin, &mut stdout);
                        stdin = [output, p3].to_vec();
                        output = intcode_interpreter(&memory, &mut stdin, &mut stdout);
                        stdin = [output, p4].to_vec();
                        output = intcode_interpreter(&memory, &mut stdin, &mut stdout);
                        stdin = [output, p5].to_vec();
                        output = intcode_interpreter(&memory, &mut stdin, &mut stdout);

                        if output > max_thruster_signal {
                            max_thruster_signal = output;
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

fn intcode_interpreter(mem: &Vec<i32>, stdin: &mut Vec<i32>, stdout: &mut Vec<i32>) -> i32 {
    let mut memory = mem.clone();

    let mut pc: usize = 0;
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

    *stdout.last().unwrap()
}
