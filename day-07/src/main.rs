use std::fs;

use day_07::process_input;

fn main() {
    let file_content = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_input(&file_content));
}
