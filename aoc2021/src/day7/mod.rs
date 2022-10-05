use std::path::PathBuf;

use aoclib::read_lines;

pub mod puzzle1;
pub mod puzzle2;

pub fn get_data(filename: PathBuf) -> Vec<usize> {
    let mut r = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let split = line.split(',').collect::<Vec<_>>();
            for crab in split {
                r.push(crab.parse::<usize>().unwrap());
            }
        }
    }
    r
}
