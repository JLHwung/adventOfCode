use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::cmp;

fn main() -> io::Result<()> {
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let mass: i32 = line?.parse().unwrap();
        let mut fuel = mass;
        while fuel > 0 {
            fuel = cmp::max(fuel / 3 - 2, 0);
            sum += fuel;
        }
    }

    println!("{}", sum);

    Ok(())
}
