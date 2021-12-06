use std::path::PathBuf;

use super::{parse_directions, Direction};

pub fn what_floor(filename: PathBuf) -> i32 {
    let directions = parse_directions(filename);
    calculate_floor(directions)
}

fn calculate_floor(directions: Vec<Direction>) -> i32 {
    directions
        .iter()
        .fold(0_i32, |acc, direction| match direction {
            Direction::Up => acc + 1,
            Direction::Down => acc - 1,
        })
}

#[cfg(test)]
mod test {
    use crate::day1::{parse_line, puzzle1::calculate_floor};

    #[test]
    fn calculate_floor_returns_correct_floor() {
        struct TestData {
            directions: String,
            result: i32,
        }

        let data = vec![
            TestData {
                directions: "(())".to_string(),
                result: 0,
            },
            TestData {
                directions: "()()".to_string(),
                result: 0,
            },
            TestData {
                directions: "(((".to_string(),
                result: 3,
            },
            TestData {
                directions: "(()(()(".to_string(),
                result: 3,
            },
            TestData {
                directions: "))(((((".to_string(),
                result: 3,
            },
            TestData {
                directions: "())".to_string(),
                result: -1,
            },
            TestData {
                directions: "))(".to_string(),
                result: -1,
            },
            TestData {
                directions: ")))".to_string(),
                result: -3,
            },
            TestData {
                directions: ")())())".to_string(),
                result: -3,
            },
        ];

        for d in data {
            assert_eq!(
                d.result,
                calculate_floor(parse_line(d.directions.clone())),
                "Test case `{}` failed.",
                d.directions
            );
        }
    }
}
