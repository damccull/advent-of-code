use std::{collections::HashSet, hash::Hash};

use advent_of_code_common::read_data_from_file;

fn main() -> Result<(), anyhow::Error> {
    let data = process_data(read_data_from_file("data/day9.txt")?)?;
    let result = locations_seen_by_tail(data)?;
    println!("The tail of the rope was in {} locations", result.len());

    render_history(result)?;

    Ok(())
}

fn locations_seen_by_tail(data: Vec<Move>) -> Result<HashSet<Point>, anyhow::Error> {
    let mut head_last_location = Point::default();
    let mut tail_last_location = Point::default();
    let mut tail_visited_points = HashSet::new();
    for m in data {
        let (head, tail, tail_history) = match m {
            Move::Up(distance) => move_vertical(head_last_location, tail_last_location, distance),
            Move::Down(distance) => {
                move_vertical(head_last_location, tail_last_location, -distance)
            }
            Move::Left(distance) => {
                move_horizontal(head_last_location, tail_last_location, -distance)
            }
            Move::Right(distance) => {
                move_horizontal(head_last_location, tail_last_location, distance)
            }
        };
        head_last_location = head;
        tail_last_location = tail;
        for p in tail_history {
            tail_visited_points.insert(p);
        }
    }

    Ok(tail_visited_points)
}

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

    dbg!(min_x, max_x, min_y, max_y, width, height, offset_x, offset_y);
    let mut map = vec![vec!['.'; width as usize + 1]; height as usize + 1];

    for p in history {
        map[((p.y as isize) + offset_y) as usize][((p.x as isize) + offset_x) as usize] = '#';
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

fn move_vertical(head: Point, tail: Point, distance: i64) -> (Point, Point, HashSet<Point>) {
    let mut head = head;
    let mut tail = tail;
    let mut tail_history = HashSet::new();
    let unit_distance = distance.abs() / distance;

    for _ in 0..distance.abs() {
        head.y += unit_distance;

        if (tail.y - head.y).abs() > 1 {
            //Need to move the tail
            tail.y += unit_distance;
            //See if a diagonal move is needed
            if tail.x != head.x {
                let xdist = -(tail.x - head.x);
                tail.x += xdist;
            }
        }
        tail_history.insert(tail);
    }
    (head, tail, tail_history)
}

fn move_horizontal(head: Point, tail: Point, distance: i64) -> (Point, Point, HashSet<Point>) {
    let mut head = head;
    let mut tail = tail;
    let mut tail_history = HashSet::new();
    let unit_distance = distance.abs() / distance;

    for _ in 0..distance.abs() {
        head.x += unit_distance;

        if (tail.x - head.x).abs() > 1 {
            //Need to move the tail
            tail.x += unit_distance;
            //See if a diagonal move is needed
            if tail.y != head.y {
                let ydist = -(tail.y - head.y);
                tail.y += ydist;
            }
        }
        tail_history.insert(tail);
    }
    (head, tail, tail_history)
}

fn process_data(data: Vec<String>) -> Result<Vec<Move>, anyhow::Error> {
    data.iter()
        .map(|step| {
            step.split_once(' ')
                .ok_or_else(|| anyhow::anyhow!("Unable to split step string"))?
                .try_into()
        })
        .collect::<Result<Vec<Move>, anyhow::Error>>()
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, PartialEq, Eq)]
enum Move {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

impl TryFrom<(&str, &str)> for Move {
    type Error = anyhow::Error;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let (direction, distance) = value;
        let distance = distance.parse()?;
        let result = match direction.to_lowercase().as_str() {
            "u" => Ok(Move::Up(distance)),
            "d" => Ok(Move::Down(distance)),
            "r" => Ok(Move::Right(distance)),
            "l" => Ok(Move::Left(distance)),
            _ => Err(anyhow::anyhow!("Invalid step")),
        }?;

        Ok(result)
    }
}

// impl Add<Move> for Point {
//     type Output = Point;

//     fn add(self, rhs: Move) -> Self::Output {
//         Point {
//             x: self.x + rhs.x_distance,
//             y: self.y + rhs.y_distance,
//         }
//     }
// }

// struct Move {
//     pub x_distance: i32,
//     pub y_distance: i32,
// }

#[cfg(test)]
mod tests {
    use crate::{locations_seen_by_tail, process_data, Move};

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

    #[test]
    fn process_data_works() {
        let data = process_data(test_data()).unwrap();

        assert!(!data.is_empty());
        assert_eq!(data[0], Move::Right(4));
    }

    #[test]
    fn locations_seen_by_tail_works() {
        let data = process_data(test_data()).unwrap();

        let result = locations_seen_by_tail(data).unwrap();

        assert_eq!(result.len(), 13);
    }
}
