use std::path::PathBuf;

use aoclib::read_lines;

use crate::{
    data_file,
    day3::{puzzle1::number_houses_received_present, puzzle2::number_houses_received_present_robo},
};

pub mod puzzle1;
pub mod puzzle2;

pub fn run() {
    println!(
        "D3P1: Santa has dropped at least one gift at {} houses.",
        number_houses_received_present(data_file("day3.txt"))
    );
    println!(
        "D3P2: Santa and Robosanta have dropped at least one gift at {} houses.",
        number_houses_received_present_robo(data_file("day3.txt"))
    );
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub fn get_directions(filename: PathBuf) -> Vec<Direction> {
    let mut result = Vec::<Direction>::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            result.append(&mut decode_directions(
                line.expect("Failed to read line.").as_str(),
            ));
        }
    }
    result
}

fn decode_directions(line: &str) -> Vec<Direction> {
    line.chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day3::{decode_directions, Direction};

    #[test]
    fn decode_directions_works_correctly() {
        struct TestData {
            payload: String,
            result: Vec<Direction>,
        }
        let data = vec![
            TestData {
                payload: ">".to_string(),
                result: vec![Direction::East],
            },
            TestData {
                payload: "^>v<".to_string(),
                result: vec![
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ],
            },
            TestData {
                payload: "^v^v^v^v^v".to_string(),
                result: vec![
                    Direction::North,
                    Direction::South,
                    Direction::North,
                    Direction::South,
                    Direction::North,
                    Direction::South,
                    Direction::North,
                    Direction::South,
                    Direction::North,
                    Direction::South,
                ],
            },
        ];

        for d in data {
            assert_eq!(
                d.result,
                decode_directions(d.payload.as_str()),
                "The `{}` case failed.",
                d.payload
            );
        }
    }
}
