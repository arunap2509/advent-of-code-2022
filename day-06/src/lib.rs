use std::collections::HashSet;

pub fn process_input(input: &str) -> String {
    let binding = input.chars().collect::<Vec<char>>();
    let size = binding.windows(14).enumerate().find(|(_i, slice)| {
        let set = slice.iter().collect::<HashSet<&char>>();
        set.len() == slice.len()
    });

    if let Some(s) = size {
        return (s.0 + 14).to_string();
    }

    0.to_string()
}