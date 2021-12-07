use std::collections::HashMap;

pub fn find_minimum_fuel(distances: &[usize]) -> usize {
    type Position = usize;
    type TotalFuel = usize;
    let mut counters = HashMap::<Position, TotalFuel>::new();

    for i in 0..distances.len() {
        let x = distances.iter().fold(
            0_usize,
            |acc, &d| if i < d { acc + (d - i) } else { acc + (i - d) },
        );
        counters.insert(i, x);
    }

    counters.iter().fold(
        0_usize,
        |acc, (_, &f)| {
            if acc == 0 || f < acc {
                f
            } else {
                acc
            }
        },
    )
}

#[cfg(test)]
mod test {
    use super::find_minimum_fuel;

    #[test]
    fn find_minimum_fuel_works() {
        let test_data: Vec<usize> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let r = find_minimum_fuel(&test_data);
        assert_eq!(r, 37);
    }
}
