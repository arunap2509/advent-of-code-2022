pub fn process_input(input: &str) -> String {
    let toatl_overlap: u32 = input.lines().map(|line| {
        let current_section = line.split(",").map(|section| {
            let (start, end) = section.split_once("-").unwrap();
            (start.parse::<u32>().unwrap(), end.parse::<u32>().unwrap())
        }).collect::<Vec::<(u32, u32)>>();

        let first = current_section[0];
        let second = current_section[1];

        if first.0 <= second.0 && first.1 >= second.1 {
            1
        } else if second.0 <= first.0 && second.1 >= first.1 {
            1
        } else {
            0
        }
    }).sum();

    toatl_overlap.to_string()
}

pub fn process_input_2(input: &str) -> String {
    let toatl_overlap: u32 = input.lines().map(|line| {
        let mut current_section = line.split(",").map(|section| {
            let (start, end) = section.split_once("-").unwrap();
            (start.parse::<u32>().unwrap(), end.parse::<u32>().unwrap())
        }).collect::<Vec::<(u32, u32)>>();

        current_section.sort_by(|a, b| a.cmp(b));

        let first = current_section[0];
        let second = current_section[1];

        if first.0 <= second.0 && first.1 >= second.0 {
            1
        } else {
            0
        }
    }).sum();

    toatl_overlap.to_string()
}