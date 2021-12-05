use std::path::PathBuf;

use unicode_segmentation::UnicodeSegmentation;

use crate::{data_file, day4::puzzle2::find_winning_board_id, read_lines};

pub mod puzzle1;
pub mod puzzle2;

pub fn run() {
    let g = build_game(data_file("day4.txt"));
    let id = find_winning_board_id(g);
    println!("D4P1: The winning board ID is {}.", id);
}

type BingoBoard = Vec<(u32, bool)>;
#[derive(Debug)]
pub struct BingoGame {
    draws: Vec<u32>,
    boards: Vec<BingoBoard>,
}
impl BingoGame {
    pub fn new_from_file(filename: PathBuf) -> Self {
        build_game(filename)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Coordinate {
    x: u32,
    y: u32,
}

fn build_game(filename: PathBuf) -> BingoGame {
    let data = read_data(filename);
    let mut data_iter = data.iter();
    let draws: Vec<u32> = data_iter
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let mut boards = Vec::new();
    let mut board: Vec<(u32, bool)> = Vec::new();
    for line in data_iter {
        if line.is_empty() {
            if !&board.is_empty() {
                boards.push(board.clone());
                board.clear();
            }
            continue;
        }

        board.append(&mut parse_line(line));
    }
    BingoGame { draws, boards }
}

fn parse_line(line: &str) -> BingoBoard {
    let result = UnicodeSegmentation::graphemes(line, true).collect::<Vec<&str>>();
    result
        .chunks(3)
        .map(|g| {
            (
                g.join("")
                    .trim()
                    .parse()
                    .expect("Unable to parse number from graphemes."),
                false,
            )
        })
        .collect::<BingoBoard>()
}

fn read_data(filename: PathBuf) -> Vec<String> {
    let mut result = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::day4::parse_line;

    #[test]
    fn parse_line_works() {
        let test_data = vec![
            (
                "22 13 17 11  0".to_string(),
                vec![
                    (22, false),
                    (13, false),
                    (17, false),
                    (11, false),
                    (0, false),
                ],
            ),
            (
                " 8  2 23  4 24".to_string(),
                vec![(8, false), (2, false), (23, false), (4, false), (24, false)],
            ),
            (
                "21  9 14 16  7".to_string(),
                vec![
                    (21, false),
                    (9, false),
                    (14, false),
                    (16, false),
                    (7, false),
                ],
            ),
            (
                " 6 10  3 18  5".to_string(),
                vec![(6, false), (10, false), (3, false), (18, false), (5, false)],
            ),
            (
                " 1 12 20 15 19".to_string(),
                vec![
                    (1, false),
                    (12, false),
                    (20, false),
                    (15, false),
                    (19, false),
                ],
            ),
        ];

        for (d, r) in test_data {
            assert_eq!(r, parse_line(&d), "Broken case: {}", d);
        }
    }
}
