pub mod puzzle1;
pub mod puzzle2;

use crate::{data_file, day5::puzzle1::find_nice_strings};

pub fn run() {
    println!(
        "D5P1: The number of nice strings is {}.",
        find_nice_strings(data_file("day5.txt"))
    );
}
