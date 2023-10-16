pub fn process_input(input: &str) -> String {
    let score: u32 = input.lines().map(|line| {
        let line_mid = line.len() / 2;
        // let f = &line[..line_mid];
        // let s = &line[line_mid..];
        let (first, second) = line.split_at(line_mid);
        let common = common_char(first, second);
        let priority = match common {
            Some(c) => get_priority(c),
            None =>  0
        };
        priority
    }).sum();
    score.to_string()
}

pub fn process_input_2(input: &str) -> String {
    let mut count = 0;
    let mut current_vec : [&str;3] = [Default::default();3];
    let score: u32 = input.lines().map(|line| {

        current_vec[count] = line;
        count += 1;
        let mut priority = 0;

        if count == 3 {
            count = 0;
            let common = common_char_in_3(current_vec[0], current_vec[1], current_vec[2]);
            let current_priority = match common {
                Some(c) => get_priority(c),
                None => 0
            };
            priority = current_priority;
        }

        priority
    }).sum();


    score.to_string()
}

fn get_priority(ch: char) -> u32 {
    // let letter_scores = ('a'..'z').chain('A'..'Z').enumerate().map(|(idx, c)| (c, idx + 1)).collect::<HashMap<char, usize>>();
    // let score = letter_scores.get(&ch).unwrap();
    if ch.is_uppercase() {
        return ch as u32 - 'A' as u32 + 26 + 1;
    }

    ch as u32 - 'a' as u32 + 1
}

fn common_char_in_3(first: &str, second: &str, third: &str) -> Option<char> {
    // first.chars().find(|c| {
    //     second.contains(*c) && third.contains(*c)
    // });
    let first_vec = first.chars().collect::<Vec<char>>();
    let second_vec = second.chars().collect::<Vec<char>>();
    for (_, c) in third.chars().enumerate() {
        if first_vec.contains(&c) && second_vec.contains(&c) {
            return Some(c);
        }
    }
    None
}

fn common_char(first: &str, second: &str) -> Option<char> {
    let first_vec = first.chars().collect::<Vec<char>>();
    for (_, c) in second.chars().enumerate() {
        if first_vec.contains(&c) {
            return Some(c);
        }
    }
    // for (idx, _) in first.chars().enumerate() {
    //     for (s_idx, _) in second.chars().enumerate() {
    //         if first.chars().nth(idx) == second.chars().nth(s_idx) {
    //             return Some(first.chars().nth(idx).unwrap());
    //         }
    //     }
    // }

    None
}