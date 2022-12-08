use advent_of_code_common::read_data_from_file;

fn main() -> Result<(), anyhow::Error> {
    let data = read_data_from_file("data/day6.txt")?
        .pop()
        .ok_or_else(|| anyhow::anyhow!("Unable to get data"))?;
    let result = find_marker(&data, 4)
        .ok_or_else(|| anyhow::anyhow!("Unable to find the packet start marker"))?;
    println!("Packet start marker is complete at character {}", result);

    let result =
        find_marker(&data, 14).ok_or_else(|| anyhow::anyhow!("Unable to find message marker"))?;
    println!(
        "The first start of message marker is at character {}",
        result,
    );
    Ok(())
}

fn find_marker(data: &str, number_distinct_characters: usize) -> Option<usize> {
    Some(
        data.as_bytes()
            .windows(number_distinct_characters)
            .position(|x| {
                for (index1, b) in x.iter().enumerate() {
                    for (index2, c) in x.iter().enumerate() {
                        if index1 == index2 {
                            continue;
                        }
                        if b == c {
                            return false;
                        }
                    }
                }
                true
            })?
            + number_distinct_characters,
    )
}

#[cfg(test)]
mod tests {
    use crate::find_marker;

    #[test]
    fn find_marker_start_works() {
        let data = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4, 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 4, 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 4, 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4, 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4, 11),
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14, 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 14, 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 14, 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14, 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14, 26),
        ];

        for (test, numchars, answer) in data {
            assert_eq!(
                find_marker(test, numchars).unwrap(),
                answer,
                "Couldn't find answer: {}",
                test
            );
        }
    }
}
