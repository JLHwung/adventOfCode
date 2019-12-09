use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut file = File::open("./data/input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    data = data.trim_end().to_string();

    const LAYER_WIDTH: usize = 25;
    const LAYER_HEIGHT: usize = 6;

    let mut cursor: usize = 0;
    let mut min_zero_counts = std::usize::MAX;
    let mut min_cursor: usize = 0;
    while cursor < data.len() {
        let cursor_end: usize = cursor + LAYER_WIDTH * LAYER_HEIGHT;
        let layer = data[cursor..cursor_end].to_string();
        let zero_counts = layer.split("").filter(|x| *x == "0").count();
        if zero_counts < min_zero_counts {
            min_cursor = cursor;
            min_zero_counts = zero_counts;
        }
        cursor = cursor_end;
    }

    let min_cursor_end = min_cursor + LAYER_HEIGHT * LAYER_WIDTH;
    let min_layer = data[min_cursor..min_cursor_end].to_string();

    println!(
        "{}",
        min_layer.split("").filter(|x| *x == "1").count()
            * min_layer.split("").filter(|x| *x == "2").count()
    );

    Ok(())
}
