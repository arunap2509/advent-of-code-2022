use std::fs;

use day_04::process_input_2;

fn main() {
    let file_content = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_input_2(&file_content));
}
