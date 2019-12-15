use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::vec;
use std::str;

fn main() -> io::Result<()> {
    let file = File::open("./packages/day2/data/input.txt")?;
    let reader = BufReader::new(file);

    let values: vec::Vec<u32> = reader.split(b',').map(|x| {
        str::from_utf8(&(
            x.unwrap())
        ).unwrap()
            .trim_end()
            .parse().unwrap()
    }).collect();

    for noun in 0..99 {
        for verb in 0..99 {
            if intcode_interpreter(&values, noun, verb)[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                break;
            }
        }
    }

    Ok(())
}

fn intcode_interpreter(vec: &Vec<u32>, noun: u32, verb: u32) -> Vec<u32> {
    let mut values: Vec<u32> = vec.clone();
    /*
        non-sense
        Once you have a working computer, the first step is to restore the gravity assist program (your puzzle input) to the "1202 program alarm" state it had just before the last computer caught fire. To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2.
    */
    values[1] = noun;
    values[2] = verb;

    let mut cursor: usize = 0;
    while values[cursor] != 99 {
        let input1 = values[cursor + 1] as usize;
        let input2 = values[cursor + 2] as usize;
        let output = values[cursor + 3] as usize;
        match values[cursor] {
            1 => {
                values[output] = values[input1] + values[input2];
                cursor += 4;
            }
            2 => {
                values[output] = values[input1] * values[input2];
                cursor += 4;
            }
            _ => {
                panic!();
            }
        }
    }

    values
}
