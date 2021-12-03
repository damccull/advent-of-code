use std::path::PathBuf;

use unicode_segmentation::UnicodeSegmentation;

use crate::{data_file, day3::puzzle1::get_diagnostics, read_lines};

pub mod puzzle1;
pub mod puzzle2;

pub fn run() {
    println!(
        "D3P1: The overall diagnostic code is {}.",
        get_diagnostics(data_file("day3.txt"))
    )
}

pub struct DiagnosticReport {
    binary: Vec<Vec<bool>>,
}
impl DiagnosticReport {
    pub fn get_gamma_rate_binary(&self) -> Vec<bool> {
        // This method assumes all the codes are the same length
        let mut result = Vec::new();
        for i in 0..self.binary[0].len() {
            let ones_count = self
                .binary
                .iter()
                .fold(0, |ones, code| if code[i] { ones + 1 } else { ones });
            if ones_count > self.binary.len() / 2 {
                result.push(true);
            } else {
                result.push(false);
            }
        }
        result
    }
    pub fn get_epsilon_rate_binary(&self) -> Vec<bool> {
        // This method assumes all the codes are the same length
        let mut result = Vec::new();
        for i in 0..self.binary[0].len() {
            let ones_count = self
                .binary
                .iter()
                .fold(0, |ones, code| if code[i] { ones + 1 } else { ones });
            if ones_count > self.binary.len() / 2 {
                result.push(false);
            } else {
                result.push(true);
            }
        }
        result
    }

    pub fn get_gamma_rate_decimal(&self) -> u32 {
        self.get_gamma_rate_binary()
            .iter()
            .fold(0, |result, &bit| (result << 1) ^ bit as u32)
    }
    pub fn get_epsilon_rate_decimal(&self) -> u32 {
        self.get_epsilon_rate_binary()
            .iter()
            .fold(0, |result, &bit| (result << 1) ^ bit as u32)
    }
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
}
