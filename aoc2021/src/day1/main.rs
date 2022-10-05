use aoc2021::{
    data_file,
    day1::{puzzle1::count_depths, puzzle2::count_depths_windowed},
};

fn main() {
    println!(
        "D1P1: Depth Increases: {}",
        count_depths(data_file("day1.txt"))
    );
    println!(
        "D1P2: Depth Increases (3-measure window): {}",
        count_depths_windowed(data_file("day1.txt"), 3)
    );
}