use super::{BingoBoard, BingoGame};

pub fn find_winning_board_id(mut game: BingoGame) -> u32 {
    let mut result = 0;
    let draws = game.draws.clone();
    for draw in draws {
        play_number_on_board(&mut game, draw);
        if let Some(winner) = check_for_winner(&game) {
            let board_sum = winner
                .iter()
                .filter(|&&(_, v)| !v)
                .fold(0_u32, |acc, (n, _)| acc + n);
            result = board_sum * draw;
            break;
        }
    }
    result
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

fn check_for_winner(game: &BingoGame) -> Option<BingoBoard> {
    for board in &game.boards {
        //Check for horizontal win
        let horizontal_win = board
            .as_slice()
            .chunks(5)
            .any(|row| row.iter().all(|&(_, v)| v));
        //Check for vertical win

        let mut vertical_win = false;
        for i in 0..5 {
            let v = board.as_slice().iter().skip(i).step_by(5).all(|&(_, v)| v);
            if v {
                vertical_win = v
            }
        }

        if horizontal_win || vertical_win {
            return Some(board.to_owned());
        }
    }
    None
}
