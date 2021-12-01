use puzzle1_sonar_sweep::{count_depths, count_depths_windowed};

fn main() {
    println!("Depth Increases: {}", count_depths("input.txt"));
    println!(
        "Depth Increases (3-measure window): {}",
        count_depths_windowed("input.txt", 3)
    );
}
