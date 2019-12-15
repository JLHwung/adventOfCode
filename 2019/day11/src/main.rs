use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;
use day11::intcode;

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

    let mut memory = intcode::Memory::new(tape);

    let mut tiles: HashMap<Pos, char> = HashMap::new();
    let initial_pos = (0isize, 0isize);
    let mut current_tile = initial_pos;
    tiles.insert(initial_pos, '#');
    let mut current_direction = 0;
    let mut pc = 0u64;
    let mut rb = 0i64;

    loop {
        let current_tile_color = tiles.entry(current_tile).or_insert('.');
        let mut stdin = Vec::new();
        stdin.push(if *current_tile_color == '.' { 0 } else { 1 });
        let mut stdout: Vec<i64> = Vec::new();
        let halted = intcode::interpreter(&mut memory, &mut stdin, &mut stdout, &mut pc, &mut rb);
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

    let x_max = tiles.keys().map(|x| x.0).max().unwrap();
    let x_min = tiles.keys().map(|x| x.0).min().unwrap();
    let y_max = tiles.keys().map(|x| x.1).max().unwrap();
    let y_min = tiles.keys().map(|x| x.1).min().unwrap();

    let mut y = y_max;
    while y >= y_min {
        for x in x_min..x_max + 1 {
            match tiles.get(&(x, y)) {
                Some(tile) => {
                    print!("{}", tile);
                }
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
