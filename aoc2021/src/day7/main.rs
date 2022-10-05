use aoc2021::{
    data_file,
    day7::get_data,
    day7::{puzzle1::find_minimum_fuel, puzzle2::find_minimum_fuel_increasing_rate},
};

fn main() {
    let x = find_minimum_fuel(&get_data(data_file("day7.txt")));
    println!(
        "D7P1: The minimum fuel required to align the crabsubs is: {}",
        x
    );

    let x = find_minimum_fuel_increasing_rate(&get_data(data_file("day7.txt")));
    println!(
        "D7P1: The minimum fuel required to align the crabsubs with the correct burn rate is: {}",
        x
    );
}
