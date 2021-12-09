use super::PointHeight;

pub fn get_basin_size_product(data: Vec<Vec<PointHeight>>) -> usize {
    let height = data[0].len();
    let width = data.len();

    let mut basins = Vec::<Basin>::new();
    for y in 0..height {
        for x in 0..width {
            if data[x][y].height == 9 {
                // Don't add this to a basin because it's not part of one
                continue;
            }
            if basins.is_empty() {
                // Probably the very first point. Don't need to run a bunch of code...
                // This is clearly the start of the first basin.
                let b = Basin::new(data[x][y]);
                basins.push(b);
                continue;
            }

            let check_up = y != 0;
            let check_down = y < height - 1;
            let check_left = x != 0;
            let check_right = x < width - 1;

            // Get the indexes of any basins this is a member of
            let mut basin_indexes = basins
                .iter()
                .enumerate()
                .filter(|(_, basin)| {
                    let lower_than_up = if check_up {
                        basin.contains(&data[x][y - 1])
                    } else {
                        false
                    };
                    let lower_than_right = if check_right {
                        basin.contains(&data[x + 1][y])
                    } else {
                        false
                    };
                    let lower_than_down = if check_down {
                        basin.contains(&data[x][y + 1])
                    } else {
                        false
                    };
                    let lower_than_left = if check_left {
                        basin.contains(&data[x - 1][y])
                    } else {
                        false
                    };
                    lower_than_up || lower_than_right || lower_than_down || lower_than_left
                })
                .collect::<Vec<_>>()
                .iter()
                .map(|(index, _)| *index)
                .collect::<Vec<_>>();

            // If point belongs to no basins, create a new one and add it
            if basin_indexes.is_empty() {
                let b = Basin::new(data[x][y]);
                basins.push(b);
                continue;
            }

            // If point has only 1 basin, add to it
            if basin_indexes.len() == 1 {
                basins[basin_indexes[0]].add(data[x][y]);
                continue;
            }

            // If point has more than one basin matched, add it to the first and merge the rest into it
            if basin_indexes.len() > 1 {
                basin_indexes.sort_unstable();
                basin_indexes.reverse();
                let mut temp_basins = Vec::new();
                for i in basin_indexes {
                    temp_basins.push(basins.remove(i));
                }
                let mut first_basin = temp_basins.remove(0);
                first_basin.add(data[x][y]);
                for basin in temp_basins.iter_mut() {
                    first_basin.append(basin);
                }
                basins.push(first_basin);
                continue;
            }
        }
    }

    // Calculate the total basin size
    basins.sort();
    basins.reverse();
    basins.iter().take(3).fold(0_usize, |acc, basin| {
        if acc == 0 {
            basin.len()
        } else {
            acc * basin.len()
        }
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Basin {
    data: Vec<PointHeight>,
}
impl Basin {
    pub fn new(pointheight: PointHeight) -> Self {
        Self {
            data: vec![pointheight],
        }
    }
    pub fn contains(&self, pointheight: &PointHeight) -> bool {
        self.data.contains(pointheight)
    }
    pub fn add(&mut self, pointheight: PointHeight) {
        self.data.push(pointheight);
    }
    pub fn append(&mut self, other: &mut Self) {
        self.data.append(&mut other.get_points());
    }
    pub fn get_points(&self) -> Vec<PointHeight> {
        self.data.clone()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
impl Ord for Basin {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.len().cmp(&other.len())
    }
}
impl PartialOrd for Basin {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.len().partial_cmp(&other.len())
    }
}

#[cfg(test)]
mod test {
    use aoclib::Point;
    use unicode_segmentation::UnicodeSegmentation;

    use crate::day9::{puzzle2::get_basin_size_product, PointHeight};

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

        let result = get_basin_size_product(test_data());

        // Assert
        assert_eq!(result, 1134);
    }
}
