use aoc2021::{
    data_file,
    day5::{
        load_coordinates, puzzle1::number_of_overlapping_line_points,
        puzzle2::number_of_overlapping_line_points_with_diagonals,
    },
};

fn main() {
    if let Ok(lines) = load_coordinates(data_file("day5.txt")) {
        let overlaps = number_of_overlapping_line_points(lines.clone());
        println!("D5P1: Number of overlapping points is: {}", overlaps);

        let overlaps = number_of_overlapping_line_points_with_diagonals(lines);
        println!(
            "D5P2: Number of overlapping points when considering diagonals is: {}",
            overlaps
        );
    } else {
        panic!("Unable to load coordinates from file.");
    }
}
