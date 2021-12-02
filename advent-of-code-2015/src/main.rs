use std::path::{Path, PathBuf};

use advent_of_code_2015::day1::{puzzle1::what_floor, puzzle2::what_floor_enters_basement};

fn main() {
    println!(
        "D1P1: Santa should be on floor {}.",
        what_floor(config_file("input.txt"))
    );
    println!(
        "D1P2: Santa enters the basement on step {}.",
        what_floor_enters_basement(config_file("input.txt"))
    );
}

fn config_file(filename: &str) -> PathBuf {
    let basepathstr = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let base = Path::new(&basepathstr);
    base.join(filename)
}
