use advent_of_code_common::read_data_from_file;
use anyhow::Context;

fn main() -> Result<(), anyhow::Error> {
    let data = process_data(read_data_from_file("data/day4.txt")?)?;
    let result = number_fully_overlapping_pairs(data.clone())?;
    println!(
        "The number of pairs that are fully overlapping is {}",
        result
    );

    let result = number_overlapping_pairs(data)?;
    println!("The number of pairs that have any overlap is {}", result);
    Ok(())
}

fn number_overlapping_pairs(data: Vec<AssignmentPair>) -> Result<u32, anyhow::Error> {
    data.iter()
        .filter(|e| e.left.overlaps_with(&e.right))
        .count()
        .try_into()
        .context("Unable to convert usize to u32")
}

fn number_fully_overlapping_pairs(data: Vec<AssignmentPair>) -> Result<u32, anyhow::Error> {
    data.iter()
        .filter(|e| e.left.fully_contains(&e.right) || e.right.fully_contains(&e.left))
        .count()
        .try_into()
        .context("Unable to convert usize to u32")
}

fn process_data(pairs: Vec<String>) -> Result<Vec<AssignmentPair>, anyhow::Error> {
    pairs
        .iter()
        .map(|line| {
            let (left, right) = line
                .split_once(',')
                .ok_or_else(|| anyhow::anyhow!("Bad data in the file"))?;

            let (left_beginning, left_end) = left
                .split_once('-')
                .ok_or_else(|| anyhow::anyhow!("Bad data in the file"))?;

            let (right_beginning, right_end) = right
                .split_once('-')
                .ok_or_else(|| anyhow::anyhow!("Bad data in the file"))?;

            let left_beginning = left_beginning.parse()?;
            let left_end = left_end.parse()?;
            let right_beginning = right_beginning.parse()?;
            let right_end = right_end.parse()?;

            Ok(AssignmentPair::new(
                Assignment::new(left_beginning, left_end),
                Assignment::new(right_beginning, right_end),
            ))
        })
        .collect::<Result<Vec<_>, _>>()
}

#[derive(Debug, Clone, Copy)]
struct AssignmentPair {
    left: Assignment,
    right: Assignment,
}

impl AssignmentPair {
    pub fn new(left: Assignment, right: Assignment) -> Self {
        Self { left, right }
    }
}

#[derive(Debug, Clone, Copy)]
struct Assignment {
    pub beginning: u32,
    pub end: u32,
}

impl Assignment {
    pub fn new(beginning: u32, end: u32) -> Self {
        Self { beginning, end }
    }
    pub fn overlaps_with(&self, other: &Assignment) -> bool {
        if self.beginning >= other.beginning && self.beginning <= other.end
            || self.end >= other.beginning && self.end <= other.end
            || other.beginning >= self.beginning && other.beginning <= self.end
            || other.end >= self.beginning && other.end <= self.end
        {
            return true;
        }

        false
    }

    pub fn fully_contains(&self, other: &Assignment) -> bool {
        if self.beginning >= other.beginning
            && self.beginning <= other.end
            && self.end >= other.beginning
            && self.end <= other.end
        {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{process_data, Assignment, AssignmentPair};

    fn test_data_string() -> Vec<String> {
        r##"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"##
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn process_data_works_correctly() {
        let data = process_data(test_data_string()).unwrap();

        assert!(!data.is_empty());
    }

    #[test]
    fn assignment_overlaps_with_is_correct() {
        let data = vec![
            (
                AssignmentPair::new(Assignment::new(2, 4), Assignment::new(6, 8)),
                false,
            ),
            (
                AssignmentPair::new(Assignment::new(2, 3), Assignment::new(4, 5)),
                false,
            ),
            (
                AssignmentPair::new(Assignment::new(5, 7), Assignment::new(7, 9)),
                true,
            ),
            (
                AssignmentPair::new(Assignment::new(2, 8), Assignment::new(3, 7)),
                true,
            ),
            (
                AssignmentPair::new(Assignment::new(6, 6), Assignment::new(4, 6)),
                true,
            ),
            (
                AssignmentPair::new(Assignment::new(2, 6), Assignment::new(4, 8)),
                true,
            ),
        ];
        for (assignment, answer) in data {
            assert_eq!(assignment.left.overlaps_with(&assignment.right), answer);
        }
    }

    #[test]
    fn assignment_fully_contains_is_corrent() {
        let data = vec![
            (
                AssignmentPair::new(Assignment::new(2, 4), Assignment::new(6, 8)),
                false,
            ),
            (
                AssignmentPair::new(Assignment::new(2, 3), Assignment::new(4, 5)),
                false,
            ),
            (
                AssignmentPair::new(Assignment::new(5, 7), Assignment::new(7, 9)),
                false,
            ),
            (
                AssignmentPair::new(Assignment::new(2, 8), Assignment::new(3, 7)),
                true,
            ),
            (
                AssignmentPair::new(Assignment::new(6, 6), Assignment::new(4, 6)),
                true,
            ),
            (
                AssignmentPair::new(Assignment::new(2, 6), Assignment::new(4, 8)),
                false,
            ),
        ];

        for (pair, answer) in data {
            let result =
                pair.left.fully_contains(&pair.right) || pair.right.fully_contains(&pair.left);
            assert_eq!(result, answer, "Incorrect: {:?}", pair);
        }
    }
}
