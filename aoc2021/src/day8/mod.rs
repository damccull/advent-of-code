use std::{collections::HashMap, str::FromStr};

use aoclib::read_lines;
use itertools::Itertools;

use crate::data_file;

#[derive(Debug)]
pub struct Notebook {
    pub notes: Vec<Note>,
}
impl Notebook {
    /// Returns the total number of times the digits 1, 4, 7, or 8 appear in the output of
    /// all of the notes.
    pub fn count_simple_output_digits(&self) -> usize {
        self.notes
            .iter()
            .map(|n| n.count_simple_output_digits())
            .sum()
    }

    /// Returns the total of all the output value totals from each note added together.
    pub fn get_output_value_total(&self) -> i32 {
        self.notes.iter().map(|n| n.get_output_value_total()).sum()
    }
}
impl TryFrom<Vec<String>> for Notebook {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let notes = value
            .into_iter()
            .map(|e| e.parse())
            .collect::<Result<Vec<Note>, _>>()?;
        Ok(Self { notes })
    }
}

#[derive(Debug)]
pub struct Note {
    pub signal_patterns: Vec<String>,
    pub output_values: Vec<String>,
    pub digit_pattern_map: HashMap<String, i32>,
}

impl Note {
    /// Returns the number of times the digits 1, 4, 7, or 8 appear in the output.
    pub fn count_simple_output_digits(&self) -> usize {
        self.output_values
            .iter()
            .filter(|d| is_simple_digit(d))
            .count()
    }

    /// Returns the total of the four output values added together.
    pub fn get_output_value_total(&self) -> i32 {
        let x = self
            .output_values
            .iter()
            .map(|v| self.digit_pattern_map[v])
            .collect::<Vec<_>>();

        let mut total = 0;
        for (i, n) in x.iter().enumerate() {
            let multiplier = 10_i32.pow((x.len() - i - 1) as u32);
            total += n * multiplier;
        }

        total
    }

    fn get_digit_map(signals: &[String]) -> Result<HashMap<String, i32>, anyhow::Error> {
        // Get the four easy digits
        let one = signals
            .iter()
            .find(|f| f.len() == 2)
            .ok_or_else(|| anyhow::anyhow!("Unable to locate digit `1`"))?;

        let four = signals
            .iter()
            .find(|f| f.len() == 4)
            .ok_or_else(|| anyhow::anyhow!("Unable to locate digit `4`"))?;

        let seven = signals
            .iter()
            .find(|f| f.len() == 3)
            .ok_or_else(|| anyhow::anyhow!("Unable to locate digit `7`"))?;

        let eight = signals
            .iter()
            .find(|f| f.len() == 7)
            .ok_or_else(|| anyhow::anyhow!("Unable to locate digit `8`"))?;

        // Divide the remaining digits in half by their number of signals
        let mut two_three_five = signals.iter().filter(|f| f.len() == 5).collect::<Vec<_>>();
        let mut six_nine_zero = signals.iter().filter(|f| f.len() == 6).collect::<Vec<_>>();

        // Compare digits from the six_nine_zero group to `one` to identify `six`
        let six_position = six_nine_zero
            .iter()
            .position(|f| {
                let onechars = one.chars().collect::<Vec<_>>();
                !(f.contains(onechars[0]) && f.contains(onechars[1]))
            })
            .ok_or_else(|| anyhow::anyhow!("Unable to locate digit `6`"))?;

        // Take `six` from the vec
        let six = six_nine_zero.remove(six_position);

        // Compare digits from the six_nine_zero group to `four` to identify `nine`
        let nine_position = six_nine_zero
            .iter()
            .position(|f| {
                let fourchars = four.chars().collect::<Vec<_>>();
                f.contains(fourchars[0])
                    && f.contains(fourchars[1])
                    && f.contains(fourchars[2])
                    && f.contains(fourchars[3])
            })
            .ok_or_else(|| anyhow::anyhow!("Unable to locate digit `9`"))?;

        // Take `nine` from the vec
        let nine = six_nine_zero.remove(nine_position);

        // Last one should be `zero` digit at position 0;
        let zero = six_nine_zero.remove(0);

        // Compare digits from the two_three_five group to `one` to identify `three`
        let three_position = two_three_five
            .iter()
            .position(|f| {
                let onechars = one.chars().collect::<Vec<_>>();
                f.contains(onechars[0]) && f.contains(onechars[1])
            })
            .ok_or_else(|| anyhow::anyhow!("Unable to locate digit `3`"))?;

        // Take `three` from the vec
        let three = two_three_five.remove(three_position);

        // Compare digits from the two_three_five group to `six` to identify `five`
        let five_position = two_three_five
            .iter()
            .position(|f| {
                let fivechars = f.chars().collect::<Vec<_>>();
                six.contains(fivechars[0])
                    && six.contains(fivechars[1])
                    && six.contains(fivechars[2])
                    && six.contains(fivechars[3])
                    && six.contains(fivechars[4])
            })
            .ok_or_else(|| anyhow::anyhow!("Unable to locate digit `5`"))?;

        // Take `three` from the vec
        let five = two_three_five.remove(five_position);

        // Last one should be `two` digit at position 0;
        let two = two_three_five.remove(0);

        let mut digit_map = HashMap::new();

        digit_map.insert(one.clone(), 1);
        digit_map.insert(two.clone(), 2);
        digit_map.insert(three.clone(), 3);
        digit_map.insert(four.clone(), 4);
        digit_map.insert(five.clone(), 5);
        digit_map.insert(six.clone(), 6);
        digit_map.insert(seven.clone(), 7);
        digit_map.insert(eight.clone(), 8);
        digit_map.insert(nine.clone(), 9);
        digit_map.insert(zero.clone(), 0);

        Ok(digit_map)
    }
}

