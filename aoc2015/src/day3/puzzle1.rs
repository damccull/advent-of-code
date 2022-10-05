use std::{collections::HashMap, path::PathBuf};

use aoclib::Point;

use super::get_directions;

pub fn number_houses_received_present(filename: PathBuf) -> u32 {
    // Get the data
    let directions = get_directions(filename);
    // Set up storage for visited houses
    let mut houses = HashMap::new();
    // Set the current coordinate to origin and insert it immediately because
    // instructions say that the origin is the first delivery
    let mut current_coord = Point { x: 0, y: 0 };
    houses.insert(current_coord, 1);

    // Set up some coordinate deltas for each direction
    let move_north = Point { x: 0, y: 1 };
    let move_south = Point { x: 0, y: -1 };
    let move_east = Point { x: 1, y: 0 };
    let move_west = Point { x: -1, y: 0 };

    for d in directions {
        // Create a new coordinate to use for the hashmap
        let new_coord = match d {
            super::Direction::North => current_coord + move_north,
            super::Direction::East => current_coord + move_east,
            super::Direction::South => current_coord + move_south,
            super::Direction::West => current_coord + move_west,
        };
        // Insert a new house with 1 gift or, if it already exists, add another gift to it
        let house = houses.entry(new_coord).or_insert(1);
        *house += 1;

        current_coord = new_coord;
    }
    houses.len() as u32
}
