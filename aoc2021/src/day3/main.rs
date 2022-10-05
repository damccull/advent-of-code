use aoc2021::{
    data_file,
    day3::{puzzle1::get_diagnostics, puzzle2::get_life_support_status},
};

fn main() {
    println!(
        "D3P1: The overall diagnostic code is {}.",
        get_diagnostics(data_file("day3.txt"))
    );
    println!(
        "D3P2: The overall life support status code is {}.",
        get_life_support_status(data_file("day3.txt"))
    );
}
