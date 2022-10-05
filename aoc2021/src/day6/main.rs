use aoc2021::{
    data_file,
    day6::{count_fish_in_pond, get_data},
};

fn main() {
    let pond = get_data(data_file("day6.txt"));
    //let pond = vec![3, 4, 3, 1, 2];
    println!(
        "D6P1: Fish in the pond after 80 days: {}",
        count_fish_in_pond(&pond, 80)
    );

    println!(
        "D6P2: Fish in the pond after 256 days: {}",
        count_fish_in_pond(&pond, 256),
    );
}
