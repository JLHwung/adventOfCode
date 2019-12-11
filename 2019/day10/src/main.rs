use std::fs::File;
use std::io;
use std::io::Read;
use std::cmp;

fn main() -> io::Result<()> {
    let mut file = File::open("./data/input.txt")?;
    let mut raw = String::new();
    file.read_to_string(&mut raw).unwrap();

    let map: Vec<Vec<char>> = raw
        .trim_end()
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();

    let height = map.len();
    let width = map[0].len();

    let mut max_detectable: usize = 0;
    let mut xx = 0;
    let mut yy = 0;

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == '.' {
                continue;
            }
            let detectable = count_detectable(&map, (x as isize, y as isize));
            if detectable > max_detectable {
                max_detectable = detectable;
                xx = x;
                yy = y;
            }
        }
    }

    println!("{},{} with {} detected", xx, yy, max_detectable);
    Ok(())
}

fn count_detectable(map: &Vec<Vec<char>>, coordinate: (isize, isize)) -> usize {
    let (xx, yy) = coordinate;
    let height = map.len() as isize;
    let width = map[0].len() as isize;

    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            if map[y as usize][x as usize] == '.' {
                continue;
            }
            if xx == x && yy == y {
                continue;
            }
            if is_detectable((xx, yy), (x, y), map) {
                count += 1;
            }
        }
    }

    count
}

fn is_detectable(from: (isize, isize), to: (isize, isize), map: &Vec<Vec<char>>) -> bool {
    let (xx, yy) = from;
    let (x, y) = to;
    let (x_min, x_max) = (cmp::min(x, xx), cmp::max(x, xx));
    let (y_min, y_max) = (cmp::min(y, yy), cmp::max(y, yy));
    for x_cursor in x_min..x_max + 1 {
        for y_cursor in y_min..y_max + 1 {
            if x_cursor == xx && y_cursor == yy {
                continue;
            }
            if x_cursor == x && y_cursor == y {
                continue;
            }
            if (x_cursor - xx) * (y - yy) == (x - xx) * (y_cursor - yy) {
                if map[y_cursor as usize][x_cursor as usize] == '#' {
                    return false
                }
            }
        }
    }
    true

}

