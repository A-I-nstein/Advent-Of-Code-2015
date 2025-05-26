use std::{fs, io, str::FromStr};

use regex::Regex;

#[derive(Debug)]
struct Instruction {
    operator: String,
    operands: Vec<String>,
    result: String,
}

fn read_input(input_file: &str) -> Result<String, io::Error> {
    fs::read_to_string(input_file)
}

fn get_and_instruction(line: &str) -> Option<Instruction> {
    let re = Regex::new(r"(\w+)\s+AND\s+(\w+)\s+->+\s+(\w+)").unwrap();
    match re.captures(line) {
        Some(captures) => {
            let operator: String = String::from_str("AND").unwrap();
            let operands: Vec<String> = vec![
                captures.get(1).unwrap().as_str().to_string(),
                captures.get(2).unwrap().as_str().to_string(),
            ];
            let result: String = captures.get(3).unwrap().as_str().to_string();

            Some(Instruction {
                operator,
                operands,
                result,
            })
        }
        None => None,
    }
}

fn get_or_instruction(line: &str) -> Option<Instruction> {
    let re = Regex::new(r"(\w+)\s+OR\s+(\w+)\s+->+\s+(\w+)").unwrap();
    match re.captures(line) {
        Some(captures) => {
            let operator: String = String::from_str("OR").unwrap();
            let operands: Vec<String> = vec![
                captures.get(1).unwrap().as_str().to_string(),
                captures.get(2).unwrap().as_str().to_string(),
            ];
            let result: String = captures.get(3).unwrap().as_str().to_string();

            Some(Instruction {
                operator,
                operands,
                result,
            })
        }
        None => None,
    }
}

fn get_lshift_instruction(line: &str) -> Option<Instruction> {
    let re = Regex::new(r"(\w+)\s+LSHIFT\s+(\w+)\s+->+\s+(\w+)").unwrap();
    match re.captures(line) {
        Some(captures) => {
            let operator: String = String::from_str("LSHIFT").unwrap();
            let operands: Vec<String> = vec![
                captures.get(1).unwrap().as_str().to_string(),
                captures.get(2).unwrap().as_str().to_string(),
            ];
            let result: String = captures.get(3).unwrap().as_str().to_string();

            Some(Instruction {
                operator,
                operands,
                result,
            })
        }
        None => None,
    }
}

fn get_rshift_instruction(line: &str) -> Option<Instruction> {
    let re = Regex::new(r"(\w+)\s+RSHIFT\s+(\w+)\s+->+\s+(\w+)").unwrap();
    match re.captures(line) {
        Some(captures) => {
            let operator: String = String::from_str("RSHIFT").unwrap();
            let operands: Vec<String> = vec![
                captures.get(1).unwrap().as_str().to_string(),
                captures.get(2).unwrap().as_str().to_string(),
            ];
            let result: String = captures.get(3).unwrap().as_str().to_string();

            Some(Instruction {
                operator,
                operands,
                result,
            })
        }
        None => None,
    }
}

fn get_not_instruction(line: &str) -> Option<Instruction> {
    let re = Regex::new(r"NOT\s+(\w+)\s+->+\s+(\w+)").unwrap();
    match re.captures(line) {
        Some(captures) => {
            let operator: String = String::from_str("NOT").unwrap();
            let operands: Vec<String> = vec![captures.get(1).unwrap().as_str().to_string()];
            let result: String = captures.get(2).unwrap().as_str().to_string();

            Some(Instruction {
                operator,
                operands,
                result,
            })
        }
        None => None,
    }
}

fn get_value_instruction(line: &str) -> Option<Instruction> {
    let re = Regex::new(r"(\w+)\s+->+\s+(\w+)").unwrap();
    match re.captures(line) {
        Some(captures) => {
            let operator: String = String::from_str("VALUE").unwrap();
            let operands: Vec<String> = vec![captures.get(1).unwrap().as_str().to_string()];
            let result: String = captures.get(2).unwrap().as_str().to_string();

            Some(Instruction {
                operator,
                operands,
                result,
            })
        }
        None => None,
    }
}

fn get_command(line: &str) -> Option<Instruction> {
    if let Some(instruction) = get_and_instruction(line) {
        Some(instruction)
    } else if let Some(instruction) = get_or_instruction(line) {
        Some(instruction)
    } else if let Some(instruction) = get_lshift_instruction(line) {
        Some(instruction)
    } else if let Some(instruction) = get_rshift_instruction(line) {
        Some(instruction)
    } else if let Some(instruction) = get_not_instruction(line) {
        Some(instruction)
    } else {
        get_value_instruction(line)
    }
}

fn parse_input(puzzle_input: &str) -> Vec<Instruction> {
    let mut puzzle_input_instructions: Vec<Instruction> = Vec::new();
    for line in puzzle_input.lines() {
        puzzle_input_instructions.push(get_command(line).unwrap());
    }
    puzzle_input_instructions
}

fn main() -> Result<(), String> {
    let puzzle_input =
        read_input("input.txt").map_err(|e| format!("Could not read input file: {}", e))?;

    let puzzle_input_instructions: Vec<Instruction> = parse_input(&puzzle_input[0..]);
    println!("{:?}", puzzle_input_instructions);

    Ok(())
}
