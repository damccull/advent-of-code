use advent_of_code_common::read_data_from_file;
use rayon::{
    prelude::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSlice,
};

fn main() -> Result<(), anyhow::Error> {
    let data = read_data_from_file("data/day3.txt")?;
    let data = process_data(data)?;
    let result = prioritize_duplicate_items(data.clone())?
        .iter()
        .sum::<u32>();

    println!("The total priority for the rucksack items is {}.", result);

    let result: u32 = find_group_badge_priorities(data)?.iter().sum();

    println!("The total priority for group badges is {}.", result);

    Ok(())
}

fn process_data(data: Vec<String>) -> Result<Vec<Rucksack>, anyhow::Error> {
    let rucks = data
        .par_iter()
        .map(|s| {
            let (left, right) = s.split_at(s.len() / 2);
            let mut ruck = Rucksack::default();
            ruck.left.append(&mut left.chars().collect::<Vec<char>>());
            ruck.right.append(&mut right.chars().collect::<Vec<char>>());

            ruck
        })
        .collect::<Vec<_>>();

    Ok(rucks)
}

#[derive(Debug, Default, Clone)]
struct Rucksack {
    left: Vec<char>,
    right: Vec<char>,
}

impl Rucksack {
    pub fn get_duplicate_items(&self) -> Option<char> {
        self.left
            .par_iter()
            .find_any(|item| self.right.contains(item))
            .cloned()
    }

    pub fn get_all_contents(&self) -> Vec<char> {
        let mut r = Vec::new();
        r.append(&mut self.left.clone());
        r.append(&mut self.right.clone());
        r
    }
}

fn prioritize_duplicate_items(rucks: Vec<Rucksack>) -> Result<Vec<u32>, anyhow::Error> {
    let result = rucks
        .par_iter()
        .flat_map(|r| r.get_duplicate_items())
        .map(convert_item_to_priority)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(result)
}

fn find_group_badge_priorities(rucks: Vec<Rucksack>) -> Result<Vec<u32>, anyhow::Error> {
    rucks
        .par_chunks(3)
        .map(|group_rucks| {
            let item = group_rucks[0]
                .get_all_contents()
                .iter()
                .find(|item| {
                    group_rucks[1].get_all_contents().contains(item)
                        && group_rucks[2].get_all_contents().contains(item)
                })
                .cloned();
            match item {
                Some(x) => convert_item_to_priority(x),
                None => anyhow::bail!("Unable to find badge"),
            }
        })
        .collect::<Result<Vec<u32>, anyhow::Error>>()
}

fn convert_item_to_priority(item: char) -> Result<u32, anyhow::Error> {
    let item = item as u32;

    if (97..=122).contains(&item) {
        return Ok(item - 97 + 1);
    }

    if (65..=90).contains(&item) {
        return Ok(item - 65 + 27);
    }

    anyhow::bail!("Item is not valid.");
}

#[cfg(test)]
mod tests {
    use crate::{convert_item_to_priority, find_group_badge_priorities, process_data, Rucksack};

    fn test_data() -> Vec<String> {
        let test_data = r##"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"##;
        test_data
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn find_group_badge_priorities_is_correct() -> Result<(), anyhow::Error> {
        // Arrange
        let data = process_data(test_data())?;
        let answers = vec![18u32, 52u32];

        // Act
        let result = find_group_badge_priorities(data)?;

        // Assert
        assert_eq!(result, answers);

        Ok(())
    }

    #[test]
    fn process_data_returns_correctly() -> Result<(), anyhow::Error> {
        // Arrange
        let data = test_data();

        // Act
        let result = process_data(data);

        // Assert
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());

        Ok(())
    }

    #[test]
    fn convert_item_to_priority_returns_correct_answer() {
        let lower = 'a';
        let upper = 'A';
        let bad = '9';

        let r = convert_item_to_priority(lower).unwrap();
        assert_eq!(r, 1);

        let r = convert_item_to_priority(upper).unwrap();
        assert_eq!(r, 27);

        let r = convert_item_to_priority(bad);
        assert!(r.is_err());
    }

    #[test]
    fn rucksack_get_duplicate_item_returns_correctly() {
        let data = vec![
            (
                Rucksack {
                    left: vec!['v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r'],
                    right: vec!['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p'],
                },
                'p',
            ),
            (
                Rucksack {
                    left: vec![
                        'j', 'q', 'H', 'R', 'N', 'q', 'R', 'j', 'q', 'z', 'j', 'G', 'D', 'L', 'G',
                        'L',
                    ],
                    right: vec![
                        'r', 's', 'F', 'M', 'f', 'F', 'Z', 'S', 'r', 'L', 'r', 'F', 'Z', 's', 'S',
                        'L',
                    ],
                },
                'L',
            ),
            (
                Rucksack {
                    left: vec!['P', 'm', 'm', 'd', 'z', 'q', 'P', 'r', 'V'],
                    right: vec!['v', 'P', 'w', 'w', 'T', 'W', 'B', 'w', 'g'],
                },
                'P',
            ),
            (
                Rucksack {
                    left: vec![
                        'w', 'M', 'q', 'v', 'L', 'M', 'Z', 'H', 'h', 'H', 'M', 'v', 'w', 'L', 'H',
                    ],
                    right: vec![
                        'j', 'b', 'v', 'c', 'j', 'n', 'n', 'S', 'B', 'n', 'v', 'T', 'Q', 'F', 'n',
                    ],
                },
                'v',
            ),
            (
                Rucksack {
                    left: vec!['t', 't', 'g', 'J', 't', 'R', 'G', 'J'],
                    right: vec!['Q', 'c', 't', 'T', 'Z', 't', 'Z', 'T'],
                },
                't',
            ),
            (
                Rucksack {
                    left: vec!['C', 'r', 'Z', 's', 'J', 's', 'P', 'P', 'Z', 's', 'G', 'z'],
                    right: vec!['w', 'w', 's', 'L', 'w', 'L', 'm', 'p', 'w', 'M', 'D', 'w'],
                },
                's',
            ),
        ];

        for (ruck, answer) in data {
            assert_eq!(ruck.get_duplicate_items().unwrap(), answer);
        }
    }
}
