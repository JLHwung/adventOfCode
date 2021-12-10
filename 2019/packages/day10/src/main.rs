use std::cmp;
use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut file = File::open("./packages/day10/data/input.txt")?;
    let mut raw = String::new();
    file.read_to_string(&mut raw).unwrap();

    let mut map: Vec<Vec<char>> = raw
        .trim_end()
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();

    let laser_pos = (17isize, 22isize);

    let mut counter = 1;
    let mut laser_angle = -PI / 2f64 - 1e-09;
    loop {
        let detectables = find_detectables(&map, laser_pos);
        let victim = find_next_victim(detectables, &laser_pos, &mut laser_angle);
        println!(
            "R.I.P. The #{} asteroid to be vaporized is at ({},{}) from {}",
            counter, victim.0, victim.1, laser_angle
        );
        destroy_asteroid(&mut map, &victim);
        if counter == 200 {
            println!("{}", victim.0 * 100 + victim.1);
            break;
        }
        counter += 1;
    }
    Ok(())
}

type Pos = (isize, isize);

fn destroy_asteroid(map: &mut Vec<Vec<char>>, at: &Pos) {
    map[at.1 as usize][at.0 as usize] = '.';
}

fn find_next_victim(detectables: Vec<Pos>, laser_pos: &Pos, laser_angle: &mut f64) -> Pos {
    let mut next_laser_angle = *laser_angle;
    let mut next_victim: Pos = (-1, 0);
    let mut min_angle_change = 2f64 * PI;
    for pos in detectables {
        let angle = (pos.1 as f64 - laser_pos.1 as f64).atan2(pos.0 as f64 - laser_pos.0 as f64);
        let mut next_angle_change = angle - *laser_angle;
        if next_angle_change <= 0f64 {
            next_angle_change += 2f64 * PI;
        }
        assert_eq!(
            next_angle_change > 0f64 && next_angle_change <= 2f64 * PI,
            true
        );
        if next_angle_change < min_angle_change {
            min_angle_change = next_angle_change;
            next_laser_angle = angle;
            next_victim = pos;
        }
    }
    *laser_angle = next_laser_angle;
    assert_ne!(next_victim.0, -1);
    next_victim
}

fn find_detectables(map: &Vec<Vec<char>>, coordinate: Pos) -> Vec<Pos> {
    let (xx, yy) = coordinate;
    let height = map.len() as isize;
    let width = map[0].len() as isize;

    let mut result = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if map[y as usize][x as usize] == '.' {
                continue;
            }
            if xx == x && yy == y {
                continue;
            }
            if is_detectable((xx, yy), (x, y), map) {
                result.push((x, y));
            }
        }
    }

    result
}

fn is_detectable(from: Pos, to: Pos, map: &Vec<Vec<char>>) -> bool {
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
                    return false;
                }
            }
        }
    }
    true
}
