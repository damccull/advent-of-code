use std::fmt::Error;

pub fn mine_advent_coin(seed: &str, beginning_zeroes: usize) -> Result<u64, Error> {
    for i in 1.. {
        let hashme = format!("{}{}", seed, i);
        let digest = md5::compute(hashme);
        let hash = format!("{:x}", digest);
        if hash.starts_with(&"0".repeat(beginning_zeroes)) {
            return Ok(i);
        }
    }
    Err(Error)
}

// #[cfg(test)]
// // These are slow due to lots of single-core MD5 hashing in a row
// mod test {
//     use crate::day4::puzzles::mine_advent_coin;

//     #[test]
//     fn mine_advent_coin_works() {
//         let data = vec![("abcdef", 609043_u64), ("pqrstuv", 1048970_u64)];

//         for (seed, i) in data {
//             assert_eq!(mine_advent_coin(seed, 5), Ok(i));
//         }
//     }
// }
