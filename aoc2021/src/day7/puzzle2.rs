use std::collections::HashMap;

pub fn find_minimum_fuel_increasing_rate(distances: &[usize]) -> usize {
    type Position = usize;
    type TotalFuel = usize;
    let mut counters = HashMap::<Position, TotalFuel>::new();

    for i in 0..distances.len() {
        let x = distances.iter().fold(0_usize, |acc, &d| {
            if i < d {
                acc + total_fuel_for_distance(d - i)
            } else {
                acc + total_fuel_for_distance(i - d)
            }
        });
        counters.insert(i, x);
    }

    counters.iter().fold(
        0_usize,
        |acc, (&d, &f)| {
            if acc == 0 || f < acc {
                f
            } else {
                acc
            }
        },
    )
}

fn total_fuel_for_distance(distance: usize) -> usize {
    let mut x = 0;
    for i in 0..=distance {
        x += i
    }
    x
}

#[cfg(test)]
mod test {
    use super::find_minimum_fuel_increasing_rate;

    #[test]
    fn find_minimum_fuel_increasing_rate_works() {
        let test_data: Vec<usize> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let r = find_minimum_fuel_increasing_rate(&test_data);
        assert_eq!(r, 168);
    }
}
