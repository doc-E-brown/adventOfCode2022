pub(crate) use anyhow::{bail, Result};
use std::fs::File;
use std::io::Read;

const OPPONENT_ROCK: &str = "A";
const OPPONENT_PAPER: &str = "B";
const OPPONENT_SCISSORS: &str = "C";

const MY_ROCK: &str = "X";
const MY_PAPER: &str = "Y";
const MY_SCISSORS: &str = "Z";

const MY_LOSS: &str = "X";
const MY_DRAW: &str = "Y";
const MY_WIN: &str = "Z";

#[derive(Debug, PartialEq, Eq)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Round1,
    Round2,
}

fn win_lose(opponent: &str, me: &str) -> Result<GameResult> {
    // 8392
    if me == MY_ROCK {
        if opponent == OPPONENT_PAPER {
            Ok(GameResult::Lose)
        } else if opponent == OPPONENT_ROCK {
            Ok(GameResult::Draw)
        } else {
            Ok(GameResult::Win)
        }
    } else if me == MY_PAPER {
        if opponent == OPPONENT_PAPER {
            Ok(GameResult::Draw)
        } else if opponent == OPPONENT_SCISSORS {
            Ok(GameResult::Lose)
        } else {
            Ok(GameResult::Win)
        }
    } else {
        if opponent == OPPONENT_SCISSORS {
            Ok(GameResult::Draw)
        } else if opponent == OPPONENT_ROCK {
            Ok(GameResult::Lose)
        } else {
            Ok(GameResult::Win)
        }
    }
}

fn target_result(opponent: &str, me: &str) -> Result<u16> {
    if me == MY_DRAW {
        match opponent {
            OPPONENT_PAPER => selection_score(MY_PAPER),
            OPPONENT_ROCK => selection_score(MY_ROCK),
            OPPONENT_SCISSORS => selection_score(MY_SCISSORS),
            _ => !bail!("Invalid selection"),
        }
    } else if me == MY_LOSS {
        if opponent == OPPONENT_PAPER {
            selection_score(MY_ROCK)
        } else if opponent == OPPONENT_ROCK {
            selection_score(MY_SCISSORS)
        } else {
            selection_score(MY_PAPER)
        }
    } else {
        if opponent == OPPONENT_PAPER {
            selection_score(MY_SCISSORS)
        } else if opponent == OPPONENT_ROCK {
            selection_score(MY_PAPER)
        } else {
            selection_score(MY_ROCK)
        }
    }
}

fn selection_score(select: &str) -> Result<u16> {
    Ok(match select {
        MY_ROCK => 1,
        MY_PAPER => 2,
        MY_SCISSORS => 3,
        _ => bail!(r#"Invalid selection"#),
    })
}

fn result_score(game_result: GameResult) -> u16 {
    match game_result {
        GameResult::Draw => 3,
        GameResult::Win => 6,
        GameResult::Lose => 0,
    }
}

fn determine_score(plays: String, mode: Mode) -> Result<u16> {
    let tmp: Vec<&str> = plays.split_terminator(" ").collect();
    let (opponent, me) = (tmp[0], tmp[1]);

    if mode == Mode::Round1 {
        // Get score from selection
        let selection_score: u16 = selection_score(me)?;
        // Get win / loss score
        let result_score: u16 = result_score(win_lose(opponent, me)?);
        let score = selection_score + result_score;
        println!("{}, {:#?}, {}", plays, win_lose(opponent, me)?, score);
        Ok(score)
    } else {
        let selection_score = target_result(opponent, me)?;
        let result_score = match me {
            MY_LOSS => result_score(GameResult::Lose),
            MY_WIN => result_score(GameResult::Win),
            MY_DRAW => result_score(GameResult::Draw),
            _ => bail!(r#"Invalid selection"#),
        };

        Ok(selection_score + result_score)
    }
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let score: u16 = buff
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x: &str| -> u16 { determine_score(x.to_string(), Mode::Round2).unwrap() })
        .sum();
    println!("Score: {}", score);
}
