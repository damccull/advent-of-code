use crate::day4::puzzles::mine_advent_coin;

pub mod puzzles;

pub fn run() {
    let seed = "ckczppom".to_string();
    let difficulty = 5;
    if let Ok(i) = mine_advent_coin(&seed, 5) {
        println!(
            "D4P1: The first AdventCoin was found with seed {} with difficulty {} was at number {}.",
            &seed, difficulty, i
        );
    } else {
        println!("Something went wrong finding the hash.")
    }

    let difficulty = 6;
    if let Ok(i) = mine_advent_coin(&seed, difficulty) {
        println!(
            "D4P2: The first AdventCoin was found with seed {} with difficulty {} was at number {}.",
            &seed, difficulty, i
        );
    } else {
        println!("Something went wrong finding the hash.")
    }
}
