use std::cell::Cell;
use std::collections::HashSet;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day4.txt")?)?;
    println!("Answer of p1: {}", p1(&raw));
    println!("Answer of p2: {}", p2(&raw));
    Ok(())
}

const WIDTH: usize = 5;
const HEIGHT: usize = 5;
const SIZE: usize = WIDTH * HEIGHT;
const MOVE_MAX: usize = 100;

type StateElement = (usize, Cell<bool>);

type State = Vec<StateElement>;

type Board<'a> = [&'a StateElement; SIZE];

#[derive(Debug)]
struct Game<'a> {
    moves: Vec<usize>,
    boards: Vec<Board<'a>>,
}
fn process<'a>(raw: &str, state: &'a State) -> Game<'a> {
    let mut line_iter = raw.split("\n\n");
    let moves: Vec<_> = line_iter
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut boards = vec![];
    for board_text in line_iter {
        let numbers: Vec<_> = board_text
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let mut board = [&(state[0]); WIDTH * HEIGHT];
        for i in 0..SIZE {
            board[i] = &(state[numbers[i]]);
        }
        boards.push(board);
    }
    Game {
        moves: moves,
        boards: boards,
    }
}

fn new_state() -> State {
    let mut state = vec![];
    for i in 0..MOVE_MAX {
        state.push((i, Cell::new(false)));
    }
    state
}

fn has_win(board: &Board) -> bool {
    // check columns
    for i in 0..WIDTH {
        let mut winned = true;
        for j in 0..HEIGHT {
            winned = winned && board[j * WIDTH + i].1.get();
            if winned == false {
                break;
            }
        }
        if winned {
            return true;
        }
    }

    // check rows
    for j in 0..HEIGHT {
        let mut winned = true;
        for i in 0..WIDTH {
            winned = winned && board[j * WIDTH + i].1.get();
            if winned == false {
                break;
            }
        }
        if winned {
            return true;
        }
    }

    false
}

fn sum_unmarked(board: &Board) -> usize {
    board
        .iter()
        .filter_map(|x| if !x.1.get() { Some(x.0) } else { None })
        .fold(0, |acc, x| acc + x)
}
fn p1(raw: &str) -> usize {
    let state = new_state();
    let input = process(&raw, &state);
    let boards = &input.boards;
    for op in input.moves {
        state[op].1.set(true);
        for board in boards {
            if has_win(board) {
                return sum_unmarked(board) * op;
            }
        }
    }
    0
}

fn p2(raw: &str) -> usize {
    let state = new_state();
    let input = process(&raw, &state);
    let boards = &input.boards;
    let mut winned = HashSet::<usize>::new();
    let mut last_winned_score: usize = 0;
    for op in input.moves {
        state[op].1.set(true);
        for (i, board) in boards.iter().enumerate() {
            if !winned.contains(&i) && has_win(board) {
                last_winned_score = sum_unmarked(board) * op;
                winned.insert(i);
            }
        }
    }
    last_winned_score
}
