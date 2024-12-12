#[allow(unused_imports)]
use advent_of_code_common::read_data_from_file;

fn main() -> Result<(), anyhow::Error> {
    let data = include_str!("../data/day5.txt");
    let result = crate::puzzle1(data).unwrap();
    println!("Day 5 Puzzle 1: {}", result);

    let result = crate::puzzle2(data).unwrap();
    println!("Day 5 Puzzle 2: {}", result);
    Ok(())
}

fn puzzle1(_input: &str) -> Result<u32, anyhow::Error> {
    todo!()
}

fn puzzle2(_input: &str) -> Result<u32, anyhow::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn puzzle1() {
        let data = include_str!("../data/d5p1-test.txt");
        let result = crate::puzzle1(data);
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn puzzle2() {
        let data = include_str!("../data/d5p2-test.txt");
        let result = crate::puzzle2(data);
        assert_eq!(result.unwrap(), 281);
    }
}
