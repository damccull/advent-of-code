use crate::common::extract_depths;

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
