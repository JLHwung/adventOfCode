use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::{Index, IndexMut};
use std::str;

fn main() -> io::Result<()> {
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);

    let tape: Vec<i64> = reader
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
    let mut stdout: Vec<i64> = Vec::new();
    let mut memory = Memory::new(tape);

    intcode_interpreter(&mut memory, &mut stdin, &mut stdout);

    for output in &stdout {
        println!("{}", output);
    }
    assert_eq!(stdout.len(), 1);

    Ok(())
}

struct Memory {
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

fn parse_opcode(opcode: i64) -> [i64; 4] {
    [
        opcode % 100,
        opcode % 1000 / 100,
        opcode % 10000 / 1000,
        opcode / 10000,
    ]
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

fn intcode_interpreter(memory: &mut Memory, stdin: &mut Vec<i64>, stdout: &mut Vec<i64>) {
    let mut pc = 0;
    // relative base starts at 0;
    let mut rb = 0;
    loop {
        let parsed = parse_opcode(memory[pc]);
        match parsed[0] {
            // add, mul
            1 | 2 | 7 | 8 => {
                let input1 = read_value(memory, memory[pc + 1], parsed[1], rb);
                let input2 = read_value(memory, memory[pc + 2], parsed[2], rb);
                let output_pos = memory[pc + 3];
                let result: i64;
                match parsed[0] {
                    1 => result = input1 + input2,
                    2 => result = input1 * input2,
                    7 => result = if input1 < input2 { 1 } else { 0 },
                    8 => result = if input1 == input2 { 1 } else { 0 },
                    _ => unreachable!(),
                }
                write_value(memory, output_pos, parsed[3], rb, result);
                pc += 4;
            }
            // stdin
            3 => {
                let output_pos = memory[pc + 1];
                write_value(memory, output_pos, parsed[1], rb, stdin.pop().unwrap());
                pc += 2;
            }
            4 | 9 => {
                let input1 = read_value(&memory, memory[pc + 1], parsed[1], rb);
                match parsed[0] {
                    // stdout
                    4 => stdout.push(input1),
                    // adjust rb
                    9 => rb += input1,
                    _ => unreachable!(),
                }
                pc += 2;
            }
            // jnz, jz
            5 | 6 => {
                let input1 = read_value(&memory, memory[pc + 1], parsed[1], rb);
                let input2 = read_value(&memory, memory[pc + 2], parsed[2], rb);
                match parsed[0] {
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
                panic!("ILLEGAL OPCODE: {}", parsed[0]);
            }
        }
    }
}
