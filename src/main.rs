use advent_of_code_2021::day1::{puzzle1::count_depths, puzzle2::count_depths_windowed};

fn main() {
    println!("D1P1: Depth Increases: {}", count_depths("input.txt"));
    println!(
        "D1P2: Depth Increases (3-measure window): {}",
        count_depths_windowed("input.txt", 3)
    );
}
