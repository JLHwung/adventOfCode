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

    let mut stdin = [1].to_vec();
    let mut stdout = Vec::new();
    let diagnostic_code = intcode_interpreter(&memory, &mut stdin, &mut stdout);
    println!("{}", diagnostic_code);
    Ok(())
}

fn parse_opcode(opcode: i32) -> [i32; 4] {
    [
        opcode % 100,
        opcode % 1000 / 100,
        opcode % 10000 / 1000,
        opcode / 10000
    ]
}

fn resolve_value(memory: &Vec<i32>, value: i32, mode: i32) -> i32 {
    match mode {
        0 => {
            memory[value as usize]
        }
        1 => {
            value
        }
        _ => {
            panic!("ILLEGAL INSTRUCTION FORMAT!")
        }
    }
}

fn intcode_interpreter(mem: &Vec<i32>, stdin: &mut Vec<i32>, stdout: &mut Vec<i32>) -> i32 {
    let mut memory = mem.clone();

    let mut pc: usize = 0;
    loop {
        let parsed = parse_opcode(memory[pc]);
        match parsed[0] {
            1 | 2 => {
                let input1 = resolve_value(&memory, memory[pc + 1], parsed[1]);
                let input2= resolve_value(&memory, memory[pc + 2], parsed[2]);
                let operand = memory[pc + 3] as usize;
                match parsed[0] {
                    1 => {
                        memory[operand] = input1 + input2
                    }
                    2 => {
                        memory[operand] = input1 * input2
                    }
                    _ => {
                        unreachable!()
                    }
                }
                pc += 4;
            }
            3 => {
                let operand = memory[pc + 1] as usize;
                memory[operand] = stdin.pop().unwrap();
                pc += 2;
            }
            4 => {
                let operand = memory[pc + 1] as usize;
                stdout.push(memory[operand]);
                pc += 2;
            }
            99 => {
                break;
            }
            _ => {
                panic!("ILLEGAL OPCODE!");
            }
        }
    }

    *stdout.last().unwrap()
}
