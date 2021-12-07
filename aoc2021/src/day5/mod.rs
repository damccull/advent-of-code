use std::path::PathBuf;

use aoclib::{read_lines, Point};

use crate::{data_file, day5::puzzle2::number_of_overlapping_line_points_with_diagonals};

use self::puzzle1::number_of_overlapping_line_points;

pub mod puzzle1;
pub mod puzzle2;

pub fn run() {
    if let Ok(lines) = load_coordinates(data_file("day5.txt")) {
        let overlaps = number_of_overlapping_line_points(lines.clone());
        println!(
            "D5P1: Number of overlapping points is: {}",
            overlaps.to_string()
        );

        let overlaps = number_of_overlapping_line_points_with_diagonals(lines);
        println!(
            "D5P2: Number of overlapping points when considering diagonals is: {}",
            overlaps.to_string()
        );
    } else {
        panic!("Unable to load coordinates from file.");
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Line(Point, Point);
impl Line {
    /// Implements Bresenham's algorithm
    pub fn all_points(&self) -> Vec<Point> {
        let x0 = self.0.x;
        let y0 = self.0.y;
        let x1 = self.1.x;
        let y1 = self.1.y;

        if (y1 - y0).abs() < (x1 - x0).abs() {
            if x0 > x1 {
                Line::plot_line_low(x1, y1, x0, y0)
            } else {
                Line::plot_line_low(x0, y0, x1, y1)
            }
        } else if y0 > y1 {
            Line::plot_line_high(x1, y1, x0, y0)
        } else {
            Line::plot_line_high(x0, y0, x1, y1)
        }
    }

    fn plot_line_low(x0: isize, y0: isize, x1: isize, y1: isize) -> Vec<Point> {
        let mut result = Vec::<Point>::new();
        let dx = x1 - x0;
        let mut dy = y1 - y0;
        let mut yi = 1;
        if dy < 0 {
            yi = -1;
            dy = -dy;
        }
        let mut d = (2 * dy) - dx;
        let mut y = y0;

        for x in x0..=x1 {
            result.push(Point { x, y });
            if d > 0 {
                y += yi;
                d += 2 * (dy - dx);
            } else {
                d += 2 * dy;
            }
        }

        result
    }
    fn plot_line_high(x0: isize, y0: isize, x1: isize, y1: isize) -> Vec<Point> {
        let mut result = Vec::<Point>::new();
        let mut dx = x1 - x0;
        let dy = y1 - y0;
        let mut xi = 1;

        if dx < 0 {
            xi = -1;
            dx = -dx;
        }

        let mut d = (2 * dx) - dy;
        let mut x = x0;

        for y in y0..=y1 {
            result.push(Point { x, y });
            if d > 0 {
                x += xi;
                d += 2 * (dx - dy)
            } else {
                d += 2 * dx;
            }
        }

        result
    }
}

pub fn load_coordinates(filename: PathBuf) -> Result<Vec<Line>, String> {
    let lines = match read_lines(filename) {
        Ok(x) => x.flatten().collect::<Vec<String>>(),
        Err(e) => return Err(format!("Error reading lines from file: {}", e)),
    };
    parse_lines(lines)
}

fn parse_lines(lines: Vec<String>) -> Result<Vec<Line>, String> {
    let mut result = Vec::<Line>::new();

    for line in lines {
        let coord_strings = line.split("->").collect::<Vec<_>>();
        let c1 = match coord_strings[0].trim().parse::<Point>() {
            Ok(c) => c,
            Err(e) => return Err(format!("Error parsing Coordinate: {}", e)),
        };
        let c2 = match coord_strings[1].trim().parse::<Point>() {
            Ok(c) => c,
            Err(e) => return Err(format!("Error parsing Coordinate: {}", e)),
        };
        result.push(Line(c1, c2));
    }
    Ok(result)
}

#[cfg(test)]
mod test {

    use aoclib::Point;

    use super::{parse_lines, Line};

    #[test]
    fn parse_lines_works() {
        let test_data = vec![(
            vec!["1,2 -> 2,2".to_string()],
            vec![Line(Point { x: 1, y: 2 }, Point { x: 2, y: 2 })],
        )];

        for (s, c) in test_data {
            assert_eq!(parse_lines(s), Ok(c))
        }
    }

    #[test]
    fn all_points_works() {
        let test_data = vec![
            (
                Line(Point { x: 1, y: 2 }, Point { x: 3, y: 2 }),
                vec![
                    Point { x: 1, y: 2 },
                    Point { x: 2, y: 2 },
                    Point { x: 3, y: 2 },
                ],
            ),
            (
                Line(Point { x: 1, y: 2 }, Point { x: 1, y: 5 }),
                vec![
                    Point { x: 1, y: 2 },
                    Point { x: 1, y: 3 },
                    Point { x: 1, y: 4 },
                    Point { x: 1, y: 5 },
                ],
            ),
            (
                Line(Point { x: 1, y: 1 }, Point { x: 4, y: 4 }),
                vec![
                    Point { x: 1, y: 1 },
                    Point { x: 2, y: 2 },
                    Point { x: 3, y: 3 },
                    Point { x: 4, y: 4 },
                ],
            ),
            (
                Line(Point { x: 1, y: 1 }, Point { x: 4, y: 2 }),
                vec![
                    Point { x: 1, y: 1 },
                    Point { x: 2, y: 1 },
                    Point { x: 3, y: 2 },
                    Point { x: 4, y: 2 },
                ],
            ),
            (
                Line(Point { x: 1, y: 1 }, Point { x: 3, y: 5 }),
                vec![
                    Point { x: 1, y: 1 },
                    Point { x: 1, y: 2 },
                    Point { x: 2, y: 3 },
                    Point { x: 2, y: 4 },
                    Point { x: 3, y: 5 },
                ],
            ),
        ];

        for (line, result) in test_data {
            assert_eq!(line.all_points(), result, "Line failed: {:?}", line)
        }
    }
}
