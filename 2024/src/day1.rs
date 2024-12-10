#[allow(unused_imports)]
use advent_of_code_common::read_data_from_file;
use anyhow::Result;
use nom::{
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    Finish, IResult,
};

fn main() -> Result<()> {
    let data = include_str!("../data/day1.txt");
    let result = crate::puzzle1(data).unwrap();
    println!("Day 1 Puzzle 1: {}", result);

    let result = crate::puzzle2(data).unwrap();
    println!("Day 1 Puzzle 2: {}", result);
    Ok(())
}

fn puzzle1(input: &'static str) -> Result<i32, anyhow::Error> {
    let (mut nums1, mut nums2) = read_lines(input)?;

    nums1.sort();
    nums2.sort();

    assert_eq!(nums1.len(), nums2.len());

    let mut results = Vec::new();

    for (k, v) in nums1.iter().enumerate() {
        let u = nums2[k];
        if u > *v {
            results.push(u - v);
        } else {
            results.push(v - u);
        }
    }

    let result = results.iter().sum();

    Ok(result)
}

fn puzzle2(input: &'static str) -> Result<i32, anyhow::Error> {
    let (nums1, nums2) = read_lines(input)?;

    assert_eq!(&nums1.len(), &nums2.len());

    let mut results = Vec::new();

    for v in nums1.iter() {
        let r = nums2.iter().filter(|a| **a == *v).count();

        let r: i32 = r as i32;

        results.push(v * r);
    }
    let result = results.iter().sum();

    Ok(result)
}

fn read_lines(input: &'static str) -> Result<(Vec<i32>, Vec<i32>)> {
    let (_input, nums) = separated_list1(line_ending, get_nums)(input).finish()?;

    let (nums1, nums2) = nums.into_iter().unzip();

    Ok((nums1, nums2))
}

fn get_nums(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, (num1, _, num2)) = tuple((
        map_res(digit1, |s: &str| s.parse::<i32>()),
        space1,
        map_res(digit1, |s: &str| s.parse::<i32>()),
    ))(input)?;
    Ok((input, (num1, num2)))
}

#[cfg(test)]
mod tests {

    use anyhow::Result;

    #[test]
    fn puzzle1() -> Result<()> {
        let data = include_str!("../data/d1p1-test.txt");
        let result = crate::puzzle1(data);
        assert_eq!(result.unwrap(), 11);
        Ok(())
    }

    #[test]
    fn puzzle2() {
        let data = include_str!("../data/d1p2-test.txt");
        let result = crate::puzzle2(data);
        assert_eq!(result.unwrap(), 31);
    }
}
