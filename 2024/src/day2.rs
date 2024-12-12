#[allow(unused_imports)]
use advent_of_code_common::read_data_from_file;
use anyhow::Result;
use nom::{
    character::complete::{digit1, line_ending, space0},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::tuple,
    Finish, IResult,
};

fn main() -> Result<(), anyhow::Error> {
    let data = include_str!("../data/day2.txt");
    let result = crate::puzzle1(data).unwrap();
    println!("Day 2 Puzzle 1: {}", result);

    let result = crate::puzzle2(data).unwrap();
    println!("Day 2 Puzzle 2: {}", result);
    Ok(())
}

fn puzzle1(input: &'static str) -> Result<usize, anyhow::Error> {
    let (_, reports) = read_reports(input).finish()?;

    let result = reports
        .into_iter()
        .filter(|report| {
            let mut all_desc: bool = true;
            let mut all_asc: bool = true;
            let mut all_in_tolerance: bool = true;
            for v in report.windows(2) {
                if v[0] < v[1] {
                    all_desc = false;
                    let diff = v[1] - v[0];
                    if !(1..=3).contains(&diff) {
                        all_in_tolerance = false;
                    }
                }
                if v[0] > v[1] {
                    all_asc = false;
                    let diff = v[0] - v[1];
                    if !(1..=3).contains(&diff) {
                        all_in_tolerance = false;
                    }
                }
                if v[0] == v[1] {
                    all_asc = false;
                    all_desc = false;
                }
                if !all_in_tolerance || (!all_asc && !all_desc) {
                    return false;
                }
            }
            true
        })
        .count();
    Ok(result)
}

fn puzzle2(input: &'static str) -> Result<usize, anyhow::Error> {
    let (_, reports) = read_reports(input).finish()?;
    dbg!(&reports.len());
    let result = reports
        .into_iter()
        .filter(|report| {
            // Get trend
            // Test for opposite directions
            // Test for equalities
            // Test for tolerances
            // For each case, return the index to the bad level
            // Create a new Vec without that bad slice and test again
            // If still fails, bad report
            let trend = match get_trend(&report) {
                Some(t) => t,
                None => {
                    return false;
                }
            };

            if let Validity::Invalid(i) = check_for_validity(&trend, report) {
                // Bad once, remove offending level and try again
                let error_corrected_report = {
                    let mut corrected = report.clone();
                    corrected.remove(i);
                    corrected
                };
                if let Validity::Invalid(_v) = check_for_validity(&trend, &error_corrected_report) {
                    // Bad twice, naughty list
                    false
                } else {
                    true
                }
            } else {
                true
            }
        })
        .count();
    Ok(result)
}

enum Validity {
    Valid,
    Invalid(usize),
}

fn check_for_validity(trend: &Trend, report: &[i32]) -> Validity {
    for (i, win) in report.windows(2).enumerate() {
        match win[0].cmp(&win[1]) {
            std::cmp::Ordering::Less => match trend {
                Trend::Ascending => {
                    if !(1..=3).contains(&(win[1] - win[0])) {
                        return Validity::Invalid(i + 1);
                    }
                }
                Trend::Descending => return Validity::Invalid(i + 1),
            },
            std::cmp::Ordering::Equal => return Validity::Invalid(i + 1),
            std::cmp::Ordering::Greater => match trend {
                Trend::Ascending => return Validity::Invalid(i + 1),
                Trend::Descending => {
                    if !(1..=3).contains(&(win[0] - win[1])) {
                        return Validity::Invalid(i + 1);
                    }
                }
            },
        }
    }
    Validity::Valid
}

#[derive(Debug)]
enum Trend {
    Ascending,
    Descending,
}

fn get_trend(report: &[i32]) -> Option<Trend> {
    if report.is_empty() {
        return None;
    }

    let mut ascending_pairs = 0;
    let mut descending_pairs = 0;

    for i in report.windows(2) {
        match i[0].cmp(&i[1]) {
            std::cmp::Ordering::Less => {
                ascending_pairs += 1;
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {
                descending_pairs += 1;
            }
        }
    }

    match ascending_pairs.cmp(&descending_pairs) {
        std::cmp::Ordering::Less => Some(Trend::Descending),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => Some(Trend::Ascending),
    }
}

fn read_reports(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, reports) = separated_list1(line_ending, get_levels)(input)?;

    Ok((input, reports))
}

fn get_levels(input: &str) -> IResult<&str, Vec<i32>> {
    let pairs = tuple((map_res(digit1, |s: &str| s.parse::<i32>()), space0));
    let (remaining, input) = many1(pairs)(input)?;
    let result = input.iter().map(|(x, _)| *x).collect::<Vec<_>>();
    Ok((remaining, result))
}

#[cfg(test)]
mod tests {

    #[test]
    fn puzzle1() {
        let data = include_str!("../data/d2p1-test.txt");
        let result = crate::puzzle1(data);
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn puzzle2() {
        let data = include_str!("../data/d2p2-test.txt");
        let result = crate::puzzle2(data);
        assert_eq!(result.unwrap(), 4);
    }
}
