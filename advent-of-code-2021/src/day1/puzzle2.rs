use super::extract_depths;

pub fn count_depths_windowed(filename: &str, window_size: usize) -> i32 {
    let depths = extract_depths(filename);

    // Iterate over the depths and accumulate a count of depths that increase
    // from one to the next. This compares the iterator to itself but starting in the next position.
    // The `zip` function creates an iterator of tuples, where each element contains the current depth
    // and the next depth.
    //
    // The `windows` function creates a new slice of `window_size` elements of the vector. This works
    // just like `count_depths` except that it sums the window of 3 depths before it compares them.
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
