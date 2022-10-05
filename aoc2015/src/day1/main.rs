use aoc2015::{day1::{puzzle1::what_floor, puzzle2::what_floor_enters_basement}, data_file};

pub fn main() {
    println!(
        "D1P1: Santa should be on floor {}.",
        what_floor(data_file("day1.txt"))
    );
    println!(
        "D1P2: Santa enters the basement on step {}.",
        what_floor_enters_basement(data_file("day1.txt"))
    );
}