use advent_of_code::read_data_from_file;

fn main() -> Result<(), anyhow::Error> {
    let data = read_data_from_file("data/2015-day1.txt".to_string())?;
    let directions = data
        .first()
        .ok_or_else(|| anyhow::anyhow!("Can't read data from file"))?;
    let (floor, index) = get_floor_from_directions(directions.as_str())?;

    println!("Santa ends up on floor {}.", floor);

    match index {
        Some(x) => println!(
            "Santa entered the basement when he followed character {} in the directions.",
            x
        ),
        None => println!("The directions do not include instructions to go to the basement."),
    }
    Ok(())
}

fn get_floor_from_directions(directions: &str) -> Result<(i32, Option<usize>), anyhow::Error> {
    let x: (i32, Option<usize>) = directions
        .chars()
        .into_iter()
        .map(|c| c.try_into())
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .enumerate()
        .fold((0, None), |(a, i), (index, v)| {
            let x = match v {
                Direction::Up => a + 1,
                Direction::Down => a - 1,
            };

            let t: Option<usize> = if i.is_none() && x < 0 {
                Some(index + 1)
            } else {
                i
            };

            (x, t)
        });
    Ok(x)
}

enum Direction {
    Up,
    Down,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::Up),
            ')' => Ok(Self::Down),
            _ => Err(anyhow::anyhow!("Bad direction; check file for corruption")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::get_floor_from_directions;

    #[test]
    fn get_floor_from_directions_returns_correct_floor() -> Result<(), anyhow::Error> {
        // Arrange
        let tests = vec![
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];

        // Act and Assert

        for (test, floor) in tests {
            let result = get_floor_from_directions(test)?;
            assert_eq!(result.0, floor, "Broken test was `{}`", test);
        }
        Ok(())
    }

    #[test]
    fn get_floor_from_directions_returns_correct_index() -> Result<(), anyhow::Error> {
        // Arrange
        let tests = vec![(")", Some(1)), ("()())", Some(5)), ("((((", None)];

        // Act and Assert

        for (test, index) in tests {
            let result = get_floor_from_directions(test)?;
            assert_eq!(result.1, index, "Broken test was `{}`", test);
        }
        Ok(())
    }

    #[test]
    fn get_floor_from_directions_returns_error_for_bad_input() -> Result<(), anyhow::Error> {
        // Arrange
        let test = "(()))(+)(";

        // Act
        let result = get_floor_from_directions(test);

        // Assert
        assert!(result.is_err());

        Ok(())
    }
}
