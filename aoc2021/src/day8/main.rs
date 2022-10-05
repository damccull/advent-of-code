use aoc2021::day8::{get_data, Notebook};

fn main() -> Result<(), anyhow::Error> {
    let notebook = Notebook::try_from(get_data("day8.txt"));

    dbg!(notebook);

    Ok(())
}
