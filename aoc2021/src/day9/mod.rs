use std::path::PathBuf;

use aoclib::{read_lines, Point};
use unicode_segmentation::UnicodeSegmentation;

pub mod puzzle1;
pub mod puzzle2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PointHeight {
    pub coordinate: Point,
    pub height: isize,
}

pub fn get_data_from_file(filename: PathBuf) -> Vec<Vec<PointHeight>> {
    let mut result = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for (row, line) in lines.flatten().enumerate() {
            let height_map_row = UnicodeSegmentation::graphemes(line.as_str(), true)
                .collect::<Vec<&str>>()
                .iter()
                .enumerate()
                .map(|(column, &s)| {
                    let height = s.parse::<isize>().expect("Could not parse height");
                    PointHeight {
                        coordinate: Point {
                            x: column as isize,
                            y: row as isize,
                        },
                        height,
                    }
                })
                .collect::<Vec<PointHeight>>();
            result.push(height_map_row);
        }
    }
    result
}
