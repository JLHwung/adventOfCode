use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};
use std::ops::{Index, IndexMut};
use std::str;

fn main() -> io::Result<()> {
    let mut file = File::open("./packages/day13/data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    // hack
    content = content.replace(
        "1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1",
        "1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,3,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1",
    );

    let mut tape: Vec<i64> = content
        .split(',')
        .map(|x| x.to_string().trim_end().parse().unwrap())
        .collect();

    let mut memory = Memory::new(tape);
    let mut stdin = Vec::new();
    let mut stdout = Vec::new();
    let mut entry = 0;
    let mut rb_entry = 0;
    // play for free :D
    memory[0] = 2;
    let result = intcode_interpreter(
        &mut memory,
        &mut stdin,
        &mut stdout,
        &mut entry,
        &mut rb_entry,
    );

    let mut cursor = 0;
    let mut tiles = HashMap::new();
    while cursor < stdout.len() {
        let [x, y, tile_id] = [stdout[cursor], stdout[cursor + 1], stdout[cursor + 2]];
        let mut inserted = tiles.entry((x, y)).or_insert(0);
        *inserted = tile_id;
        cursor += 3;
    }

    let x_max = tiles.keys().map(|x| x.0).max().unwrap();
    let x_min = tiles.keys().map(|x| x.0).min().unwrap();
    let y_max = tiles.keys().map(|x| x.1).max().unwrap();
    let y_min = tiles.keys().map(|x| x.1).min().unwrap();

    let mut y = y_max;
    while y >= y_min {
        for x in x_min..x_max + 1 {
            if x == -1 {
                match tiles.get(&(x, y)) {
                    Some(tile) => {
                        print!("{}", tile);
                    }
                    None => {
                        print!(" ");
                    }
                }
                continue;
            }
            match tiles.get(&(x, y)) {
                Some(tile) => match tile {
                    0 => print!(" "),
                    1 => print!("W"),
                    2 => print!("B"),
                    3 => print!("-"),
                    4 => print!("O"),
                    _ => unreachable!(),
                },
                None => {
                    print!(" ");
                }
            }
        }
        y -= 1;
        println!();
    }

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

fn intcode_interpreter(
    memory: &mut Memory,
    stdin: &mut Vec<i64>,
    stdout: &mut Vec<i64>,
    entry: &mut u64,
    rb_entry: &mut i64,
) -> bool {
    let mut pc = *entry;
    // relative base starts at 0;
    let mut rb = *rb_entry;
    let mut time = 0;
    loop {
        time += 1;
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
                // leave joystick at 0
                write_value(memory, memory[pc + 1], mode1, rb, 0);
                pc += 2;
            }
            4 | 9 => {
                let input1 = read_value(&memory, memory[pc + 1], mode1, rb);
                match opcode {
                    // stdout
                    4 => {
                        stdout.push(input1);
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
        // The game does not halt, we manually break when time elapses for a long time
        if time > 1000000 {
            break;
        }
    }
    *entry = pc;
    *rb_entry = rb;
    return true;
}
