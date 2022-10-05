use aoc2021::{
    data_file,
    day2::{puzzle1::determine_depth_and_distance, puzzle2::determine_depth_and_distance_with_aim},
};

fn main() {
    let result = determine_depth_and_distance(data_file("day2.txt"));
    println!(
        "D2P1: New Coordinates: Distance: {}, Depth: {}; Multiplied: {}",
        result.distance,
        result.depth,
        result.distance as i32 * result.depth
    );
    let aim_result = determine_depth_and_distance_with_aim(data_file("day2.txt"));
    println!(
        "D2P1: New Coordinates: Distance: {}, Depth: {}; Multiplied: {}",
        aim_result.distance,
        aim_result.depth,
        aim_result.distance as i32 * aim_result.depth
    );
}
