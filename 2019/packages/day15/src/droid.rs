use crate::intcode;
use std::collections::HashMap;

type Metric = i32;
type Pos = (Metric, Metric);
type Solution = Vec<Pos>;

#[derive(PartialEq, Clone)]
enum TileType {
    Unknown = 0,
    Air = 1,
    Wall = 2,
    Oxygen = 3,
}

type Environment = HashMap<Pos, TileType>;

type Direction = i64;
const NORTH: i64 = 1;
const SOUTH: i64 = 2;
const WEST: i64 = 3;
const EAST: i64 = 4;

pub struct Droid {
    memory: intcode::Memory,
    // pc, rb
    program_state: (u64, i64),
    environment: Environment,
    solution: Solution,
}

impl Droid {
    pub fn new(program: intcode::Program) -> Droid {
        let mut environment = HashMap::new();
        environment.insert((0, 0), TileType::Air);
        Droid {
            memory: intcode::Memory::new(program),
            environment,
            solution: Vec::new(),
            program_state: (0, 0),
        }
    }

    pub fn explore(&mut self) {
        self.update_environment(NORTH, &(0, 0));
    }

    pub fn distance(&self) -> usize {
        unimplemented!()
    }

    fn reject(&self, solution: &Solution) -> bool {
        if solution.len() == 1 {
            return false;
        }

        let current_pos = solution[solution.len() - 1];
        // loop
        if solution[0..solution.len() - 2]
            .iter()
            .any(|pos| *pos == current_pos)
        {
            return true;
        }
        if get_type(&current_pos, &self.environment) == TileType::Wall {
            return true;
        }
        false
    }

    fn accept(&self, solution: &Solution) -> bool {
        let current_pos = solution[solution.len() - 1];
        get_type(&current_pos, &self.environment) == TileType::Oxygen
    }

    fn first(&mut self, solution: Solution) -> Solution {
        let mut new_solution = solution.clone();
        let current_pos = solution[solution.len() - 1];
        let new_pos = self.update_environment(NORTH, &current_pos);
        new_solution.push(new_pos);
        new_solution
    }

    fn next(&mut self, solution: Solution) -> Option<Solution> {
        assert_ne!(solution.len(), 1);
        let current_pos = solution[solution.len() - 1];
        let prev_pos = solution[solution.len() - 2];
        let next_direction = next_direction(prev_direction(&prev_pos, &current_pos));
        match next_direction {
            Some(direction) => {
                let new_pos = self.update_environment(direction, &prev_pos);
                let mut new_solution = solution.clone();
                new_solution[solution.len() - 1] = new_pos;
                Some(new_solution)
            }
            None => None,
        }
    }

    fn backtrack(&mut self, solution: Solution) {
        if self.reject(&solution) {
            return;
        }
        if self.accept(&solution) {
            self.solution = solution.clone();
        }
        let mut next_solution: Option<Solution> = Some(self.first(solution.to_vec()));
        loop {
            match next_solution {
                Some(solution) => {
                    self.backtrack(solution.clone());
                    next_solution = self.next(solution);
                }
                None => break,
            }
        }
    }

    fn update_environment(&mut self, direction: Direction, current_pos: &Pos) -> Pos {
        let new_pos_type = self.call_sensor(direction);
        let new_pos = next_tile(current_pos, direction);
        println!("{}, {}", new_pos.0, new_pos.1);
        self.environment.entry(new_pos).or_insert(new_pos_type);
        new_pos
    }

    fn call_sensor(&mut self, direction: Direction) -> TileType {
        let mut stdin = [direction].to_vec();
        let mut stdout = Vec::new();
        let (mut pc, mut rb) = self.program_state;
        intcode::interpreter(&mut self.memory, &mut stdin, &mut stdout, &mut pc, &mut rb);
        assert_eq!(stdout.len(), 1);
        match stdout[0] {
            0 => TileType::Wall,
            1 => TileType::Air,
            2 => TileType::Oxygen,
            _ => unreachable!(),
        }
    }
}

fn next_tile(pos: &Pos, direction: Direction) -> Pos {
    match direction {
        NORTH => (pos.0, pos.1 + 1),
        SOUTH => (pos.0, pos.1 - 1),
        EAST => (pos.0 + 1, pos.1),
        WEST => (pos.0 - 1, pos.1),
        _ => unreachable!(),
    }
}

fn prev_direction(base: &Pos, prev: &Pos) -> Direction {
    neighbors(base).iter().position(|f| *f == *prev).unwrap() as i64 + 1
}

fn next_direction(prev_direction: Direction) -> Option<Direction> {
    match prev_direction {
        1 | 2 | 3 => Some(prev_direction + 1),
        4 => None,
        _ => unreachable!(),
    }
}

fn neighbors(pos: &Pos) -> Vec<Pos> {
    [
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
    ]
    .to_vec()
}

fn neighbors_except(pos: &Pos, prev: &Pos) -> Vec<Pos> {
    neighbors(pos)
        .iter()
        .filter(|&p| *p != *prev)
        .map(|x| x.to_owned())
        .collect()
}

fn get_type(pos: &Pos, environment: &Environment) -> TileType {
    environment
        .get(pos)
        .map_or(TileType::Unknown, |x| x.clone())
}

fn is_wall(pos: &Pos, environment: &Environment) -> bool {
    get_type(pos, environment) == TileType::Wall
}
