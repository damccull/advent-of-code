pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

// Below is code that is not part of the puzzle itself, but that supports it.
// Example: code to load data from a file.

use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn data_file(filename: &str) -> PathBuf {
    let basepathstr = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let base = Path::new(&basepathstr);
    base.join(filename)
}
