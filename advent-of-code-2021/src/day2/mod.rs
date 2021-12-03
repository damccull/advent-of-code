use std::path::PathBuf;

use crate::{
    config_file,
    day2::{puzzle1::determine_depth_and_distance, puzzle2::determine_depth_and_distance_with_aim},
    read_lines,
};

pub mod puzzle1;
pub mod puzzle2;

pub fn run() {
    let result = determine_depth_and_distance(config_file("day2.txt"));
    println!(
        "D2P1: New Coordinates: Distance: {}, Depth: {}; Multiplied: {}",
        result.distance,
        result.depth,
        result.distance as i32 * result.depth
    );
    let aim_result = determine_depth_and_distance_with_aim(config_file("day2.txt"));
    println!(
        "D2P1: New Coordinates: Distance: {}, Depth: {}; Multiplied: {}",
        aim_result.distance,
        aim_result.depth,
        aim_result.distance as i32 * aim_result.depth
    );
}

#[derive(Debug)]
pub struct MovementResult {
    pub aim: i32,
    pub distance: u32,
    pub depth: i32,
}
impl MovementResult {
    pub fn new() -> Self {
        Self {
            aim: 0,
            distance: 0,
            depth: 0,
        }
    }
}
impl Default for MovementResult {
    fn default() -> Self {
        Self::new()
    }
}

pub enum MovementInstruction {
    Forward(u32),
    Up(u32),
    Down(u32),
}
impl From<String> for MovementInstruction {
    fn from(instruction: String) -> Self {
        let instruction = instruction.split(' ').collect::<Vec<_>>();
        let distance = instruction[1].parse().expect("Unable to parse distance");
        match instruction[0] {
            "forward" => MovementInstruction::Forward(distance),
            "up" => MovementInstruction::Up(distance),
            "down" => MovementInstruction::Down(distance),
            _ => panic!("Unable to parse movement instruction."),
        }
    }
}

pub fn extract_movement(filename: PathBuf) -> Vec<MovementInstruction> {
    let mut result = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consume iterator, return an (Optional) String
        for line in lines.flatten() {
            result.push(line.into());
        }
    }
    result
}
