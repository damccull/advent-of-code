use std::path::{Path, PathBuf};

use advent_of_code_2015::{
    day1::{puzzle1::what_floor, puzzle2::what_floor_enters_basement},
    day2::{puzzle1::get_wrapping_paper_sqft, puzzle2::get_ribbon_length},
};

fn main() {
    day1();
    day2();
}

fn data_file(filename: &str) -> PathBuf {
    let basepathstr = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let base = Path::new(&basepathstr);
    base.join(filename)
}

fn day1() {
    println!(
        "D1P1: Santa should be on floor {}.",
        what_floor(data_file("day1.txt"))
    );
    println!(
        "D1P2: Santa enters the basement on step {}.",
        what_floor_enters_basement(data_file("day1.txt"))
    );
}

fn day2() {
    println!(
        "D2P1: The elves need to order {} square feet of wrapping paper.",
        get_wrapping_paper_sqft(data_file("day2.txt"))
    );
    println!(
        "D2P2: The elves need to order {} feet of ribbon.",
        get_ribbon_length(data_file("day2.txt"))
    );
}
