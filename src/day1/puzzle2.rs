use crate::common::extract_depths;

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
