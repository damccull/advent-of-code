use std::path::PathBuf;

use super::{extract_movement, MovementInstruction, MovementResult};

pub fn determine_depth_and_distance_with_aim(filename: PathBuf) -> MovementResult {
    let instructions = extract_movement(filename);
    instructions
        .iter()
        .fold(MovementResult::new(), |a, i| match i {
            MovementInstruction::Forward(distance) => MovementResult {
                aim: a.aim,
                distance: a.distance + distance,
                depth: a.depth + (a.aim * distance.to_owned() as i32),
            },
            MovementInstruction::Up(aim) => MovementResult {
                aim: a.aim - aim.to_owned() as i32,
                distance: a.distance,
                depth: a.depth,
            },
            MovementInstruction::Down(aim) => MovementResult {
                aim: a.aim + aim.to_owned() as i32,
                distance: a.distance,
                depth: a.depth,
            },
        })
}
