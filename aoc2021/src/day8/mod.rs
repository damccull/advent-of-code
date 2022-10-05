use std::str::FromStr;

use aoclib::read_lines;

use crate::data_file;

#[derive(Debug)]
pub struct Notebook {
    pub notes: Vec<Note>,
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

        Ok(Self {
            signal_patterns,
            output_values,
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
}
