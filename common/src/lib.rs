use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_data_from_file(filepath: &str) -> Result<Vec<String>, anyhow::Error> {
    let file = File::open(filepath)?;
    let lines = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    Ok(lines)
}

pub type Lines = std::io::Lines<BufReader<File>>;

pub fn read_lines_from_file(filepath: &str) -> Result<Lines, anyhow::Error> {
    let file = File::open(filepath)?;
    let lines = BufReader::new(file).lines();

    Ok(lines)
}
