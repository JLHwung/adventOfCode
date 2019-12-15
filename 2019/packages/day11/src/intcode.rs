use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, Result};
use std::ops::{Index, IndexMut};

type Program = Vec<i64>;

pub struct Memory {
    program: Vec<i64>,
    data: HashMap<u64, i64>,
}

impl Memory {
    pub fn new(program: Vec<i64>) -> Self {
        Self {
            program,
            data: HashMap::new(),
        }
    }
}

impl Index<u64> for Memory {
    type Output = i64;
    fn index(&self, index: u64) -> &Self::Output {
        if index < (self.program.len() as u64) {
            &self.program[index as usize]
        } else {
            match self.data.get(&index) {
                Some(value) => value,
                None => &0,
            }
        }
    }
}

impl IndexMut<u64> for Memory {
    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        if index < (self.program.len() as u64) {
            &mut self.program[index as usize]
        } else {
            self.data.entry(index).or_insert(0)
        }
    }
}

fn parse_op(op: i64) -> [i64; 4] {
    [op % 100, op % 1000 / 100, op % 10000 / 1000, op / 10000]
}

fn read_value(memory: &Memory, pos: i64, mode: i64, rb: i64) -> i64 {
    match mode {
        0 => memory[pos as u64],
        1 => pos,
        2 => memory[(rb + pos) as u64],
        _ => panic!("ILLEGAL INSTRUCTION FORMAT!"),
    }
}

fn write_value(memory: &mut Memory, pos: i64, mode: i64, rb: i64, value: i64) {
    match mode {
        0 => memory[pos as u64] = value,
        2 => memory[(rb + pos) as u64] = value,
        _ => panic!("ILLEGAL INSTRUCTION FORMAT!"),
    }
}

pub fn interpreter(
    memory: &mut Memory,
    stdin: &mut Vec<i64>,
    stdout: &mut Vec<i64>,
    entry: &mut u64,
    rb_entry: &mut i64,
) -> bool {
    let mut pc = *entry;
    // relative base starts at 0;
    let mut rb = *rb_entry;
    loop {
        let [opcode, mode1, mode2, mode3] = parse_op(memory[pc]);
        match opcode {
            // add, mul
            1 | 2 | 7 | 8 => {
                let input1 = read_value(memory, memory[pc + 1], mode1, rb);
                let input2 = read_value(memory, memory[pc + 2], mode2, rb);
                let result: i64;
                match opcode {
                    1 => result = input1 + input2,
                    2 => result = input1 * input2,
                    7 => result = if input1 < input2 { 1 } else { 0 },
                    8 => result = if input1 == input2 { 1 } else { 0 },
                    _ => unreachable!(),
                }
                write_value(memory, memory[pc + 3], mode3, rb, result);
                pc += 4;
            }
            // stdin
            3 => {
                write_value(memory, memory[pc + 1], mode1, rb, stdin.pop().unwrap());
                pc += 2;
            }
            4 | 9 => {
                let input1 = read_value(&memory, memory[pc + 1], mode1, rb);
                match opcode {
                    // stdout
                    4 => {
                        stdout.push(input1);
                        // INTERRUPT when there are 2 output
                        if stdout.len() == 2 {
                            pc += 2;
                            *entry = pc;
                            *rb_entry = rb;
                            return false;
                        }
                    }
                    // adjust rb
                    9 => rb += input1,
                    _ => unreachable!(),
                }
                pc += 2;
            }
            // jnz, jz
            5 | 6 => {
                let input1 = read_value(&memory, memory[pc + 1], mode1, rb);
                let input2 = read_value(&memory, memory[pc + 2], mode2, rb);
                match opcode {
                    5 => {
                        if input1 != 0 {
                            pc = input2 as u64
                        } else {
                            pc += 3
                        }
                    }
                    6 => {
                        if input1 == 0 {
                            pc = input2 as u64
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
                panic!("ILLEGAL OPCODE: {}", opcode);
            }
        }
    }
    *entry = pc;
    *rb_entry = rb;
    return true;
}

pub fn read_file_as_program(path: &str) -> io::Result<Program> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content);

    let tape: Vec<i64> = content
        .trim_end()
        .split(',')
        .map(|x| x.to_string().trim_end().parse().unwrap())
        .collect();

    Ok(tape)
}
