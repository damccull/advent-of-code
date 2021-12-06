use std::path::PathBuf;

use crate::{data_file, read_lines};

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

pub fn count_fish_in_pond(pond: Vec<Fish>) -> usize {
    let mut pond = pond;
    //dbg!(&pond);
    for _day in 0..80 {
        let fishies = &pond.clone();
        for (i, fish) in fishies.iter().enumerate() {
            if fish.age == 0 {
                pond[i].age = 7;
                pond.push(Fish { age: 8 });
            }
            pond[i].age -= 1;
        }
    }
    pond.len()
}
