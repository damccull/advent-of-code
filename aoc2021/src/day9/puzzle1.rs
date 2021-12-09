use super::PointHeight;

pub fn get_total_risk_level(data: Vec<Vec<PointHeight>>) -> isize {
    let height = data[0].len();
    let width = data.len();

    let mut low_points_tracker = Vec::<(PointHeight, bool)>::new();

    for y in 0..height {
        for x in 0..width {
            let check_up = y != 0;
            let check_down = y < height - 1;
            let check_left = x != 0;
            let check_right = x < width - 1;

            let lower_than_up = if check_up {
                data[x][y].height < data[x][y - 1].height
            } else {
                true
            };
            let lower_than_right = if check_right {
                data[x][y].height < data[x + 1][y].height
            } else {
                true
            };
            let lower_than_down = if check_down {
                data[x][y].height < data[x][y + 1].height
            } else {
                true
            };
            let lower_than_left = if check_left {
                data[x][y].height < data[x - 1][y].height
            } else {
                true
            };

            low_points_tracker.push((
                data[x][y],
                lower_than_up && lower_than_right && lower_than_down && lower_than_left,
            ))
        }
    }

    low_points_tracker
        .into_iter()
        .filter(|(_, is_lowpoint)| *is_lowpoint)
        .collect::<Vec<(PointHeight, bool)>>()
        .iter()
        .map(|(point, _)| *point)
        .collect::<Vec<PointHeight>>()
        .iter()
        .fold(0_isize, |acc, pointheight| acc + pointheight.height + 1)
}

#[cfg(test)]
mod test {
    use aoclib::Point;
    use unicode_segmentation::UnicodeSegmentation;

    use crate::day9::{puzzle1::get_total_risk_level, PointHeight};

    #[test]
    fn get_total_risk_level_works() {
        // Set up test
        let test_data_string = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];
        let test_data = move || {
            let mut result = Vec::new();
            let lines = test_data_string;
            for (row, &line) in lines.iter().enumerate() {
                let height_map_row = UnicodeSegmentation::graphemes(line, true)
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
            result
        };

        // Act

        let result = get_total_risk_level(test_data());

        // Assert
        assert_eq!(result, 15);
    }
}
