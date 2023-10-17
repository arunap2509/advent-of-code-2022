pub fn process_input(input: &str) -> String {
    let (current, move_plan) = input.split_once("\r\n\r\n").unwrap();
    let mut current_stack = current.lines().collect::<Vec<&str>>();
    let mut stack = process_current_stack(&mut current_stack);
    let moves = process_move_plan(move_plan);
    move_crates(&mut stack, moves)
}

fn process_move_plan(move_plan: &str) -> Vec<Vec<u32>> {
    let steps: Vec<Vec<u32>> = move_plan
        .lines()
        .map(|line| {
            let step = line.split(" ").filter(|s| {
                match s.parse::<u32>() {
                    Ok(_) => true,
                    Err(_) => false
                }
            }).map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
            step
        })
        .collect();
    steps
}

fn process_current_stack<'a>(current_stack: &'a mut Vec<&str>) -> Vec<Vec<char>> {
    let mut stacks_col = current_stack
        .pop()
        .unwrap()
        .trim()
        .split("   ")
        .map(|_| Vec::<char>::new())
        .collect::<Vec<Vec<char>>>();

    let rows: Vec<Vec<char>> = current_stack
        .iter()
        .map(|line| {
            let row = line
                .chars()
                .enumerate()
                .filter(|(idx, _)| idx % 4 == 1)
                .map(|(_idx, c)| c)
                .collect::<Vec<char>>();
            row
        })
        .collect();

    add_to_map(rows, &mut stacks_col);
    stacks_col
}

fn add_to_map(rows: Vec<Vec<char>>, stack_col: &mut Vec<Vec<char>>) {
    rows.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(idx, c)| {
            if !c.is_ascii_whitespace() {
                stack_col[idx].insert(0, *c);
            }
        })
    });
}

fn move_crates(stacks: &mut Vec<Vec<char>>, moves: Vec<Vec<u32>>) -> String {
    moves.iter().for_each(|m| {
        let quantity = m[0];
        let from = m[1] as usize;
        let to = m[2] as usize;
        let mut popped_val = vec![];

        // this is the only change required for the day 5, part 2 solution
        if quantity <= 1 {
            (0..quantity).for_each(|_| popped_val.push(stacks[from - 1].pop().unwrap()));
        } else {
            (0..quantity).for_each(|_| popped_val.push(stacks[from - 1].pop().unwrap()));
            popped_val.reverse();
        }

        popped_val.iter().for_each(|v| stacks[to - 1].push(*v));
    });

    get_top_element(stacks)
}

fn get_top_element(stacks: &mut Vec<Vec<char>>) -> String {
    stacks
        .iter_mut()
        .map(|f| f.pop().unwrap())
        .into_iter()
        .collect()
}
