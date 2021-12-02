use std::{
    arch::x86_64::_mm_broadcastb_epi8,
    path::{Path, PathBuf},
};

use advent_of_code_2021::{
    day1::{puzzle1::count_depths, puzzle2::count_depths_windowed},
    day2::{puzzle1::determine_depth_and_distance, puzzle2::determine_depth_and_distance_with_aim},
};

fn main() {
    println!("{}", config_file("day1-test.txt").display());
    // Day 1
    println!(
        "D1P1: Depth Increases: {}",
        count_depths(config_file("day1.txt"))
    );
    println!(
        "D1P2: Depth Increases (3-measure window): {}",
        count_depths_windowed(config_file("day1.txt"), 3)
    );

    // Day 2
    let result = determine_depth_and_distance(config_file("day2.txt"));
    println!(
        "D2P1 New Coordinates: Distance: {}, Depth: {}; Multiplied: {}",
        result.distance,
        result.depth,
        result.distance as i32 * result.depth
    );
    let aim_result = determine_depth_and_distance_with_aim(config_file("day2.txt"));
    println!(
        "D2P1 New Coordinates: Distance: {}, Depth: {}; Multiplied: {}",
        aim_result.distance,
        aim_result.depth,
        aim_result.distance as i32 * aim_result.depth
    );
}

fn config_file(filename: &str) -> PathBuf {
    let basepathstr = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let base = Path::new(&basepathstr);
    base.join(filename)
}
