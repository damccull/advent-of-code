pub mod puzzle1;
pub mod puzzle2;

use std::path::PathBuf;

use crate::read_lines;

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
