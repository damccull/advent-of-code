use advent_of_code_common::read_data_from_file;

type Map = Vec<Vec<char>>;

fn main() -> Result<(), anyhow::Error> {
    let data = process_data(read_data_from_file("data/day5.txt")?)?;
    let result = crate_mover_9000(data.clone())?;
    println!("The top crates for Crate Mover 9000 are: {}", result);

    let result = crate_mover_9001(data)?;
    println!("The top crates for Crate Mover 9001 are: {}", result);
    Ok(())
}

fn crate_mover_9000(instructions: InstructionSet) -> Result<String, anyhow::Error> {
    let mut map = instructions.map;
    for step in instructions.steps {
        for _ in 0..step.quantity {
            let c = map[step.source - 1]
                .pop()
                .ok_or_else(|| anyhow::anyhow!("Unable to move crates, some unknown error"))?;
            map[step.destination - 1].push(c);
        }
    }
    let mut result = String::default();
    for mut column in map {
        result.push(column.pop().unwrap_or_default());
    }
    Ok(result)
}

fn crate_mover_9001(instructions: InstructionSet) -> Result<String, anyhow::Error> {
    let mut map = instructions.map;

    for step in instructions.steps {
        let mut temp_stack = Vec::new();
        for _ in 0..step.quantity {
            let c = map[step.source - 1]
                .pop()
                .ok_or_else(|| anyhow::anyhow!("Unable to move crates, some unknown error"))?;
            temp_stack.push(c);
        }
        for _ in 0..step.quantity {
            let c = temp_stack
                .pop()
                .ok_or_else(|| anyhow::anyhow!("Unable to pop from temp_stack"))?;
            map[step.destination - 1].push(c);
        }
    }

    let mut result = String::default();
    for mut column in map {
        result.push(column.pop().unwrap_or_default());
    }

    Ok(result)
}

fn process_data(data: Vec<String>) -> Result<InstructionSet, anyhow::Error> {
    let x = data.split(|line| line.is_empty()).collect::<Vec<_>>();
    let mut drawing = x[0].to_vec();

    let columns = drawing
        .pop()
        .ok_or_else(|| anyhow::anyhow!("Unable to process drawing"))?
        .replace(' ', "")
        .len();

    let mut map: Map = Vec::with_capacity(columns);
    for _ in 0..columns {
        map.push(Vec::new());
    }

    while let Some(line) = drawing.pop() {
        for (index, container) in line[..]
            .as_bytes()
            .chunks(4)
            .map(|s| -> char { s[1].into() })
            .enumerate()
        {
            if !container.is_whitespace() {
                map[index].push(container);
            }
        }
    }

    let steps = x[1]
        .iter()
        .map(|step| {
            let split = step.split(' ').collect::<Vec<_>>();
            let quantity = split[1].parse()?;
            let source = split[3].parse()?;
            let destination = split[5].parse()?;

            Ok(Step {
                quantity,
                source,
                destination,
            })
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    Ok(InstructionSet { map, steps })
}

#[derive(Debug, Clone)]
struct InstructionSet {
    map: Map,
    steps: Vec<Step>,
}

#[derive(Debug, Clone)]
struct Step {
    quantity: u32,
    source: usize,
    destination: usize,
}
#[cfg(test)]
mod tests {
    use crate::{crate_mover_9000, crate_mover_9001, process_data};

    fn test_data() -> Vec<String> {
        r##"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"##
        .lines()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn crate_mover_9000_is_correct() {
        let data = process_data(test_data()).unwrap();
        let result = crate_mover_9000(data);

        assert_eq!("CMZ".to_string(), result.unwrap());
    }

    #[test]
    fn crate_mover_9001_is_correct() {
        let data = process_data(test_data()).unwrap();
        let result = crate_mover_9001(data);

        assert_eq!("MCD".to_string(), result.unwrap());
    }

    #[test]
    fn process_data_is_correct() {
        let data = process_data(test_data()).unwrap();

        assert!(!data.steps.is_empty() && !data.map.is_empty())
    }
}
