use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_data_from_file(filepath: String) -> Result<Vec<String>, anyhow::Error> {
    let file = File::open(filepath)?;
    let lines = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    Ok(lines)
}
