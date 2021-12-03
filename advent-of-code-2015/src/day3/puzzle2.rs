use std::{collections::HashMap, path::PathBuf};

use super::{get_directions, Coordinate};

pub fn number_houses_received_present_robo(filename: PathBuf) -> u32 {
    // Get the data
    let directions = get_directions(filename);
    // Set up storage for visited houses
    let mut houses = HashMap::new();
    // Set the current coordinate to origin and insert it immediately because
    // instructions say that the origin is the first delivery.
    // Do this for both santa and robosanta.
    let mut current_coord_santa = Coordinate(0, 0);
    let mut current_coord_robosanta = Coordinate(0, 0);
    houses.insert(current_coord_santa.clone(), 2);

    // Set up some coordinate deltas for each direction
    let move_north = Coordinate(0, 1);
    let move_south = Coordinate(0, -1);
    let move_east = Coordinate(1, 0);
    let move_west = Coordinate(-1, 0);

    for (i, d) in directions.iter().enumerate() {
        // Create a new coordinate to use for the hashmap for santa
        let new_coord = if i % 2 == 0 {
            match d {
                super::Direction::North => &current_coord_santa + &move_north,
                super::Direction::East => &current_coord_santa + &move_east,
                super::Direction::South => &current_coord_santa + &move_south,
                super::Direction::West => &current_coord_santa + &move_west,
            }
        } else {
            // Create a new coordinate to use for the hashmap for robosanta
            match d {
                super::Direction::North => &current_coord_robosanta + &move_north,
                super::Direction::East => &current_coord_robosanta + &move_east,
                super::Direction::South => &current_coord_robosanta + &move_south,
                super::Direction::West => &current_coord_robosanta + &move_west,
            }
        };
        // Insert a new house with 1 gift or, if it already exists, add another gift to it
        let house = houses.entry(new_coord.clone()).or_insert(1);
        *house += 1;

        if i % 2 == 0 {
            current_coord_santa = new_coord;
        } else {
            current_coord_robosanta = new_coord;
        }
    }
    houses.len() as u32
}
