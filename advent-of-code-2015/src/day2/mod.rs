use std::path::PathBuf;

use crate::read_lines;

pub mod puzzle1;
pub mod puzzle2;

#[derive(Debug, Clone)]
pub struct Package {
    pub length: u32,
    pub width: u32,
    pub height: u32,
}
impl Package {
    pub fn surface_area(&self) -> u32 {
        (2 * self.length * self.width)
            + (2 * self.length * self.height)
            + (2 * self.width * self.height)
    }

    pub fn smallest_side_area(&self) -> u32 {
        let lw = self.length * self.width;
        let lh = self.length * self.height;
        let wh = self.width * self.height;

        let mut smallest_side = vec![lw, lh, wh];
        smallest_side.sort_unstable();
        smallest_side[0]
    }

    pub fn sorted_lengths(&self) -> Vec<u32> {
        let mut sorted = vec![self.length, self.width, self.height];
        sorted.sort();
        sorted
    }
}

pub fn get_packages(filename: PathBuf) -> Vec<Package> {
    let mut result = Vec::<Package>::new();
    if let Ok(lines) = read_lines(filename) {
        let mut x: Vec<Package> = lines
            .map(|line| {
                let package = line.expect("Bad data. Unable to unwrap package line.");
                let package = package.split('x').collect::<Vec<_>>();
                let length = package[0].parse().expect("Unable to parse package.");
                let width = package[1].parse().expect("Unable to parse package.");
                let height = package[2].parse().expect("Unable to parse package.");
                Package {
                    length,
                    width,
                    height,
                }
            })
            .collect();
        result.append(&mut x);
    }
    result
}

#[cfg(test)]
mod test {
    use super::Package;

    #[test]
    fn package_smallest_side_area_is_correct() {
        let p = Package {
            length: 1,
            width: 2,
            height: 3,
        };
        assert_eq!(p.smallest_side_area(), 2);
    }

    #[test]
    fn package_suface_area_is_correct() {
        let p = Package {
            length: 2,
            width: 3,
            height: 4,
        };

        assert_eq!(p.surface_area(), 52);
    }
}
