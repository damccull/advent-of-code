use std::path::PathBuf;

use super::{get_packages, Package};

pub fn get_ribbon_length(filename: PathBuf) -> u32 {
    calc_length(get_packages(filename))
}

fn calc_length(packages: Vec<Package>) -> u32 {
    packages.iter().fold(0_u32, |acc, package| {
        let l1 = package.sorted_lengths()[0];
        let l2 = package.sorted_lengths()[1];
        acc + l1 * 2 + l2 * 2 + package.length * package.width * package.height
    })
}

#[cfg(test)]
mod test {
    use crate::day2::{puzzle2::calc_length, Package};

    #[test]
    fn calc_length_is_correct() {
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
                result: 34,
            },
            TestData {
                package: vec![Package {
                    length: 1,
                    width: 1,
                    height: 10,
                }],
                result: 14,
            },
        ];

        for d in data {
            assert_eq!(
                d.result,
                calc_length(d.package.clone()),
                "Test case `{:?}` failed.",
                d.package
            );
        }
    }
}
