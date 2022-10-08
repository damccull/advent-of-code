use advent_of_code::read_data_from_file;

fn main() -> Result<(), anyhow::Error> {
    let data = read_data_from_file("data/2015-day1.txt".to_string())?;
    let directions = data
        .first()
        .ok_or_else(|| anyhow::anyhow!("Can't read data from file"))?;
    let result = get_floor_from_directions(directions.as_str());

    println!("Santa ends up on floor {}.", result);
    Ok(())
}

fn get_floor_from_directions(directions: &str) -> i32 {
    directions.chars().into_iter().fold(0, |a, v| match v {
        '(' => a + 1,
        ')' => a - 1,
        _ => a,
    })
}

#[cfg(test)]
mod tests {
    use crate::get_floor_from_directions;

    #[test]
    fn get_floor_from_directions_returns_correct_floor() {
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
            let result = get_floor_from_directions(test);
            assert_eq!(result, floor, "Broken test was `{}`", test);
        }
    }
}
