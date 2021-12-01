use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn count_depths(filename: &str) -> i32 {
    let depths = extract_depths(filename);
    depths.iter().zip(depths.iter().skip(1)).fold(
        0_i32,
        |sum, (current, next)| {
            if current < next {
                sum + 1
            } else {
                sum
            }
        },
    )
}

pub fn count_depths_windowed(filename: &str, window_size: usize) -> i32 {
    let depths = extract_depths(filename);
    depths
        .windows(window_size)
        .zip(depths.windows(window_size).skip(1))
        .fold(0_i32, |sum, (current, next)| {
            if current.iter().sum::<i32>() < next.iter().sum() {
                sum + 1
            } else {
                sum
            }
        })
}

fn extract_depths(filename: &str) -> Vec<i32> {
    let mut depths = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consume iterator, return an (Optional) String
        for line in lines.flatten() {
            depths.push(line.parse().expect("Failed to parse the file"));
        }
    }
    depths
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
