use std::path::PathBuf;

use super::{get_packages, Package};

pub fn get_wrapping_paper_sqft(filename: PathBuf) -> u32 {
    calc_sqft(get_packages(filename))
}

fn calc_sqft(packages: Vec<Package>) -> u32 {
    packages.iter().fold(0_u32, |acc, package| {
        acc + package.surface_area() + package.smallest_side_area()
    })
}

#[cfg(test)]
mod test {

    use crate::day2::{puzzle1::calc_sqft, Package};

    #[test]
    fn get_wrapping_paper_sqft_returns_correct_amount() {
        #[derive(Debug)]
        struct TestData {
            package: Vec<Package>,
            result: u32,
        }

        let data = vec![
            TestData {
                package: vec![Package {
                    length: 2,
                    width: 3,
                    height: 4,
                }],
                result: 58,
            },
            TestData {
                package: vec![Package {
                    length: 1,
                    width: 1,
                    height: 10,
                }],
                result: 43,
            },
        ];

        for d in data {
            assert_eq!(
                d.result,
                calc_sqft(d.package.clone()),
                "Test case `{:?}` failed.",
                d.package
            );
        }
    }
}
