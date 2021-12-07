use std::collections::HashMap;

use aoclib::Point;

use super::Line;

pub fn number_of_overlapping_line_points_with_diagonals(lines: Vec<Line>) -> usize {
    let mut all_coords = HashMap::<Point, u32>::new();
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
    use aoclib::Point;

    use crate::day5::Line;

    use super::number_of_overlapping_line_points_with_diagonals;

    #[test]
    fn number_of_overlapping_line_points_works() {
        let test_data = vec![
            Line(Point { x: 0, y: 9 }, Point { x: 5, y: 9 }),
            Line(Point { x: 8, y: 0 }, Point { x: 0, y: 8 }),
            Line(Point { x: 9, y: 4 }, Point { x: 3, y: 4 }),
            Line(Point { x: 2, y: 2 }, Point { x: 2, y: 1 }),
            Line(Point { x: 7, y: 0 }, Point { x: 7, y: 4 }),
            Line(Point { x: 6, y: 4 }, Point { x: 2, y: 0 }),
            Line(Point { x: 0, y: 9 }, Point { x: 2, y: 9 }),
            Line(Point { x: 3, y: 4 }, Point { x: 1, y: 4 }),
            Line(Point { x: 0, y: 0 }, Point { x: 8, y: 8 }),
            Line(Point { x: 5, y: 5 }, Point { x: 8, y: 2 }),
        ];

        let result = number_of_overlapping_line_points_with_diagonals(test_data);

        assert_eq!(result, 12);
    }
}
