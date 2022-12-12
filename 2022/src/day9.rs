use std::{
    collections::HashSet,
    hash::Hash,
    ops::{Add, Sub},
};

use advent_of_code_common::read_data_from_file;

fn main() -> Result<(), anyhow::Error> {
    let data = process_data(read_data_from_file("data/day9.txt")?)?;
    let result = rope_movement(data.clone(), 2)?.unique_locations;
    println!(
        "The tail of the rope was in {} locations",
        result[result.len() - 1].len()
    );

    let result = rope_movement(data, 10)?.unique_locations;
    //render_history(result[result.len() - 1].clone())?;
    println!(
        "The tail of the 10-knot rope was in {} locations",
        result[result.len() - 1].len()
    );

    Ok(())
}

fn rope_movement(data: Vec<Instruction>, number_knots: usize) -> Result<RopeResult, anyhow::Error> {
    let mut knot_histories = vec![vec![Point::default(); 1]; number_knots];
    let mut unique_locations = vec![HashSet::<Point>::new(); number_knots];
    for history in unique_locations.iter_mut() {
        history.insert(Point::default());
    }

    let moves = instructions_to_moves(data);

    for m in moves {
        let last_head_location = &knot_histories[0][knot_histories[0].len() - 1];

        let new_head_location = last_head_location + m;
        knot_histories[0].push(new_head_location);
        unique_locations[0].insert(new_head_location);

        for i in 1..knot_histories.len() {
            let lead = knot_histories[i - 1].clone();
            let trail = &mut knot_histories[i];

            let lead_current_location = lead[lead.len() - 1];
            let trail_current_position = trail[trail.len() - 1];
            let difference = lead_current_location - trail_current_position;

            if difference.x.abs() > 1 {
                // Move x
                let xmove_x = trail_current_position.x + difference.unit().x;
                let mut xmove_y = trail_current_position.y;
                if difference.y != 0 {
                    // Diagonal move
                    xmove_y = trail_current_position.y + (difference.unit().y);
                }
                let p = Point {
                    x: xmove_x,
                    y: xmove_y,
                };
                trail.push(p);
                unique_locations[i].insert(p);
            } else if difference.y.abs() > 1 {
                // Move y

                let ymove_y = trail_current_position.y + difference.unit().y;
                let mut ymove_x = trail_current_position.x;
                if difference.x != 0 {
                    // Diagonal move
                    ymove_x = trail_current_position.x + (difference.unit().x);
                }
                let p = Point {
                    x: ymove_x,
                    y: ymove_y,
                };
                trail.push(p);
                unique_locations[i].insert(p);
            }
        }
    }
    let result = RopeResult {
        histories: knot_histories,
        unique_locations,
    };
    Ok(result)
}

fn instructions_to_moves(data: Vec<Instruction>) -> Vec<Move> {
    let mut moves = Vec::new();
    for m in data {
        match m {
            Instruction::Up(distance) => {
                for _ in 0..distance {
                    moves.push(Move { x: 0, y: 1 });
                }
            }
            Instruction::Down(distance) => {
                for _ in 0..distance {
                    moves.push(Move { x: 0, y: -1 });
                }
            }
            Instruction::Left(distance) => {
                for _ in 0..distance {
                    moves.push(Move { x: -1, y: 0 });
                }
            }
            Instruction::Right(distance) => {
                for _ in 0..distance {
                    moves.push(Move { x: 1, y: 0 });
                }
            }
        };
    }
    moves
}

#[allow(dead_code)]
fn render_history(history: HashSet<Point>) -> Result<(), anyhow::Error> {
    let max_y = history
        .iter()
        .map(|p| p.y)
        .max()
        .ok_or_else(|| anyhow::anyhow!("Could not find map height"))? as isize;
    let min_y = history
        .iter()
        .map(|p| p.y)
        .min()
        .ok_or_else(|| anyhow::anyhow!("Could not find map height"))? as isize;
    let max_x = history
        .iter()
        .map(|p| p.x)
        .max()
        .ok_or_else(|| anyhow::anyhow!("Could not find map height"))? as isize;
    let min_x = history
        .iter()
        .map(|p| p.x)
        .min()
        .ok_or_else(|| anyhow::anyhow!("Could not find map height"))? as isize;
    let width = max_x - min_x;
    let height = max_y - min_y;
    let offset_x = min_x.abs();
    let offset_y = min_y.abs();

    //dbg!(min_x, max_x, min_y, max_y, width, height, offset_x, offset_y);
    let mut map = vec![vec!['.'; width as usize + 1]; height as usize + 1];

    for p in history {
        let c = if p.x == 0 && p.y == 0 { 'S' } else { '#' };
        map[((p.y as isize) + offset_y) as usize][((p.x as isize) + offset_x) as usize] = c;
    }
    for line in map.into_iter().rev() {
        let mut s = String::default();
        for c in line {
            s.push(c);
        }
        println!("{}", s);
    }
    Ok(())
}

fn process_data(data: Vec<String>) -> Result<Vec<Instruction>, anyhow::Error> {
    data.iter()
        .map(|step| {
            step.split_once(' ')
                .ok_or_else(|| anyhow::anyhow!("Unable to split step string"))?
                .try_into()
        })
        .collect::<Result<Vec<Instruction>, anyhow::Error>>()
}

#[allow(dead_code)]
#[derive(Debug)]
struct RopeResult {
    histories: Vec<Vec<Point>>,
    unique_locations: Vec<HashSet<Point>>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    pub x: i64,
    pub y: i64,
}
impl Point {
    pub fn unit(&self) -> Self {
        let x = if self.x == 0 {
            0
        } else {
            self.x.abs() / self.x
        };
        let y = if self.y == 0 {
            0
        } else {
            self.y.abs() / self.y
        };
        Self { x, y }
    }
}

impl Add<Move> for Point {
    type Output = Point;

    fn add(self, rhs: Move) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Add<Move> for &Point {
    type Output = Point;

    fn add(self, rhs: Move) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Sub<Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

impl TryFrom<(&str, &str)> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let (direction, distance) = value;
        let distance = distance.parse()?;
        let result = match direction.to_lowercase().as_str() {
            "u" => Ok(Instruction::Up(distance)),
            "d" => Ok(Instruction::Down(distance)),
            "r" => Ok(Instruction::Right(distance)),
            "l" => Ok(Instruction::Left(distance)),
            _ => Err(anyhow::anyhow!("Invalid step")),
        }?;

        Ok(result)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Move {
    pub x: i64,
    pub y: i64,
}

#[cfg(test)]
mod tests {
    use crate::{process_data, rope_movement, Instruction};

    fn test_data() -> Vec<String> {
        r##"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"##
            .lines()
            .map(|s| s.to_string())
            .collect()
    }

    fn test_data2() -> Vec<String> {
        r##"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"##
            .lines()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn process_data_works() {
        let data = process_data(test_data()).unwrap();

        assert!(!data.is_empty());
        assert_eq!(data[0], Instruction::Right(4));
    }

    #[test]
    fn locations_seen_by_tail_works() {
        let data = process_data(test_data()).unwrap();

        let result = rope_movement(data, 2).unwrap();

        assert_eq!(
            result.unique_locations[result.unique_locations.len() - 1].len(),
            13
        );
    }

    #[test]
    fn rope_movement_works() {
        let data = process_data(test_data2()).unwrap();

        let result = rope_movement(data, 10).unwrap();

        assert_eq!(
            result.unique_locations[result.unique_locations.len() - 1].len(),
            36
        );
    }
}
