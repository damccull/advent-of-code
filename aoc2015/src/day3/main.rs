use aoc2015::{
    data_file,
    day3::{puzzle1::number_houses_received_present, puzzle2::number_houses_received_present_robo},
};

pub fn main() {
    println!(
        "D3P1: Santa has dropped at least one gift at {} houses.",
        number_houses_received_present(data_file("day3.txt"))
    );
    println!(
        "D3P2: Santa and Robosanta have dropped at least one gift at {} houses.",
        number_houses_received_present_robo(data_file("day3.txt"))
    );
}
