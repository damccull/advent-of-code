#![allow(dead_code)]
#![allow(unused)]
use std::str::FromStr;

use advent_of_code_common::read_data_from_file;
use anyhow::Context;

fn main() -> Result<(), anyhow::Error> {
    let data = process_data(read_data_from_file("data/day10.txt")?)?;
    let result = run_cpu(data);
    println!("The total signal strength is {}", result.sample_total);

    let screen = result.screen.chunks(40);
    for line in screen {
        let x = line.iter().collect::<String>();
        println!("{x}");
    }
    Ok(())
}

fn run_cpu(data: Vec<Instruction>) -> CpuResult {
    let mut screen = vec![' '; 240];
    let mut total_cycles = 0i32;
    let mut instruction_cycles = 0i32;
    let mut register = 1i32;
    let mut running_totals = Vec::<i32>::new();

    let mut instructions = data.into_iter();
    let mut instruction: Option<Instruction> = None;
    loop {
        //start of cycle
        total_cycles += 1;

        if instruction_cycles == 0 {
            // Get instruction
            instruction = instructions.next();

            let Some(current_instruction) = instruction else {
                break;
            };
            instruction_cycles = current_instruction.cycles();
        }

        if total_cycles < 221 && total_cycles == 20 || (total_cycles + 20) % 40 == 0 {
            running_totals.push(total_cycles * register);
        }

        let Some(current_instruction) = instruction else {
            break;
        };
        draw_to_matrix(&mut screen, (total_cycles - 1) as usize, register as isize);

        instruction_cycles -= 1;
        if (instruction_cycles == 0) {
            match current_instruction {
                Instruction::Noop => {}
                Instruction::Addx(x) => register += x,
            }
        }
    }

    CpuResult {
        total_cycles,
        register,
        sample_total: running_totals.iter().sum(),
        screen,
    }
}

fn draw_to_matrix(screen: &mut [char], position: usize, register: isize) {
    let draw = (position % 40) as isize == register
        || (position % 40) as isize - 1 == register
        || (position % 40) as isize + 1 == register;
    if draw {
        screen[position] = '#';
    }
}

fn process_data(data: Vec<String>) -> Result<Vec<Instruction>, anyhow::Error> {
    data.iter()
        .map(Instruction::try_from)
        .collect::<Result<Vec<_>, anyhow::Error>>()
}

#[derive(Debug)]
struct CpuResult {
    total_cycles: i32,
    register: i32,
    sample_total: i32,
    screen: Vec<char>,
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    pub fn cycles(&self) -> i32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

impl TryFrom<&String> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.as_str() == "noop" {
            return Ok(Instruction::Noop);
        }
        if let Some((_, amount)) = value.split_once(' ') {
            return Ok(Instruction::Addx(amount.parse().context(
                "Unable to convert instruction, could not parse number",
            )?));
        }
        anyhow::bail!("Unable to convert instruction");
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code_common::read_data_from_file;

    use crate::{process_data, run_cpu};

    #[test]
    fn process_data_works() {
        let data = process_data(read_data_from_file("data/day10-test.txt").unwrap()).unwrap();

        assert!(!data.is_empty());
        assert_eq!(data.len(), 146);
    }

    #[test]
    fn run_cpu_works() {
        let data = process_data(read_data_from_file("data/day10-test.txt").unwrap()).unwrap();

        let result = run_cpu(data);

        assert_eq!(result.sample_total, 13140);
    }

    #[test]
    fn draw_to_matrix_works() {
        let data = process_data(read_data_from_file("2022/data/day10-test.txt").unwrap()).unwrap();

        let result = run_cpu(data);

        let screen = result.screen.chunks(40);
        for line in screen {
            let x = line.iter().collect::<String>();
            println!("{x}");
        }

        assert_eq!(result.sample_total, 13140);
    }
}
