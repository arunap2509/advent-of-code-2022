use std::fs;

use day_02::process_input_2;

fn main() {
    let file_content = fs::read_to_string("./input.txt").unwrap();
    println!("score is {}", process_input_2(&file_content));
}
