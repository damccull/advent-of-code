use aoc2015::{
    data_file,
    day2::{puzzle1::get_wrapping_paper_sqft, puzzle2::get_ribbon_length},
};

pub fn main() {
    println!(
        "D2P1: The elves need to order {} square feet of wrapping paper.",
        get_wrapping_paper_sqft(data_file("day2.txt"))
    );
    println!(
        "D2P2: The elves need to order {} feet of ribbon.",
        get_ribbon_length(data_file("day2.txt"))
    );
}
