use std::{collections::HashMap, path::PathBuf};

use super::{get_directions, Coordinate};

pub fn number_houses_received_present(filename: PathBuf) -> u32 {
    // Get the data
    let directions = get_directions(filename);
    // Set up storage for visited houses
    let mut houses = HashMap::new();
    // Set the current coordinate to origin and insert it immediately because
    // instructions say that the origin is the first delivery
    let mut current_coord = Coordinate(0, 0);
    houses.insert(current_coord.clone(), 1);

    // Set up some coordinate deltas for each direction
    let move_north = Coordinate(0, 1);
    let move_south = Coordinate(0, -1);
    let move_east = Coordinate(1, 0);
    let move_west = Coordinate(-1, 0);

    for d in directions {
        // Create a new coordinate to use for the hashmap
        let new_coord = match d {
            super::Direction::North => &current_coord + &move_north,
            super::Direction::East => &current_coord + &move_east,
            super::Direction::South => &current_coord + &move_south,
            super::Direction::West => &current_coord + &move_west,
        };
        // Insert a new house with 1 gift or, if it already exists, add another gift to it
        let house = houses.entry(new_coord.clone()).or_insert(1);
        *house += 1;

        current_coord = new_coord;
    }
    houses.len() as u32
}
