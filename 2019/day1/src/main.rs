use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let mass: i32 = line?.parse().unwrap();
        let fuel = mass / 3 - 2;
        sum += fuel;
    }

    println!("{}", sum);

    Ok(())
}
