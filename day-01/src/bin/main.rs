use day_01::process_part1;
use std::fs;

fn main() {
    // let byte_ip = fs::read("./input.txt");
    // println!("{:?}", byte_ip.unwrap().iter().map(|&c| c as char).collect::<String>());
    let input = fs::read_to_string("./input.txt");
    let output = match input {
        Ok(ip) => process_part1(&ip),
        Err(_) => "error while processing input".to_string(),
    };
    println!("{}", output);
}
