use std::{path::PathBuf, str::FromStr};

use unicode_segmentation::UnicodeSegmentation;

use crate::read_lines;

pub fn find_nice_strings(filename: PathBuf) -> u32 {
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
        if SantasListString::contains_blacklisted_sequences(s) {
            return Ok(SantasListString::Naughty(s.to_string()));
        } else if SantasListString::contains_a_doubled_letter(s)
            && SantasListString::contains_three_vowels(s)
        {
            return Ok(SantasListString::Nice(s.to_string()));
        }
        Ok(SantasListString::Naughty(s.to_string()))
    }
}
impl SantasListString {
    fn contains_blacklisted_sequences(s: &str) -> bool {
        s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
    }
    fn contains_a_doubled_letter(s: &str) -> bool {
        let strvec = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
        strvec
            .windows(2)
            .fold(false, |acc, c| if !acc { c[0].eq(c[1]) } else { true })
    }
    fn contains_three_vowels(s: &str) -> bool {
        let vowels = ["a", "e", "i", "o", "u"];
        let strvec = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();

        let num_vowels = strvec.iter().fold(
            0_i32,
            |acc, c| if vowels.contains(c) { acc + 1 } else { acc },
        );
        num_vowels >= 3
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::day5::puzzle1::SantasListString;

    #[test]
    fn from_str_returns_nice_for_valid_nice_string() {
        assert_eq!(
            SantasListString::from_str("ugknbfddgicrmopn"),
            Ok(SantasListString::Nice("ugknbfddgicrmopn".to_string()))
        );
        assert_eq!(
            SantasListString::from_str("aaa"),
            Ok(SantasListString::Nice("aaa".to_string()))
        );
    }

    #[test]
    fn from_str_returns_naughty_for_valid_naughty_string() {
        let test_data = vec![
            ("jchzalrnumimnmhp", "No double letter"),
            ("haegwjzuvuyypxyu", "Contains xy"),
            ("dvszwmarrgswjxmb", "Contains only one vowel"),
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
}
