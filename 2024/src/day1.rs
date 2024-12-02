#[allow(unused_imports)]
use advent_of_code_common::read_data_from_file;
use nom::{
    character::complete::{digit1, space1},
    combinator::map_res,
    sequence, IResult,
};

fn main() -> Result<(), anyhow::Error> {
    let data = include_str!("../data/day1.txt");
    let result = puzzle1(data).unwrap();
    println!("Day 1 Puzzle 1: {}", result);

    let result = puzzle2(data).unwrap();
    println!("Day 1 Puzzle 2: {}", result);
    Ok(())
}

fn puzzle1(_input: &str) -> Result<u32, anyhow::Error> {
    let result: u32 = 0;

    Ok(result)
}

fn puzzle2(_input: &str) -> Result<u32, anyhow::Error> {
    let result: u32 = 0;

    Ok(result)
}

fn parse_integer(_input: &str) -> IResult<&str, i32> {
    map_res(sequence::pair((digit1, space1)), |(digits, space2)| {
        digits.parse::<i32>()
    })(input)
}

#[cfg(test)]
mod tests {
    use crate::{puzzle1, puzzle2};

    #[test]
    fn day1() {
        let data = include_str!("../data/d1p1-test.txt");
        let result = puzzle1(data);
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn day2() {
        let data = include_str!("../data/d1p2-test.txt");
        let result = puzzle2(data);
        assert_eq!(result.unwrap(), 281);
    }
}
