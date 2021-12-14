use std::cmp::max;
use std::collections::HashSet;
use std::fs;
use std::io;

const DATA_PATH: &str = "./data/day13.txt";

fn main() -> io::Result<()> {
    let raw = fs::read_to_string(fs::canonicalize(&DATA_PATH)?)?;
    let input = process(&raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
    Ok(())
}

type Location = (usize, usize);

#[derive(Debug)]
struct Folding {
    horizontal: bool,
    axis: usize,
}

type Dots = HashSet<Location>;

struct Input {
    dots: Dots,
    foldings: Vec<Folding>,
}

fn process(raw: &str) -> Input {
    let mut dots = HashSet::<Location>::new();
    let mut line_iter = raw.split('\n');
    for line in line_iter.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut value_iter = line.split(',');
        let x: usize = value_iter.next().unwrap().parse().unwrap();
        let y: usize = value_iter.next().unwrap().parse().unwrap();
        dots.insert((x, y));
    }
    let mut foldings = vec![];
    for line in line_iter {
        // fold along x=42
        if line.is_empty() {
            break;
        }
        let horizontal = &line[11..12] == "y";
        let axis: usize = line[13..].parse().unwrap();
        foldings.push(Folding { horizontal, axis });
    }
    Input { dots, foldings }
}

fn fold(dots: &mut Dots, folding: &Folding) {
    let axis = folding.axis;
    let mut to_be_removed = vec![];
    for dot in dots.iter() {
        if folding.horizontal {
            if dot.1 > axis {
                to_be_removed.push(*dot);
            }
        } else if dot.0 > axis {
            to_be_removed.push(*dot);
        }
    }
    for dot in to_be_removed {
        dots.remove(&dot);
        if folding.horizontal {
            dots.insert((dot.0, axis * 2 - dot.1));
        } else {
            dots.insert((axis * 2 - dot.0, dot.1));
        }
    }
}

fn p1(input: &Input) -> usize {
    let mut merged_dots = input.dots.clone();
    fold(&mut merged_dots, &input.foldings[0]);
    merged_dots.len()
}

fn print_dots(dots: &Dots) -> String {
    let mut result = "".to_owned();
    let mut width = 0;
    let mut height = 0;
    for dot in dots {
        width = max(dot.0, width);
        height = max(dot.1, height);
    }
    for y in 0..height + 1 {
        result.push('\n');
        for x in 0..width + 1 {
            result.push(if dots.contains(&(x, y)) { '#' } else { ' ' });
        }
    }
    result
}

fn p2(input: &Input) -> String {
    let mut merged_dots = input.dots.clone();
    for folding in &input.foldings {
        fold(&mut merged_dots, folding);
    }
    print_dots(&merged_dots)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() -> io::Result<()> {
        let raw = fs::read_to_string(fs::canonicalize(&DATA_PATH)?)?;
        let input = process(&raw);
        assert_eq!(p1(&input), 653);
        Ok(())
    }

    #[test]
    fn test_p2() -> io::Result<()> {
        let raw = fs::read_to_string(fs::canonicalize(&DATA_PATH)?)?;
        let input = process(&raw);
        assert_eq!(
            p2(&input),
            "
#    #  # ###  #### ###  ###  ###  #  #
#    # #  #  # #    #  # #  # #  # # # 
#    ##   #  # ###  ###  #  # #  # ##  
#    # #  ###  #    #  # ###  ###  # # 
#    # #  # #  #    #  # #    # #  # # 
#### #  # #  # #### ###  #    #  # #  #"
        );
        Ok(())
    }
}
