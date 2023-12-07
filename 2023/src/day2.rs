#[allow(unused_imports)]
use advent_of_code_common::read_data_from_file;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, multispace1, u32},
    error::{ErrorKind, ParseError, VerboseError},
    error_position,
    sequence::separated_pair,
    IResult, Parser,
};

fn main() -> Result<(), anyhow::Error> {
    let data = include_str!("../data/day2puzzle1-test.txt");
    dbg!(parse_game().parse(data)?);
    Ok(())
}

fn parse_game<'i: 't, 't>() -> impl Parser<&'i str, u32, ()> + 't {
    move |input| {
        let (tail, _) = tag("Game ").parse(input)?;
        let (tail, value2) = u32(tail)?;
        let (tail, _) = tag(": ").parse(tail)?;
        Ok((tail, value2))
    }
}

fn parse_draw<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Draw, E> {
    let (tail, color) = parse_color(input)?;
    Ok((
        tail,
        Draw {
            reds: 0,
            greens: 0,
            blues: 0,
        },
    ))
}

fn parse_color<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Color, E> {
    let (tail, (number, parsed_color)) = match separated_pair(
        u32,
        multispace1,
        alt((tag("red"), tag("green"), tag("blue"))),
    )
    .parse(input)
    {
        Ok(result) => result,
        Err(err) => return Err(err),
    };
    let colorerror = match parsed_color {
        "red" => return Ok((tail, Color::Red(number))),
        "green" => return Ok((tail, Color::Green(number))),
        "blue" => return Ok((tail, Color::Blue(number))),
        _ => nom::Err::Error(error_position!(input, ErrorKind::Not)),
    };
    Err(colorerror)
}

// type NomResult<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

#[derive(Debug)]
struct Draw {
    reds: u32,
    greens: u32,
    blues: u32,
}
#[derive(Debug)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}
#[cfg(test)]
mod tests {}
