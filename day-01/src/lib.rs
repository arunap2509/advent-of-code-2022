pub fn process_part1(input: &str) -> String {
    let mut group_sum = input
        .split("\r\n\r\n")
        .map(|group| {
            group
                .lines()
                .map(|num_str| match num_str.trim().parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => 0,
                })
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    group_sum.sort_by(|a, b| b.cmp(a));
    let sum: u32 = group_sum.iter().take(3).sum();

    sum.to_string()
}
