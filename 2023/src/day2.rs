#[allow(unused_imports)]
use advent_of_code_common::read_data_from_file;
use nom::{bytes::complete::tag, character::complete::{u32, line_ending}, Parser, sequence::separated_pair};

fn main() -> Result<(), anyhow::Error> {
    let data = include_str!("../data/day2puzzle1-test.txt");
    dbg!(parse_game().parse(data)?);
    Ok(())
}

fn parse_game<'i: 't, 't>() -> impl Parser<&'i str, u32, ()> + 't {
    move |input| {
        let (tail, _) = tag("Game ").parse(input)?;
        let (tail, value2) = u32(tail)?;
        let (tail,_) = tag(": ").parse(tail)?;
        Ok((tail, value2))
    }
}

// type NomResult<T, U> = IResult<T, U, VerboseError<T>>;

// #[derive(Debug)]
// struct Game {
//     id: u32,
//     draws: Vec<Draw>,
// }

// #[derive(Debug)]
// struct Draw {
//     reds: u32,
//     greens: u32,
//     blues: u32,
// }
#[cfg(test)]
mod tests {}
