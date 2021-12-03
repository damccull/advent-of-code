use std::{path::PathBuf, str::FromStr};

use unicode_segmentation::UnicodeSegmentation;

use crate::read_lines;

pub fn find_nice_strings_new_rules(filename: PathBuf) -> u32 {
    let mut n: u32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let d = SantasListString::from_str(&line).expect("Could not parse string");
            match d {
                SantasListString::Nice(_) => n += 1,
                SantasListString::Naughty(_) => {}
            }
        }
    }
    n
}

#[derive(Debug, PartialEq)]
pub enum SantasListString {
    Nice(String),
    Naughty(String),
}
impl FromStr for SantasListString {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if contains_separated_repeated_letter(s) && contains_repeated_pair_twice(s) {
            return Ok(SantasListString::Nice(s.to_string()));
        }
        Ok(SantasListString::Naughty(s.to_string()))
    }
}

fn contains_repeated_pair_twice(s: &str) -> bool {
    let strvec = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
    let mut num_matches = 0;
    let mut last_matched_index = 0;
    for (index, &letter) in strvec.iter().enumerate() {
        if index < strvec.len() - 1 && index + 1 > last_matched_index {
            let current_letter = letter;
            let next_letter = strvec[index + 1];
            if let Some(pos_current_letter) =
                strvec.iter().skip(index).position(|&x| x == current_letter)
            {
                if strvec[pos_current_letter + 1].eq(next_letter) {
                    num_matches += 1;
                    last_matched_index = pos_current_letter;
                }
            }
        }
    }
    num_matches >= 2
}
fn contains_separated_repeated_letter(s: &str) -> bool {
    let strvec = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
    strvec.windows(3).fold(false, |acc, c| {
        if !acc {
            let r = c[0].eq(c[2]);
            r
        } else {
            true
        }
    })
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::{contains_repeated_pair_twice, contains_separated_repeated_letter};
    use crate::day5::puzzle2::SantasListString;

    #[test]
    fn from_str_returns_nice_for_valid_nice_string() {
        assert_eq!(
            SantasListString::from_str("qjhvhtzxzqqjkmpb"),
            Ok(SantasListString::Nice("qjhvhtzxzqqjkmpb".to_string())),
            "`qjhvhtzxzqqjkmpb` is broken"
        );
        assert_eq!(
            SantasListString::from_str("xxyxx"),
            Ok(SantasListString::Nice("xxyxx".to_string())),
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
                SantasListString::from_str(d),
                Ok(SantasListString::Naughty(d.to_string())),
                "Failed case: {}",
                e
            )
        }
    }

    #[test]
    fn contains_separated_repeated_letter_works() {
        let nice_words = vec!["qjhvhtzxzqqjkmpb", "xyxy", "xxyxx"];
        let naughty_words = vec!["uurcxstgmygtbstg", "xxzi", "dsadsadsadsa", "aaa"];

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
        let naughty_words = vec!["ieodomkazucvgmuy", "xxzi", "abcdefghi"];

        for w in nice_words {
            assert!(contains_repeated_pair_twice(w), "{} is broken", w);
        }

        for w in naughty_words {
            assert!(!contains_repeated_pair_twice(w), "{} is broken", w);
        }
    }
}
