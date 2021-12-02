use advent_of_code_2021::{
    day1::{puzzle1::count_depths, puzzle2::count_depths_windowed},
    day2::puzzle1::determine_depth_and_distance,
};

fn main() {
    // Day 1
    println!("D1P1: Depth Increases: {}", count_depths("input/day1.txt"));
    println!(
        "D1P2: Depth Increases (3-measure window): {}",
        count_depths_windowed("input/day1.txt", 3)
    );

    // Day 2
    let result = determine_depth_and_distance("input/day2.txt");
    println!(
        "D2P1 New Coordinates: Distance: {}, Depth: {}; Multiplied: {}",
        result.distance,
        result.depth,
        result.distance as i32 * result.depth
    );
}
