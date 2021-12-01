use crate::common::extract_depths;

pub fn count_depths(filename: &str) -> i32 {
    // Get a vector of depts from the file
    let depths = extract_depths(filename);

    // Iterate over the depths and accumulate a count of depths that increase
    // from one to the next. This compares the iterator to itself but starting in the next position.
    // The `zip` function creates an iterator of tuples, where each element contains the current depth
    // and the next depth.
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
