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

    let mut memory = Memory::new(tape);

    let mut tiles: HashMap<Pos, char> = HashMap::new();
    let initial_pos = (0isize, 0isize);
    let mut current_tile = initial_pos;
    tiles.insert(initial_pos, '.');
    let mut current_direction = 0;
    let mut pc = 0u64;

    loop {
        let current_tile_color = tiles.entry(current_tile).or_insert('.');
        let mut stdin = Vec::new();
        stdin.push(if *current_tile_color == '.' { 0 } else { 1 });
        let mut stdout: Vec<i64> = Vec::new();
        let halted = intcode_interpreter(&mut memory, &mut stdin, &mut stdout, &mut pc);
        if halted == false {
            *current_tile_color = if stdout[0] == 0 { '.' } else { '#' };
            let direction_adjustment = stdout[1] as i32;
            current_direction = adjust_direction(current_direction, direction_adjustment);
            current_tile = march_to_next_tile(current_tile, current_direction);
        } else {
            assert_eq!(stdout.len(), 0);
            break;
        }
    }

    println!("{}", tiles.len());
    Ok(())
}

fn adjust_direction(current_direction: i32, adjustment: i32) -> i32 {
    let mut new_direction = current_direction;
    match adjustment {
        0 => new_direction -= 1,
        1 => new_direction += 1,
        _ => unreachable!(),
    }
    if new_direction < 0 {
        new_direction += 4;
    } else if new_direction >= 4 {
        new_direction -= 4;
    }
    new_direction
}

fn march_to_next_tile(current_tile: Pos, direction: i32) -> Pos {
    let mut next_tile = current_tile;
    match direction {
        0 => next_tile.1 += 1,
        1 => next_tile.0 += 1,
        2 => next_tile.1 -= 1,
        3 => next_tile.0 -= 1,
        _ => unreachable!(),
    }
    next_tile
}

type Pos = (isize, isize);

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
) -> bool {
    let mut pc = *entry;
    // relative base starts at 0;
    let mut rb = 0;
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
    return true;
}
