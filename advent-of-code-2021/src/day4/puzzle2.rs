use std::collections::HashMap;

use super::{BingoBoard, BingoGame};

pub fn find_last_winning_board_id(mut game: BingoGame) -> u32 {
    let draws = game.draws.clone();
    let mut when_won: HashMap<String, (usize, u32)> = HashMap::new();
    for (i, draw) in draws.iter().enumerate() {
        play_number_on_board(&mut game, *draw);
        for board in &game.boards {
            let x: String = board.iter().map(|n| n.0.to_string()).collect();
            let digest = md5::compute(x);
            let board_hash = format!("{:x}", digest);
            if let Some(winner) = check_for_winner(board) {
                let board_sum = winner
                    .iter()
                    .filter(|&&(_, v)| !v)
                    .fold(0_u32, |acc, (n, _)| acc + n);

                let id = board_sum * draw;
                when_won.entry(board_hash).or_insert((i, id));
            }
        }
    }
    let x = when_won.iter().fold(
        (0_u32, 0_usize),
        |(saved_id, saved_when), (_hash, (when, id))| {
            if id > &0 && when > &saved_when {
                (*id, *when)
            } else {
                (saved_id, saved_when)
            }
        },
    );
    x.0
}

fn play_number_on_board(game: &mut BingoGame, draw: u32) {
    for board in game.boards.iter_mut() {
        for (num, flag) in board.iter_mut() {
            if num == &draw {
                *flag = true;
            }
        }
    }
}

fn check_for_winner(board: &[(u32, bool)]) -> Option<BingoBoard> {
    //Check for horizontal win
    let horizontal_win = board.chunks(5).any(|row| row.iter().all(|&(_, v)| v));
    //Check for vertical win

    let mut vertical_win = false;
    for i in 0..5 {
        let v = board.iter().skip(i).step_by(5).all(|&(_, v)| v);
        if v {
            vertical_win = v
        }
    }

    if horizontal_win || vertical_win {
        return Some(board.to_owned());
    }

    None
}
