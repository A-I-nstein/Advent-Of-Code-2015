use std::{fs, io};

use regex::Regex;

#[derive(Debug)]
struct LighCommands<'a> {
    command: &'a str,
    start_coord: (u32, u32),
    end_coord: (u32, u32),
}

fn read_input(input_file: &str) -> Result<String, io::Error> {
    return fs::read_to_string(input_file);
}

fn get_command(line: &str) -> LighCommands {
    let re = Regex::new(r"(\w+)\s+(\d+,\d+)\s+through\s+(\d+,\d+)").unwrap();
    match re.captures(line) {
        Some(captures) => {
            let command: &str = captures.get(1).unwrap().into();
            let coord_1: &str = captures.get(2).unwrap().into();
            let mut coord_1 = coord_1.split(",");
            let coord_2: &str = captures.get(3).unwrap().into();
            let mut coord_2 = coord_2.split(",");

            LighCommands {
                command,
                start_coord: (
                    coord_1.next().expect("Error").parse().unwrap(),
                    coord_1.next().expect("Error").parse().unwrap(),
                ),
                end_coord: (
                    coord_2.next().expect("Error").parse().unwrap(),
                    coord_2.next().expect("Error").parse().unwrap(),
                ),
            }
        }
        None => {
            println!("No Match");
            LighCommands {
                command: "on",
                start_coord: (0, 0),
                end_coord: (0, 0),
            }
        }
    }
}

fn parse_input(puzzle_input: &String) -> Vec<LighCommands> {
    let mut puzzle_input_commands: Vec<LighCommands> = Vec::new();
    for line in puzzle_input.lines() {
        puzzle_input_commands.push(get_command(line));
    }
    return puzzle_input_commands;
}

fn main() -> Result<(), String> {
    let puzzle_input =
        read_input("input.txt").map_err(|e| format!("Could not read input file: {}", e))?;
    let puzzle_input_commands: Vec<LighCommands> = parse_input(&puzzle_input);
    println!("{:?}", puzzle_input_commands);
    Ok(())
}
