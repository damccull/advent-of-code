use std::{collections::HashMap, path::PathBuf, str::FromStr};

use unicode_segmentation::UnicodeSegmentation;

use crate::read_lines;

pub fn find_nice_strings_new_rules(filename: PathBuf) -> u32 {
    let mut n: u32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let d = SantasListStringNewRules::from_str(&line).expect("Could not parse string");
            match d {
                SantasListStringNewRules::Nice(x) => {dbg!(x);n += 1},
                SantasListStringNewRules::Naughty(_) => {}
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
        if contains_separated_repeated_letter(s) && contains_repeated_pair_twice(s) {
            return Ok(SantasListStringNewRules::Nice(s.to_string()));
        }
        Ok(SantasListStringNewRules::Naughty(s.to_string()))
    }
}

fn contains_repeated_pair_twice(s: &str) -> bool {
    let strvec = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();

    let pairs: Vec<&[&str]> = strvec.windows(2).collect();
    let mut counters = HashMap::new();
    for (i, x) in pairs.iter().enumerate() {
        if x[0] == x[1] && i < pairs.len() - 1 && x[1] == pairs[i + 1][0] {
            continue;
        }
        let c = counters.entry(x).or_insert(0);
        *c += 1;
    }
    let mut fuse = false;
    for (_key, val) in counters.iter() {
        if val >= &2 {
            fuse = true;
        }
    }
    fuse
}

fn contains_separated_repeated_letter(s: &str) -> bool {
    let strvec = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
    strvec
        .windows(3)
        .fold(false, |acc, c| if !acc { c[0].eq(c[2]) } else { true })
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::{contains_repeated_pair_twice, contains_separated_repeated_letter};
    use crate::day5::puzzle2::SantasListStringNewRules;

    #[test]
    fn from_str_returns_nice_for_valid_nice_string() {
        assert_eq!(
            SantasListStringNewRules::from_str("qjhvhtzxzqqjkmpb"),
            Ok(SantasListStringNewRules::Nice(
                "qjhvhtzxzqqjkmpb".to_string()
            )),
            "`qjhvhtzxzqqjkmpb` is broken"
        );
        assert_eq!(
            SantasListStringNewRules::from_str("xxyxx"),
            Ok(SantasListStringNewRules::Nice("xxyxx".to_string())),
            "`xxyxx` is broken"
        );
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
        let nice_words = vec!["qjhvhtzxzqqjkmpb", "xyxy", "xxyxx", "aaa"];
        let naughty_words = vec!["uurcxstgmygtbstg", "xxzi", "dsadsadsadsa"];

        for w in nice_words {
            assert!(contains_separated_repeated_letter(w), "{} is broken", w);
        }

        for w in naughty_words {
            assert!(!contains_separated_repeated_letter(w), "{} is broken", w);
        }
    }

    #[test]
    fn contains_repeated_pair_twice_works() {
        let nice_words = vec!["qjhvhtzxzqqjkmpb", "xyxy", "xxyxx"];
        let naughty_words = vec!["aaa", "ieodomkazucvgmuy", "xxzi", "abcdefghi"];

        for w in nice_words {
            assert!(contains_repeated_pair_twice(w), "NICE: {} is broken", w);
        }

        for w in naughty_words {
            assert!(!contains_repeated_pair_twice(w), "NAUGHTY: {} is broken", w);
        }
    }
}
