use std::{path::PathBuf, str::FromStr};

use aoclib::read_lines;
use fancy_regex::Regex;

pub fn find_nice_strings_new_rules(filename: PathBuf) -> u32 {
    let mut n: u32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let d = SantasListStringNewRules::from_str(&line);
            match d {
                Ok(SantasListStringNewRules::Nice(_)) => n += 1,
                Ok(SantasListStringNewRules::Naughty(_)) | Err(_) => {}
            }
        }
    }
    n
}

#[derive(Debug, PartialEq)]
pub enum SantasListStringNewRules {
    Nice(String),
    Naughty(String),
}
impl FromStr for SantasListStringNewRules {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if contains_repeated_pair_twice(s) && contains_separated_repeated_letter(s) {
            Ok(SantasListStringNewRules::Nice(s.to_string()))
        } else {
            Ok(SantasListStringNewRules::Naughty(s.to_string()))
        }
    }
}

fn contains_repeated_pair_twice(s: &str) -> bool {
    Regex::new(r"^.*(..).*\1.*$").unwrap().is_match(s).unwrap()
}

fn contains_separated_repeated_letter(s: &str) -> bool {
    Regex::new(r"^.*(.).\1.*$").unwrap().is_match(s).unwrap()
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::{contains_repeated_pair_twice, contains_separated_repeated_letter};
    use crate::day5::puzzle2::SantasListStringNewRules;

    #[test]
    fn from_str_returns_nice_for_valid_nice_string() {
        let test_data = vec!["qjhvhtzxzqqjkmpb", "xxyxx", "fdhsasfdhuias", "osoi8ysys"];
        for d in test_data {
            assert_eq!(
                SantasListStringNewRules::from_str(d),
                Ok(SantasListStringNewRules::Nice(d.to_string())),
                "Failed case: {}",
                d
            )
        }
    }

    #[test]
    fn from_str_returns_naughty_for_valid_naughty_string() {
        let test_data = vec![
            (
                "uurcxstgmygtbstg",
                "Has pair appearing twice but no repeat with single letter between",
            ),
            (
                "ieodomkazucvgmuy",
                "Repeating letter with one in between but no pair appearing twice",
            ),
            ("ofewoaaayhw", "Only one pair that isn't overlapped."),
        ];

        for (d, e) in test_data {
            assert_eq!(
                SantasListStringNewRules::from_str(d),
                Ok(SantasListStringNewRules::Naughty(d.to_string())),
                "Failed case: {}",
                e
            )
        }
    }

    #[test]
    fn contains_separated_repeated_letter_works() {
        let nice_words = vec![
            "qjhvhtzxzqqjkmpb",
            "xyxy",
            "xxyxx",
            "aaa",
            "ieodomkazucvgmuy",
        ];
        let naughty_words = vec!["uurcxstgmygtbstg", "xxzi", "dsadsadsadsa"];

        for w in nice_words {
            assert!(
                contains_separated_repeated_letter(w),
                "NICE: {} is broken",
                w
            );
        }

        for w in naughty_words {
            assert!(
                !contains_separated_repeated_letter(w),
                "NAUGHTY: {} is broken",
                w
            );
        }
    }

    #[test]
    fn contains_repeated_pair_twice_works() {
        let nice_words = vec!["qjhvhtzxzqqjkmpb", "xyxy", "xxyxx", "uurcxstgmygtbstg"];
        let naughty_words = vec!["aaa", "ieodomkazucvgmuy", "xxzi", "abcdefghi"];

        for w in nice_words {
            assert!(contains_repeated_pair_twice(w), "NICE: {} is broken", w);
        }

        for w in naughty_words {
            assert!(!contains_repeated_pair_twice(w), "NAUGHTY: {} is broken", w);
        }
    }
}
