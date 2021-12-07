use std::path::PathBuf;

use crate::{data_file, read_lines};

use self::puzzle1::find_minimum_fuel;

pub mod puzzle1;
pub mod puzzle2;

pub fn run() {
    let x = find_minimum_fuel(&get_data(data_file("day7.txt")));
    println!(
        "D7P1: The minimum fuel required to align the crabsubs is: {}",
        x
    );
}

fn get_data(filename: PathBuf) -> Vec<usize> {
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
