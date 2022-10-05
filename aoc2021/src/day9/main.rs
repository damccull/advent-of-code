use aoc2021::{
    data_file,
    day9::{get_data_from_file, puzzle1::get_total_risk_level, puzzle2::get_basin_size_product},
};

fn main() {
    let data = get_data_from_file(data_file("day9.txt"));
    println!(
        "The total risk level is: {}",
        get_total_risk_level(data.clone())
    );
    println!(
        "The product of basin sizes is: {}",
        get_basin_size_product(data)
    );
}