impl FromStr for Note {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains('|') {
            return Err(anyhow::anyhow!("Bad input; could not parse note string"));
        }
        let parts = s.split('|').collect::<Vec<&str>>();

        let signal_patterns = parts[0]
            .trim()
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if signal_patterns.len() != 10 {
            return Err(anyhow::anyhow!(
                "Bad input; incorrect number of signal patterns"
            ));
        }

        let output_values = parts[1]
            .trim()
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if output_values.len() != 4 {
            return Err(anyhow::anyhow!(
                "Bad input; incorrect number of output values"
            ));
        }

        let digit_pattern_map = Note::get_digit_map(&signal_patterns)?;

        Ok(Self {
            signal_patterns,
            output_values,
            digit_pattern_map,
        })
    }
}

pub fn get_data(filename: &str) -> Vec<String> {
    let mut x = Vec::<String>::new();
    if let Ok(lines) = read_lines(data_file(filename)) {
        x.append(&mut lines.flatten().collect::<Vec<String>>());
    }
    x
}

pub fn is_simple_digit(d: &str) -> bool {
    d.len() == 7 // Seven signal lines is digit 8
    || d.len() == 3 // Three signal lines is digit 7
    || d.len() == 4 // Four signal lines is digit 4
    || d.len() == 2 // Two signal lines is digit 1
}

#[cfg(test)]
mod tests {
    use crate::day8::get_data;

    use super::Notebook;

    #[test]
    fn parse_notebook_returns_notebook_with_valid_input() {
        // Arrange
        let data = get_data("day8-test.txt");

        // Act
        let result = Notebook::try_from(data);

        // Assert
        assert!(result.is_ok(), "Problem caused by: {:?}", result);
    }

    #[test]
    fn parse_notebook_returns_err_with_invalid_input() {
        // Arrange
        let tests = vec![
            ("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | cefdb cefbgd gcbe".to_string(),"One output digit missing"),
            ("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc fdcagb".to_string(),"One extra output digit"),
            ("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb | cg cg fdcagb cbg".to_string(),"One input signal missing"),
            ("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega fcbdga | efabcd cedba gadfec cb".to_string(),"One extra input signal"),
            ("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga".to_string(),"No pipe or output digits"),
            ("| gebdcfa ecba ca fadegcb".to_string(),"No input digits"),
            ("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |".to_string(),"No output digits")
        ];

        // Act
        for (test, reason) in tests {
            let data = vec![test];
            let result = Notebook::try_from(data);

            // Assert
            assert!(result.is_err(), "Failed test: {}", reason);
        }
    }

    #[test]
    fn note_count_simple_digits_returns_correct_value() {
        // Arrange
        const CORRECT_COUNT: usize = 26;

        let notebook = Notebook::try_from(get_data("day8-test.txt")).unwrap();

        // Act
        let result = notebook.count_simple_output_digits();

        // Assert
        assert_eq!(CORRECT_COUNT, result);
    }
}
