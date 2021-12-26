use std::cell::Cell;

macro_rules! DATA_PATH {
    () => {
        "../data/day25.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    println!("Answer of p1: {}", p1(raw));
    println!("Answer of p2: {}", p2());
}

fn process(raw: &str) -> Vec<Vec<Cell<char>>> {
    raw.lines().map(|n| n.chars().map(Cell::new).collect()).collect()
}

// Graph states:
const VOID: char = '.';
const EAST: char = '>';
const DOWN: char = 'v';

/// Inplace sea cucumbers simulation
fn simulate(graph: &[Vec<Cell<char>>]) -> bool {
    let mut changed = false;
    let width = graph[0].len();
    let height = graph.len();
    // east herds
    #[allow(clippy::needless_range_loop)]
    for y in 0..height {
        if let Some(x_base) = graph[y].iter().position(|x| x.get() == VOID) {
            let mut x = 0;
            while x < width {
                let cucumber = &graph[y][(x_base + x) % width];
                if cucumber.get() == EAST {
                    let next = &graph[y][(x_base + x + 1) % width];
                    if next.get() == VOID {
                        cucumber.swap(next);
                        changed = true;
                        x += 1;
                    }
                }
                x += 1;
            }
        }
    }
    // west herds
    for x in 0..width {
        if let Some(y_base) = graph.iter().position(|line| line[x].get() == VOID) {
            let mut y = 0;
            while y < height {
                let cucumber = &graph[(y_base + y) % height][x];
                if cucumber.get() == DOWN {
                    let next = &graph[(y_base + y + 1) % height][x];
                    if next.get() == VOID {
                        cucumber.swap(next);
                        changed = true;
                        y += 1;
                    }
                }
                y += 1;
            }
        }
        
    }
    changed
}

fn p1(raw: &str) -> usize {
    let graph = process(raw);
    let mut step = 0;
    loop {
        step += 1;
        if !simulate(&graph) {
            break;
        }
    }
    step
}

fn p2() -> &'static str {
    "Happy Holiday"
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        assert_eq!(p1(&raw), 334);
    }
}
