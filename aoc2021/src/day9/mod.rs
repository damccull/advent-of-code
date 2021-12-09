use std::path::PathBuf;

use aoclib::{read_lines, Point};
use unicode_segmentation::UnicodeSegmentation;

use crate::data_file;

use self::puzzle1::get_total_risk_level;

mod puzzle1;
mod puzzle2;

pub fn run() {
    let data = get_data_from_file(data_file("day9.txt"));
    println!("The total risk level is: {}", get_total_risk_level(data));
}

#[derive(Clone, Copy, Debug)]
pub struct PointHeight {
    pub coordinate: Point,
    pub height: isize,
}

fn get_data_from_file(filename: PathBuf) -> Vec<Vec<PointHeight>> {
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
