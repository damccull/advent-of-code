pub mod puzzle1;
pub mod puzzle2;

use std::path::PathBuf;

use aoclib::read_lines;



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
