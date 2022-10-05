use std::{collections::VecDeque, path::PathBuf};

use aoclib::read_lines;

use crate::data_file;

pub fn run() {
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

#[derive(Clone, Copy, Debug)]
pub struct Fish;

pub fn get_data(filename: PathBuf) -> Vec<usize> {
    let mut x: Vec<usize> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let mut fishies = line
                .split(',')
                .map(|f| f.parse().unwrap())
                .collect::<Vec<usize>>();
            x.append(&mut fishies);
        }
    }
    x
}

pub fn count_fish_in_pond(fish: &[usize], days: usize) -> usize {
    let fish = Vec::from(fish);
    let mut pond = vec![0_usize; 9];
    //build the initial state
    for f in fish {
        pond[f] += 1;
    }

    let mut pond = VecDeque::from(pond);
    for _day in 0..days {
        let net = pond[0];
        let hatchery = pond[0];
        pond[0] = 0;
        pond.rotate_left(1);
        pond[6] += net;
        pond[8] = hatchery;
    }
    pond.make_contiguous().iter().sum::<usize>()
}
