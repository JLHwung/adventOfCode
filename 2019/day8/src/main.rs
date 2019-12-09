use std::fs::File;
use std::io;
use std::io::Read;

const LAYER_WIDTH: usize = 25;
const LAYER_HEIGHT: usize = 6;
const LAYER_AREA: usize = LAYER_HEIGHT * LAYER_WIDTH;

fn main() -> io::Result<()> {
    let mut file = File::open("./data/input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    data = data.trim_end().to_string();

    let data_len = data.len();
    let mut cursor = data_len - LAYER_AREA;

    let mut base_layer: Vec<char> = data[cursor..data_len].to_string().chars().collect();

    while cursor > 0 {
        let next_cursor = cursor - LAYER_AREA;
        let filter: Vec<char> = data[next_cursor..cursor].to_string().chars().collect();
        apply_layer(&mut base_layer, &filter);
        cursor = next_cursor;
    }

    print_layer(&base_layer);
    Ok(())
}
fn print_layer(layer: &Vec<char>) {
    for i in 0..LAYER_HEIGHT {
        for j in 0..LAYER_WIDTH {
            match layer[i * LAYER_WIDTH + j] {
                '2' => print!(" "),
                '1' => print!("■"),
                '0' => print!(" "),
                _ => unreachable!(),
            }
        }
        print!("\n");
    }
}

fn apply_layer(base: &mut Vec<char>, filter: &Vec<char>) {
    for i in 0..LAYER_AREA {
        if filter[i] != '2' {
            base[i] = filter[i];
        }
    }
}
