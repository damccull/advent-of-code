use aoc2021::day8::{get_data, Notebook};

fn main() -> Result<(), anyhow::Error> {
    let notebook = Notebook::try_from(get_data("day8.txt"));

    let puzzle1 = notebook?.count_simple_output_digits();
    println!(
        "The total number of times the digits 1, 4, 7, or 8 appear in the outputs is {}",
        puzzle1
    );



    Ok(())
}
