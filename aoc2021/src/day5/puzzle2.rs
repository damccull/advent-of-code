use std::collections::HashMap;

use crate::day5::Coordinate;

use super::Line;

pub fn number_of_overlapping_line_points_with_diagonals(lines: Vec<Line>) -> usize {
    let mut all_coords = HashMap::<Coordinate, u32>::new();
    for line in lines {
        for coord in line.all_points() {
            let entry = all_coords.entry(coord).or_insert(0);
            *entry += 1;
        }
    }
    all_coords.iter().filter(|&(_, &n)| n > 1).count()
}

#[cfg(test)]
mod test {
    use crate::day5::{Coordinate, Line};

    use super::number_of_overlapping_line_points_with_diagonals;

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

        let result = number_of_overlapping_line_points_with_diagonals(test_data);

        assert_eq!(result, 12);
    }
}
