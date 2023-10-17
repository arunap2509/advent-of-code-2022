use std::fs;

use day_06::process_input;

fn main() {
    let file_content = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_input(&file_content));
}
