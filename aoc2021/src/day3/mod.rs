use std::path::PathBuf;

use aoclib::read_lines;
use unicode_segmentation::UnicodeSegmentation;

pub mod puzzle1;
pub mod puzzle2;

pub struct DiagnosticReport {
    binary: Vec<Vec<bool>>,
}
impl DiagnosticReport {
    pub fn get_gamma_rate_binary(&self) -> Vec<bool> {
        // This method assumes all the codes are the same length
        let mut result = Vec::new();
        for i in 0..self.binary[0].len() {
            let bit = match DiagnosticReport::most_common_bit(self.binary.clone(), i) {
                CommonBit::One => true,
                CommonBit::Zero => false,
                CommonBit::Equal => true,
            };
            result.push(bit);
        }
        result
    }
    pub fn get_epsilon_rate_binary(&self) -> Vec<bool> {
        // This method assumes all the codes are the same length
        let mut result = Vec::new();
        for i in 0..self.binary[0].len() {
            let bit = match DiagnosticReport::most_common_bit(self.binary.clone(), i) {
                CommonBit::One => true,
                CommonBit::Zero => false,
                CommonBit::Equal => true,
            };
            result.push(!bit);
        }
        result
    }

    pub fn get_oxygen_generator_rating_binary(&self) -> Vec<bool> {
        // This method assumes all the codes are the same length
        let mut working_set = self.binary.clone();
        for i in 0..self.binary[0].len() {
            let common_bit = match DiagnosticReport::most_common_bit(working_set.clone(), i) {
                CommonBit::One => true,
                CommonBit::Zero => false,
                CommonBit::Equal => true,
            };
            working_set = working_set
                .into_iter()
                .filter(|x| x[i] == common_bit)
                .collect();
            if working_set.len() == 1 {
                break;
            }
        }
        working_set[0].clone()
    }
    pub fn get_co2_scrubber_rating_binary(&self) -> Vec<bool> {
        // This method assumes all the codes are the same length
        let mut working_set = self.binary.clone();
        for i in 0..self.binary[0].len() {
            let common_bit = match DiagnosticReport::least_common_bit(working_set.clone(), i) {
                CommonBit::One => true,
                CommonBit::Zero => false,
                CommonBit::Equal => false,
            };
            working_set = working_set
                .into_iter()
                .filter(|x| x[i] == common_bit)
                .collect();
            if working_set.len() == 1 {
                break;
            }
        }
        working_set[0].clone()
    }

    pub fn get_gamma_rate_decimal(&self) -> u32 {
        DiagnosticReport::binary_to_decimal(self.get_gamma_rate_binary())
    }
    pub fn get_epsilon_rate_decimal(&self) -> u32 {
        DiagnosticReport::binary_to_decimal(self.get_epsilon_rate_binary())
    }

    pub fn get_oxygen_generator_rating_decimal(&self) -> u32 {
        DiagnosticReport::binary_to_decimal(self.get_oxygen_generator_rating_binary())
    }
    pub fn get_co2_scrubber_rating_decimal(&self) -> u32 {
        DiagnosticReport::binary_to_decimal(self.get_co2_scrubber_rating_binary())
    }

    pub fn binary_to_decimal(binary: Vec<bool>) -> u32 {
        binary
            .iter()
            .fold(0, |result, &bit| (result << 1) ^ bit as u32)
    }
    pub fn most_common_bit(set: Vec<Vec<bool>>, bit_position: usize) -> CommonBit {
        let ones_count = set.iter().fold(
            0,
            |ones, code| if code[bit_position] { ones + 1 } else { ones },
        );
        let len = (set.len() + 1) / 2;
        match ones_count.cmp(&len) {
            std::cmp::Ordering::Less => CommonBit::Zero,
            std::cmp::Ordering::Equal => CommonBit::Equal,
            std::cmp::Ordering::Greater => CommonBit::One,
        }
    }
    pub fn least_common_bit(set: Vec<Vec<bool>>, bit_position: usize) -> CommonBit {
        let ones_count = set.iter().fold(
            0,
            |ones, code| if code[bit_position] { ones + 1 } else { ones },
        );
        let len = (set.len() + 1) / 2;
        match ones_count.cmp(&len) {
            std::cmp::Ordering::Less => CommonBit::One,
            std::cmp::Ordering::Equal => CommonBit::Equal,
            std::cmp::Ordering::Greater => CommonBit::Zero,
        }
    }
}

pub enum CommonBit {
    One,
    Zero,
    Equal,
}

pub fn get_diagnostic_report(filename: PathBuf) -> DiagnosticReport {
    let mut result = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            result.push(parse_diagnostic_binary(line));
        }
    }
    DiagnosticReport { binary: result }
}

fn parse_diagnostic_binary(binary_string: String) -> Vec<bool> {
    UnicodeSegmentation::graphemes(binary_string.as_str(), true)
        .map(|c| c.eq("1"))
        .collect::<Vec<bool>>()
}

