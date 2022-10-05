use std::path::PathBuf;

use aoclib::read_lines;



pub mod puzzle1;
pub mod puzzle2;



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
