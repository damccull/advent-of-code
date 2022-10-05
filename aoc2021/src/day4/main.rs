use aoc2021::{
    data_file,
    day4::{build_game, puzzle1::find_winning_board_id, puzzle2::find_last_winning_board_id},
};

fn main() {
    let g = build_game(data_file("day4.txt"));
    let id = find_winning_board_id(g);
    println!("D4P1: The winning board ID is {}.", id);

    let g = build_game(data_file("day4.txt"));
    let id = find_last_winning_board_id(g);
    println!("D4P1: The winning board ID is {}.", id);
}
