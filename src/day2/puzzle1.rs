use super::{extract_movement, MovementInstruction, MovementResult};

pub fn determine_depth_and_distance(filename: &str) -> MovementResult {
    let instructions = extract_movement(filename);
    instructions
        .iter()
        .fold(MovementResult::new(), |a, i| match i {
            MovementInstruction::Forward(distance) => MovementResult {
                aim: 0,
                distance: a.distance + distance,
                depth: a.depth,
            },
            MovementInstruction::Up(depth) => MovementResult {
                aim: 0,
                distance: a.distance,
                depth: a.depth - depth.to_owned() as i32,
            },
            MovementInstruction::Down(depth) => MovementResult {
                aim: 0,
                distance: a.distance,
                depth: a.depth + depth.to_owned() as i32,
            },
        })
}
