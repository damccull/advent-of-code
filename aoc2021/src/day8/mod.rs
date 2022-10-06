use std::str::FromStr;

use aoclib::read_lines;

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
    pub signal_patterns: Vec<Vec<WireSignal>>,
    pub output_values: Vec<Vec<WireSignal>>,
}

impl Note {
    /// Returns the number of times the digits 1, 4, 7, or 8 appear in the output.
    pub fn count_simple_output_digits(&self) -> usize {
        self.output_values
            .iter()
            .filter(|d| is_simple_digit(d))
            .count()
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

        // Break each pattern into characters and map those to WireSignal variants
        // Return a Vec<Vec<WireSignal>>
        let signal_patterns = signal_patterns
            .iter()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_string().parse::<WireSignal>())
                    .collect::<Result<Vec<WireSignal>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

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

        let output_values = output_values
            .iter()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_string().parse::<WireSignal>())
                    .collect::<Result<Vec<WireSignal>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if output_values.len() != 4 {
            return Err(anyhow::anyhow!(
                "Bad input; incorrect number of output values"
            ));
        }

        Ok(Self {
            signal_patterns,
            output_values,
        })
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum WireSignal {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    #[default]
    Off,
}
impl FromStr for WireSignal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s {
            "A" | "a" => Self::A,
            "B" | "b" => Self::B,
            "C" | "c" => Self::C,
            "D" | "d" => Self::D,
            "E" | "e" => Self::E,
            "F" | "f" => Self::F,
            "G" | "g" => Self::G,
            _ => anyhow::bail!("Not a valid wire signal"),
        };
        Ok(r)
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct WireSignalMap {
    top: WireSignal,
    top_left: WireSignal,
    top_right: WireSignal,
    center: WireSignal,
    bottom_left: WireSignal,
    bottom_right: WireSignal,
    bottom: WireSignal,
}
impl TryFrom<Vec<Vec<WireSignal>>> for WireSignalMap {
    type Error = anyhow::Error;

    fn try_from(value: Vec<Vec<WireSignal>>) -> Result<Self, Self::Error> {
        let mut signal_map = Self::default();
        // Get the 1 digit
        let one_digit = value
            .iter()
            .find(|f| f.len() == 2)
            .ok_or_else(|| anyhow::anyhow!("No `1` digit"))?;

        // Get the 7 digit
        let seven_digit = value
            .iter()
            .find(|f| f.len() == 3)
            .ok_or_else(|| anyhow::anyhow!("No `7` digit"))?;

        // Map the top segment
        signal_map.top = *seven_digit
            .iter()
            .find(|s| one_digit.contains(s))
            .ok_or_else(|| anyhow::anyhow!("Unable to find top segment"))?;

        // To find 5, we need to get the two segments of 4 that are not also in 1
        // Get the 4 digit
        let four_digit = value
            .iter()
            .find(|f| f.len() == 4)
            .ok_or_else(|| anyhow::anyhow!("No `4` digit"))?;

        // Get the two segments that are not in 1
        let four_left_segments = four_digit
            .iter()
            .filter(|f| !one_digit.contains(f))
            .collect::<Vec<_>>();

        // Get all digits that have 5 segments
        let five_digit = value
            .iter()
            .filter(|f| f.len() == 5)
            .find(|f| f.contains(four_left_segments[0]) && f.contains(four_left_segments[1]))
            .ok_or_else(|| anyhow::anyhow!("Unable to find `5` digit"))?;

        // Map the top right segment
        signal_map.top_right = *one_digit
            .iter()
            .find(|f| !five_digit.contains(f))
            .ok_or_else(|| anyhow::anyhow!("Unable to find top right segment"))?;
        // Map the bottom right segment
        signal_map.bottom_right = *one_digit
            .iter()
            .find(|f| **f != signal_map.top_right)
            .ok_or_else(|| anyhow::anyhow!("Unable to find bottom right segment"))?;

        todo!()
    }
}

pub fn get_data(filename: &str) -> Vec<String> {
    let mut x = Vec::<String>::new();
    if let Ok(lines) = read_lines(data_file(filename)) {
        x.append(&mut lines.flatten().collect::<Vec<String>>());
    }
    x
}

pub fn is_simple_digit(d: &Vec<WireSignal>) -> bool {
    d.len() == 7 // Seven signal lines is digit 8
    || d.len() == 3 // Three signal lines is digit 7
    || d.len() == 4 // Four signal lines is digit 4
    || d.len() == 2 // Two signal lines is digit 1
}

// #[allow(non_snake_case)]
// pub mod WireSignalFlags {
//     pub const A: u8 = 0x01;
//     pub const B: u8 = 0x02;
//     pub const C: u8 = 0x04;
//     pub const D: u8 = 0x08;
//     pub const E: u8 = 0x10;
//     pub const F: u8 = 0x20;
//     pub const G: u8 = 0x40;
// }

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
