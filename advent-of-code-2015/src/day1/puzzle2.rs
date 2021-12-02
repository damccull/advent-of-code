use std::path::PathBuf;

use super::{parse_directions, Direction};

pub fn what_floor_enters_basement(filename: PathBuf) -> i32 {
    let directions = parse_directions(filename);
    calculate_basement_position(directions)
}

fn calculate_basement_position(directions: Vec<Direction>) -> i32 {
    let mut pos = 0;
    let mut fuse = true;
    let mut acc = 0;
    for d in directions {
        match d {
            Direction::Up => acc += 1,
            Direction::Down => acc -= 1,
        };
        if fuse {
            pos += 1;
        }
        if acc <= -1 {
            fuse = false;
        }
    }
    pos
}

#[cfg(test)]
mod test {
    use crate::day1::{parse_line, puzzle2::calculate_basement_position};

    #[test]
    fn calculate_basement_position_returns_correct_position() {
        struct TestData {
            directions: String,
            result: i32,
        }

        let data = vec![
            TestData {
                directions: ")".to_string(),
                result: 1,
            },
            TestData {
                directions: "()())".to_string(),
                result: 5,
            },
            TestData {
                directions: "(())()(())))))".to_string(),
                result: 11,
            },
        ];

        for d in data {
            assert_eq!(
                d.result,
                calculate_basement_position(parse_line(d.directions.clone())),
                "Test case `{}` failed.",
                d.directions
            );
        }
    }
}
