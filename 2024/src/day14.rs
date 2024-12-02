#[allow(unused_imports)]
use advent_of_code_common::read_data_from_file;

fn main() -> Result<(), anyhow::Error> {
    let data = include_str!("../data/day14.txt");
    let result = puzzle1(data).unwrap();
    println!("Day 14 Puzzle 1: {}", result);

    let result = puzzle2(data).unwrap();
    println!("Day 14 Puzzle 2: {}", result);
    Ok(())
}

fn puzzle1(input: &str) -> Result<u32, anyhow::Error> {
    todo!()
}

fn puzzle2(input: &str) -> Result<u32, anyhow::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{puzzle1, puzzle2};

    #[test]
    fn day1() {
        let data = include_str!("../data/d14p1-test.txt");
        let result = puzzle1(data);
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn day2() {
        let data = include_str!("../data/d14p2-test.txt");
        let result = puzzle2(data);
        assert_eq!(result.unwrap(), 281);
    }
}
