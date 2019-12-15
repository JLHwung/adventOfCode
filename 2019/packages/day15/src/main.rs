use day15::*;
use std::io::Result;

fn main() -> Result<()> {
    let program = intcode::read_file_as_program("./packages/day15/data/input.txt")?;

    let mut droid = droid::Droid::new(program);

    droid.explore();

    println!("{}", droid.distance());

    Ok(())
}
