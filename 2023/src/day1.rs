use std::collections::{BTreeMap, HashMap};

#[allow(unused_imports)]
use advent_of_code_common::read_data_from_file;

fn main() -> Result<(), anyhow::Error> {
    let data = include_str!("../data/day1.txt");
    let result = puzzle1(data).unwrap();
    println!("Day 1 Puzzle 1: {}", result);

    let result = puzzle2(data).unwrap();
    println!("Day 2 Puzzle 2: {}", result);
    Ok(())
}

fn puzzle1(input: &str) -> Result<u32, anyhow::Error> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let x = lines
        .iter()
        .filter(|&line| !line.is_empty())
        .map(|&line| {
            let y = line
                .chars()
                .into_iter()
                .filter_map(|c| c.to_string().parse::<u32>().ok())
                .collect::<Vec<_>>();
            y[0] * 10 + y[y.len() - 1]
        })
        .sum();

    Ok(x)
}

fn puzzle2(input: &str) -> Result<u32, anyhow::Error> {
    let searches = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    let lines = input.split("\n").collect::<Vec<&str>>();

    let result: Result<u32, anyhow::Error> = lines
        .iter()
        .filter(|&line| !line.is_empty())
        .map(|&line| {
            let mut index: BTreeMap<usize, u32> = BTreeMap::new();
            for term in &searches {
                let i = line.find(&term.0);
                if let Some(i) = i {
                    index.insert(i, term.1);
                }
            }
            for term in &searches {
                let i = line.rfind(&term.0);
                if let Some(i) = i {
                    index.insert(i, term.1);
                }
            }
            let first = index
                .first_key_value()
                .ok_or_else(|| anyhow::anyhow!("No first digit"))?;
            let last = index
                .last_key_value()
                .ok_or_else(|| anyhow::anyhow!("No last digit"))?;
            println!("{}{}", &first.1, &last.1);
            Ok((first.1 * 10) + last.1)
        })
        .sum();
    result
}

#[cfg(test)]
mod tests {
    use crate::{puzzle1, puzzle2};

    #[test]
    fn day1() {
        let data = include_str!("../data/day1puzzle1-test.txt");
        let result = puzzle1(data);
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn day2() {
        let data = include_str!("../data/day1puzzle2-test.txt");
        let result = puzzle2(data);
        assert_eq!(result.unwrap(), 281);
    }
}
