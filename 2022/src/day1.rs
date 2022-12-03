use advent_of_code_common::read_data_from_file;

fn main() -> Result<(), anyhow::Error> {
    let d = read_data_from_file("data/day1.txt")?;
    let d = process_data(d)?;
    match d.iter().map(|e| e.total_calories()).max() {
        Some(calories) => println!(
            "The elf with the most food has {} total calories.",
            calories
        ),
        None => println!("Apparently there were no elves that brought food..."),
    }

    // Need to sort the elves by top three calorie values
    let mut d = d;
    d.sort_by_key(|a| a.total_calories());
    let top_three_total: u32 = d.iter().rev().take(3).map(|e| e.total_calories()).sum();
    println!(
        "The top three elves are holding a combined total of {} calories.",
        top_three_total
    );
    Ok(())
}

fn process_data(data: Vec<String>) -> Result<Vec<Elf>, anyhow::Error> {
    let x = data
        .split(|d| d.is_empty())
        .map(|e| Ok(Elf::new(Food::from_string_array(e)?)))
        .collect::<Result<Vec<_>, _>>();
    x
}

#[derive(Debug)]
struct Elf {
    foods: Vec<Food>,
}
impl Elf {
    pub fn new(foods: Vec<Food>) -> Self {
        Self { foods }
    }
    pub fn total_calories(&self) -> u32 {
        self.foods.iter().map(|f| f.calories()).sum()
    }
}

#[derive(Debug)]
struct Food {
    calories: u32,
}
impl Food {
    pub fn new(calories: u32) -> Self {
        Food { calories }
    }

    pub fn from_string_array(strings: &[String]) -> Result<Vec<Food>, anyhow::Error> {
        strings
            .iter()
            .map(|f| {
                f.parse::<u32>()
                    .map(Food::new)
                    .map_err(|e| anyhow::anyhow!(e))
            })
            .collect::<Result<Vec<_>, _>>()
    }

    pub fn calories(&self) -> u32 {
        self.calories
    }
}

#[cfg(test)]
mod tests {
    use crate::process_data;

    fn test_data() -> Vec<String> {
        const TEST_DATA: &str = r##"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"##;
        TEST_DATA
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn parse_data_successful() {
        let data = process_data(test_data()).unwrap();

        let most_calories = data.iter().map(|e| e.total_calories()).max().unwrap();
        assert_eq!(most_calories, 24000);
    }
}
