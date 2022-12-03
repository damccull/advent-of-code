use advent_of_code_common::read_data_from_file;

fn main() -> Result<(), anyhow::Error> {
    let data = process_data(read_data_from_file("data/day2.txt")?)?;
    let score = calculate_score(xyz_to_move(data.clone()));
    println!(
        "The total score for all the guidebook's games is {}.",
        score
    );

    let score = calculate_score(xyz_to_move_by_outcome(data));
    println!(
        "After the elf explains how the guidebook actually works, your recalcualted total score is {}.",
        score
    );
    Ok(())
}

const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSORS_SCORE: u32 = 3;
const WIN_SCORE: u32 = 6;
const DRAW_SCORE: u32 = 3;
const LOSS_SCORE: u32 = 0;

fn process_data(rounds: Vec<String>) -> Result<Vec<Strategy>, anyhow::Error> {
    let x = rounds
        .iter()
        .map(|r| {
            let values = r.split(' ').collect::<Vec<_>>();

            Ok(Strategy {
                opponent_move: values[0].try_into()?,
                player_move: values[1].try_into()?,
            })
        })
        .collect::<Result<Vec<_>, _>>();
    x
}

/// X, Y, and Z become Paper, Rock, and Scissors respectively
fn xyz_to_move(rounds: Vec<Strategy>) -> Vec<Round> {
    rounds
        .iter()
        .map(|r| {
            let player_move = match r.player_move {
                PlayerChoice::X => Move::Rock,
                PlayerChoice::Y => Move::Paper,
                PlayerChoice::Z => Move::Scissors,
            };
            Round {
                opponent_move: r.opponent_move,
                player_move,
            }
        })
        .collect::<Vec<_>>()
}

/// Need to translate X, Y, and Z into appropriate outcomes depending on the opponent move.
/// X is a loss move
/// Y is a draw move
/// Z is a win move
fn xyz_to_move_by_outcome(rounds: Vec<Strategy>) -> Vec<Round> {
    rounds
        .iter()
        .map(|r| {
            let player_move = match r.opponent_move {
                Move::Rock => match r.player_move {
                    PlayerChoice::X => Move::Scissors,
                    PlayerChoice::Y => Move::Rock,
                    PlayerChoice::Z => Move::Paper,
                },
                Move::Paper => match r.player_move {
                    PlayerChoice::X => Move::Rock,
                    PlayerChoice::Y => Move::Paper,
                    PlayerChoice::Z => Move::Scissors,
                },
                Move::Scissors => match r.player_move {
                    PlayerChoice::X => Move::Paper,
                    PlayerChoice::Y => Move::Scissors,
                    PlayerChoice::Z => Move::Rock,
                },
            };
            Round {
                opponent_move: r.opponent_move,
                player_move,
            }
        })
        .collect::<Vec<_>>()
}

fn calculate_score(rounds: Vec<Round>) -> u32 {
    let mut score: u32 = 0;
    for round in rounds {
        let winner = determine_round_winner(round);

        score += match winner {
            Outcome::Draw => DRAW_SCORE,
            Outcome::Opponent => LOSS_SCORE,
            Outcome::Player => WIN_SCORE,
        };

        score += match round.player_move {
            Move::Rock => ROCK_SCORE,
            Move::Paper => PAPER_SCORE,
            Move::Scissors => SCISSORS_SCORE,
        }
    }
    score
}

fn determine_round_winner(round: Round) -> Outcome {
    match round.opponent_move {
        Move::Rock if round.player_move == Move::Paper => Outcome::Player,
        Move::Rock if round.player_move == Move::Scissors => Outcome::Opponent,
        Move::Rock => Outcome::Draw,

        Move::Paper if round.player_move == Move::Scissors => Outcome::Player,
        Move::Paper if round.player_move == Move::Rock => Outcome::Opponent,
        Move::Paper => Outcome::Draw,

        Move::Scissors if round.player_move == Move::Rock => Outcome::Player,
        Move::Scissors if round.player_move == Move::Paper => Outcome::Opponent,
        Move::Scissors => Outcome::Draw,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Round {
    pub opponent_move: Move,
    pub player_move: Move,
}

#[derive(Debug, Clone, Copy)]
struct Strategy {
    pub opponent_move: Move,
    pub player_move: PlayerChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlayerChoice {
    X,
    Y,
    Z,
}

impl TryFrom<&str> for PlayerChoice {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(PlayerChoice::X),
            "Y" => Ok(PlayerChoice::Y),
            "Z" => Ok(PlayerChoice::Z),
            _ => anyhow::bail!("Invalid character in pages: '{}'", value),
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Draw,
    Opponent,
    Player,
}

impl TryFrom<&str> for Move {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => anyhow::bail!("Invalid character in pages: '{}'", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{calculate_score, process_data, xyz_to_move, xyz_to_move_by_outcome, Move, Round};

    fn test_data() -> Vec<String> {
        let test_data = r##"A Y
B X
C Z"##;
        test_data
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn process_data_successful() {
        let data = process_data(test_data());
        assert!(data.is_ok());
    }

    #[test]
    fn xyz_to_move_is_correct() -> Result<(), anyhow::Error> {
        let data = xyz_to_move(process_data(test_data())?);
        let target_data = vec![
            Round {
                opponent_move: Move::Rock,
                player_move: Move::Paper,
            },
            Round {
                opponent_move: Move::Paper,
                player_move: Move::Rock,
            },
            Round {
                opponent_move: Move::Scissors,
                player_move: Move::Scissors,
            },
        ];

        assert_eq!(data, target_data);

        Ok(())
    }

    #[test]
    fn xyz_to_move_by_outcome_is_correct() -> Result<(), anyhow::Error> {
        let data = xyz_to_move_by_outcome(process_data(test_data())?);
        let target_data = vec![
            Round {
                opponent_move: Move::Rock,
                player_move: Move::Rock,
            },
            Round {
                opponent_move: Move::Paper,
                player_move: Move::Rock,
            },
            Round {
                opponent_move: Move::Scissors,
                player_move: Move::Rock,
            },
        ];

        assert_eq!(data, target_data);

        Ok(())
    }

    #[test]
    fn calculate_score_is_correct() -> Result<(), anyhow::Error> {
        let data = xyz_to_move(process_data(test_data())?);
        let target_score = 15u32;

        let score = calculate_score(data);

        assert_eq!(target_score, score);

        Ok(())
    }
}
