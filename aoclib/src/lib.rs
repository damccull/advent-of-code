use std::{
    fs::File,
    io::{self, BufRead},
    ops::Add,
    path::{Path, PathBuf},
    str::FromStr,
};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn data_file(base_directory: &str, filename: &str) -> PathBuf {
    let basepathstr = format!("{}/input", base_directory);
    let base = Path::new(&basepathstr);
    base.join(filename)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}
impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.split(',').collect::<Vec<_>>();
        let x: isize = match c[0].parse() {
            Ok(x) => x,
            Err(e) => return Err(format!("Error parsing X coordinate: {}", e)),
        };
        let y: isize = match c[1].parse() {
            Ok(y) => y,
            Err(e) => return Err(format!("Error parsing Y coordinate: {}", e)),
        };
        Ok(Point { x, y })
    }
}
