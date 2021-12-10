use std::path::PathBuf;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day9;
pub mod day10;

pub fn data_file(filename: &str) -> PathBuf {
    aoclib::data_file(env!("CARGO_MANIFEST_DIR"), filename)
}
