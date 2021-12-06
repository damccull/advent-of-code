use std::path::PathBuf;

use crate::{data_file, day6::puzzle1::count_fish_in_pond, read_lines};

pub mod puzzle1;
pub mod puzzle2;

pub fn run() {
    let pond = get_data(data_file("day6.txt"));

    println!("D6P1: Fish in the pond: {}", count_fish_in_pond(pond));
}

#[derive(Clone, Copy, Debug)]
pub struct Fish {
    age: u32,
}

pub fn get_data(filename: PathBuf) -> Vec<Fish> {
    let mut x = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let mut fishies = line
                .split(',')
                .map(|f| Fish {
                    age: f.parse().unwrap(),
                })
                .collect::<Vec<_>>();
            x.append(&mut fishies);
        }
    }
    x
}
