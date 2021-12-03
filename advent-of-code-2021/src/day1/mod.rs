pub mod puzzle1;
pub mod puzzle2;

use std::path::PathBuf;

use crate::{
    config_file,
    day1::{puzzle1::count_depths, puzzle2::count_depths_windowed},
    read_lines,
};

pub fn run() {
    println!(
        "D1P1: Depth Increases: {}",
        count_depths(config_file("day1.txt"))
    );
    println!(
        "D1P2: Depth Increases (3-measure window): {}",
        count_depths_windowed(config_file("day1.txt"), 3)
    );
}

pub fn extract_depths(filename: PathBuf) -> Vec<i32> {
    let mut depths = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consume iterator, return an (Optional) String
        for line in lines.flatten() {
            depths.push(line.parse().expect("Failed to parse the file"));
        }
    }
    depths
}
