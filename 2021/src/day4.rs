use std::cell::Cell;
use std::collections::HashSet;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize("./data/day4.txt")?)?;
    let state = new_state();
    let input = process(&raw, &state);
    println!("Answer of p1: {}", p1(&input, &state));
    println!("Answer of p2: {}", p2(&input, &state));
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

fn reset_state(state: &State) {
    for i in 0..MOVE_MAX {
        state[i].1.set(false);
    }
}

fn has_win(board: &Board) -> bool {
    // check columns
    for i in 0..WIDTH {
        let mut won = true;
        for j in 0..HEIGHT {
            won = won && board[j * WIDTH + i].1.get();
            if won == false {
                break;
            }
        }
        if won {
            return true;
        }
    }

    // check rows
    for j in 0..HEIGHT {
        let mut won = true;
        for i in 0..WIDTH {
            won = won && board[j * WIDTH + i].1.get();
            if won == false {
                break;
            }
        }
        if won {
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
fn p1(input: &Game, state: &State) -> usize {
    let boards = &input.boards;
    for op in &input.moves {
        state[*op].1.set(true);
        for board in boards {
            if has_win(board) {
                return sum_unmarked(board) * op;
            }
        }
    }
    unreachable!();
}

fn p2(input: &Game, state: &State) -> usize {
    reset_state(state);
    let boards = &input.boards;
    let mut won = HashSet::<usize>::new();
    let mut last_won_score: usize = 0;
    for op in &input.moves {
        state[*op].1.set(true);
        for (i, board) in boards.iter().enumerate() {
            if !won.contains(&i) && has_win(board) {
                last_won_score = sum_unmarked(board) * op;
                won.insert(i);
            }
        }
    }
    last_won_score
}
