use std::path::PathBuf;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

// Below is code that is not part of the puzzle itself, but that supports it.
// Example: code to load data from a file.

pub fn data_file(filename: &str) -> PathBuf {
    aoclib::data_file(env!("CARGO_MANIFEST_DIR"), filename)
}
