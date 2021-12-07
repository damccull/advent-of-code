use aoc2015::{day1, day2, day3, /*day4,*/ day5, day6};

fn main() {
    day1::run();
    day2::run();
    day3::run();
    //day4::run(); // Runs slow due to lots of single-core MD5 hashing in a row
    day5::run();
    day6::run();
}
