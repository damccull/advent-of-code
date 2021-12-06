use std::path::PathBuf;

use super::{extract_movement, MovementInstruction, MovementResult};

pub fn determine_depth_and_distance(filename: PathBuf) -> MovementResult {
    // Get a vector of `MovementInstruction`s.
    let instructions = extract_movement(filename);

    // Iterate over the vector and fold each event into the accumulator (a new `MovementResult`)
    // Folding just means to apply something to a static state in the order its received:
    // * State: 0; Add 1;
    // * State: 1; Subtract 3;
    // * State: -2;
    //NOTE: The `aim` parameter isn't used at all in the puzzle. It's for puzzle 2.
    instructions.iter().fold(MovementResult::new(), |a, i| {
        // Figure out which variant of the `MovementInstruction` enum we're dealing with
        match i {
            // If `Forward`
            MovementInstruction::Forward(distance) => MovementResult {
                aim: 0,
                // Increase the distance
                distance: a.distance + distance,
                depth: a.depth,
            },
            // If `Up`
            MovementInstruction::Up(depth) => MovementResult {
                aim: 0,
                distance: a.distance,
                // Decrease the depth
                depth: a.depth - depth.to_owned() as i32,
            },
            // If `Down`
            MovementInstruction::Down(depth) => MovementResult {
                aim: 0,
                distance: a.distance,
                // Increase the depth
                depth: a.depth + depth.to_owned() as i32,
            },
        }
    })
}