#[cfg(test)]
mod test {
    use crate::day3::{parse_diagnostic_binary, DiagnosticReport};

    #[test]
    fn get_diagnostic_report_returns_correct_struct() {
        let data = vec![
            ("00100".to_string(), vec![false, false, true, false, false]),
            ("11110".to_string(), vec![true, true, true, true, false]),
        ];

        for (d, r) in data {
            assert_eq!(parse_diagnostic_binary(d.clone()), r, "Broken case: {}", d);
        }
    }

    #[test]
    fn get_gamma_rate_binary_works() {
        let test_data = DiagnosticReport {
            binary: vec![
                vec![false, false, true, false, false],
                vec![true, true, true, true, false],
                vec![true, false, true, true, false],
                vec![true, false, true, true, true],
                vec![true, false, true, false, true],
                vec![false, true, true, true, true],
                vec![false, false, true, true, true],
                vec![true, true, true, false, false],
                vec![true, false, false, false, false],
                vec![true, true, false, false, true],
                vec![false, false, false, true, false],
                vec![false, true, false, true, false],
            ],
        };
        let result = vec![true, false, true, true, false];
        assert_eq!(test_data.get_gamma_rate_binary(), result);
    }

    #[test]
    fn get_epsilon_rate_binary_works() {
        let test_data = DiagnosticReport {
            binary: vec![
                vec![false, false, true, false, false],
                vec![true, true, true, true, false],
                vec![true, false, true, true, false],
                vec![true, false, true, true, true],
                vec![true, false, true, false, true],
                vec![false, true, true, true, true],
                vec![false, false, true, true, true],
                vec![true, true, true, false, false],
                vec![true, false, false, false, false],
                vec![true, true, false, false, true],
                vec![false, false, false, true, false],
                vec![false, true, false, true, false],
            ],
        };
        let result = vec![false, true, false, false, true];
        assert_eq!(test_data.get_epsilon_rate_binary(), result);
    }

    #[test]
    fn get_gamma_rate_decimal_returns_correct_number() {
        let test_data = DiagnosticReport {
            binary: vec![
                vec![false, false, true, false, false],
                vec![true, true, true, true, false],
                vec![true, false, true, true, false],
                vec![true, false, true, true, true],
                vec![true, false, true, false, true],
                vec![false, true, true, true, true],
                vec![false, false, true, true, true],
                vec![true, true, true, false, false],
                vec![true, false, false, false, false],
                vec![true, true, false, false, true],
                vec![false, false, false, true, false],
                vec![false, true, false, true, false],
            ],
        };
        let result = 22_u32;
        assert_eq!(test_data.get_gamma_rate_decimal(), result);
    }

    #[test]
    fn get_epsilon_rate_decimal_returns_correct_number() {
        let test_data = DiagnosticReport {
            binary: vec![
                vec![false, false, true, false, false],
                vec![true, true, true, true, false],
                vec![true, false, true, true, false],
                vec![true, false, true, true, true],
                vec![true, false, true, false, true],
                vec![false, true, true, true, true],
                vec![false, false, true, true, true],
                vec![true, true, true, false, false],
                vec![true, false, false, false, false],
                vec![true, true, false, false, true],
                vec![false, false, false, true, false],
                vec![false, true, false, true, false],
            ],
        };
        let result = 9_u32;
        assert_eq!(test_data.get_epsilon_rate_decimal(), result);
    }

    #[test]
    fn get_oxygen_generator_rating_binary_works() {
        let test_data = DiagnosticReport {
            binary: vec![
                vec![false, false, true, false, false],
                vec![true, true, true, true, false],
                vec![true, false, true, true, false],
                vec![true, false, true, true, true],
                vec![true, false, true, false, true],
                vec![false, true, true, true, true],
                vec![false, false, true, true, true],
                vec![true, true, true, false, false],
                vec![true, false, false, false, false],
                vec![true, true, false, false, true],
                vec![false, false, false, true, false],
                vec![false, true, false, true, false],
            ],
        };
        let result = vec![true, false, true, true, true];
        assert_eq!(test_data.get_oxygen_generator_rating_binary(), result);
    }

    #[test]
    fn get_co2_scrubber_rating_binary_works() {
        let test_data = DiagnosticReport {
            binary: vec![
                vec![false, false, true, false, false],
                vec![true, true, true, true, false],
                vec![true, false, true, true, false],
                vec![true, false, true, true, true],
                vec![true, false, true, false, true],
                vec![false, true, true, true, true],
                vec![false, false, true, true, true],
                vec![true, true, true, false, false],
                vec![true, false, false, false, false],
                vec![true, true, false, false, true],
                vec![false, false, false, true, false],
                vec![false, true, false, true, false],
            ],
        };
        let result = vec![false, true, false, true, false];
        assert_eq!(test_data.get_co2_scrubber_rating_binary(), result);
    }
}
