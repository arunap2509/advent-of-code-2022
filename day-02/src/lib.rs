enum Game {
    Rock,
    Paper,
    Scissor
}

pub fn process_input(input: &str) -> String {
    let total_score: u32 = input.lines().map(|line| {
        let mut current_round_score = 0;
        if let Some((first, second)) = line.split_once(" ") {
            let player1 = convert_to_game(first);
            let player2 = convert_to_game(second);
            let score = match player1 {
                Game::Rock => choosing_rock(player2),
                Game::Paper => choosing_paper(player2),
                Game::Scissor => choosing_scissor(player2)
            };
            current_round_score = score;
        }
        current_round_score
    }).sum();
    total_score.to_string()
}

fn convert_to_game(choice: &str) -> Game {
    if choice == "A" || choice == "X" {
        Game::Rock
    } else if choice == "B" || choice == "Y" {
        Game::Paper
    } else {
        Game::Scissor
    }
}

fn choosing_rock(player2: Game) -> u32 {
    match player2 {
        Game::Rock => 3 + 1,
        Game::Paper => 6 + 2,
        Game::Scissor => 0 + 3
    }
}

fn choosing_paper(player2: Game) -> u32 {
    match player2 {
        Game::Rock => 0 + 1,
        Game::Paper => 3 + 2,
        Game::Scissor => 6 + 3
    }
}

fn choosing_scissor(player2: Game) -> u32 {
    match player2 {
        Game::Rock => 6 + 1,
        Game::Paper => 0 + 2,
        Game::Scissor => 3 + 3
    }
}

pub fn process_input_2(input: &str) -> String {
    let total_score: u32 = input.lines().map(|line| {
        let mut current_round_score = 0;
        if let Some((first, second)) = line.split_once(" ") {
            let player1 = convert_to_game(first);
            let score = player2_tactics(player1, second);
            current_round_score = score;
        }
        current_round_score
    }).sum();

    total_score.to_string()
}

fn player2_tactics(player1: Game, player2: &str) -> u32 {
    match player2 {
        "X" => to_loose(player1),
        "Y" => to_draw(player1),
        _ => to_win(player1)
    }
}

fn to_win(player1: Game) -> u32 {
    match player1 {
        Game::Rock => 6 + 2,
        Game::Paper => 6 + 3,
        Game::Scissor => 6 + 1
    }
}

fn to_draw(player1: Game) -> u32 {
    match player1 {
        Game::Rock => 3 + 1,
        Game::Paper => 3 + 2,
        Game::Scissor => 3 + 3
    }
}

fn to_loose(player1: Game) -> u32 {
    match player1 {
        Game::Rock => 0 + 3,
        Game::Paper => 0 + 1,
        Game::Scissor => 0 + 2
    }
}