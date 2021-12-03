pub mod puzzle1;
pub mod puzzle2;

use std::path::PathBuf;

use crate::{
    data_file,
    day1::{puzzle1::what_floor, puzzle2::what_floor_enters_basement},
    read_lines,
};

pub fn run() {
    println!(
        "D1P1: Santa should be on floor {}.",
        what_floor(data_file("day1.txt"))
    );
    println!(
        "D1P2: Santa enters the basement on step {}.",
        what_floor_enters_basement(data_file("day1.txt"))
    );
}

pub enum Direction {
    Up,
    Down,
}

pub fn parse_directions(filename: PathBuf) -> Vec<Direction> {
    let mut directions = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consume iterator, return an (Optional) String
        for line in lines.flatten() {
            directions.append(&mut parse_line(line));
        }
    }
    directions
}

pub fn parse_line(input: String) -> Vec<Direction> {
    let mut directions = Vec::new();
    for c in input.chars() {
        match c {
            '(' => directions.push(Direction::Up),
            ')' => directions.push(Direction::Down),
            _ => {}
        }
    }

    directions
}
