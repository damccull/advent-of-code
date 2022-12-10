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

fn rope_movement(
    data: Vec<Move>,
    number_knots: usize,
) -> Result<Vec<HashSet<Point>>, anyhow::Error> {
    let mut last_locations = vec![Point::default(); number_knots];
    let mut knot_histories = vec![HashSet::<Point>::new(); number_knots];

    for m in data {
        match m {
            Move::Up(distance) => {
                move_vertical_advanced(&mut last_locations, &mut knot_histories, distance)
            }
            Move::Down(distance) => {
                move_vertical_advanced(&mut last_locations, &mut knot_histories, -distance)
            }
            Move::Left(distance) => {
                move_horizontal_advanced(&mut last_locations, &mut knot_histories, -distance)
            }
            Move::Right(distance) => {
                move_horizontal_advanced(&mut last_locations, &mut knot_histories, distance)
            }
        };
    }
    Ok(knot_histories)
}

fn move_vertical_advanced(
    last_locations: &mut Vec<Point>,
    histories: &mut [HashSet<Point>],
    distance: i64,
) {
    let unit_distance = distance.abs() / distance;

    for _ in 0..distance.abs() {
        last_locations[0].y += unit_distance;
        histories[0].insert(last_locations[0]);

        for i in 1..last_locations.len() - 1 {
            let lead = last_locations[i];
            let mut trail = last_locations[i + 1];
            if (trail.y - lead.y).abs() > 1 {
                trail.y += unit_distance;

                //Check if diagonal movement is necessary
                if trail.x != lead.x {
                    let xdist = -(trail.x - lead.x);
                    trail.x += xdist;
                }
            }
        }
    }
}

fn move_horizontal_advanced(
    last_locations: &mut Vec<Point>,
    histories: &mut [HashSet<Point>],
    distance: i64,
) {
    let unit_distance = distance.abs() / distance;

    for _ in 0..distance.abs() {
        last_locations[0].x += unit_distance;
        histories[0].insert(last_locations[0]);

        for i in 1..last_locations.len() - 1 {
            let lead = last_locations[i];
            let trail = &mut last_locations[i + 1];
            if (trail.x - lead.x).abs() > 1 {
                trail.x += unit_distance;

                //Check if diagonal movement is necessary
                if trail.y != lead.y {
                    let ydist = -(trail.y - lead.y);
                    trail.y += ydist;
                }
            }
            //last_locations[i] = trail.clone();
            histories[i].insert(*trail);
        }
    }

    dbg!(last_locations);
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

fn move_vertical(
    lead_knot: Point,
    follower_knot: Point,
    distance: i64,
) -> (Point, Point, HashSet<Point>) {
    let mut lead_knot = lead_knot;
    let mut follower_knot = follower_knot;
    let mut follower_history = HashSet::new();
    let unit_distance = distance.abs() / distance;

    for _ in 0..distance.abs() {
        lead_knot.y += unit_distance;

        if (follower_knot.y - lead_knot.y).abs() > 1 {
            //Need to move the tail
            follower_knot.y += unit_distance;
            //See if a diagonal move is needed
            if follower_knot.x != lead_knot.x {
                let xdist = -(follower_knot.x - lead_knot.x);
                follower_knot.x += xdist;
            }
        }
        follower_history.insert(follower_knot);
    }
    (lead_knot, follower_knot, follower_history)
}

fn move_horizontal(
    lead_knot: Point,
    follower_knot: Point,
    distance: i64,
) -> (Point, Point, HashSet<Point>) {
    let mut lead_knot = lead_knot;
    let mut follower_knot = follower_knot;
    let mut follower_history = HashSet::new();
    let unit_distance = distance.abs() / distance;

    for _ in 0..distance.abs() {
        lead_knot.x += unit_distance;

        if (follower_knot.x - lead_knot.x).abs() > 1 {
            //Need to move the tail
            follower_knot.x += unit_distance;
            //See if a diagonal move is needed
            if follower_knot.y != lead_knot.y {
                let ydist = -(follower_knot.y - lead_knot.y);
                follower_knot.y += ydist;
            }
        }
        follower_history.insert(follower_knot);
    }
    (lead_knot, follower_knot, follower_history)
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
    use crate::{locations_seen_by_tail, process_data, rope_movement, Move};

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
        assert_eq!(data[0], Move::Right(4));
    }

    #[test]
    fn locations_seen_by_tail_works() {
        let data = process_data(test_data()).unwrap();

        let result = locations_seen_by_tail(data).unwrap();

        assert_eq!(result.len(), 13);
    }

    #[test]
    fn rope_movement_works() {
        let data = process_data(test_data2()).unwrap();

        let result = rope_movement(data, 10).unwrap();

        assert_eq!(result[9].len(), 36);
    }
}
