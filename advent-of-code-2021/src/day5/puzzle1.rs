use std::collections::HashMap;

use crate::day5::Coordinate;

use super::Line;

pub fn number_of_overlapping_line_points(lines: Vec<Line>) -> usize {
    let mut all_coords = HashMap::<Coordinate, u32>::new();
    for line in lines {
        let x0 = line.0 .0;
        let y0 = line.0 .1;
        let x1 = line.1 .0;
        let y1 = line.1 .1;
        if x0 == x1 || y0 == y1 {
            for coord in line.all_points() {
                let entry = all_coords.entry(coord).or_insert(0);
                *entry += 1;
            }
        }
    }
    all_coords.iter().filter(|&(_, &n)| n > 1).count()
}

#[cfg(test)]
mod test {
    use crate::day5::{Coordinate, Line};

    use super::number_of_overlapping_line_points;

    #[test]
    fn number_of_overlapping_line_points_works() {
        let test_data = vec![
            Line(Coordinate(0, 9), Coordinate(5, 9)),
            Line(Coordinate(8, 0), Coordinate(0, 8)),
            Line(Coordinate(9, 4), Coordinate(3, 4)),
            Line(Coordinate(2, 2), Coordinate(2, 1)),
            Line(Coordinate(7, 0), Coordinate(7, 4)),
            Line(Coordinate(6, 4), Coordinate(2, 0)),
            Line(Coordinate(0, 9), Coordinate(2, 9)),
            Line(Coordinate(3, 4), Coordinate(1, 4)),
            Line(Coordinate(0, 0), Coordinate(8, 8)),
            Line(Coordinate(5, 5), Coordinate(8, 2)),
        ];

        let result = number_of_overlapping_line_points(test_data);

        assert_eq!(result, 5);
    }
}
