use std::{fs, io};

use regex::Regex;

#[derive(Debug)]
struct LighCommands<'a> {
    command: &'a str,
    start_coord: (usize, usize),
    end_coord: (usize, usize),
}

fn read_input(input_file: &str) -> Result<String, io::Error> {
    fs::read_to_string(input_file)
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

fn parse_input(puzzle_input: &str) -> Vec<LighCommands> {
    let mut puzzle_input_commands: Vec<LighCommands> = Vec::new();
    for line in puzzle_input.lines() {
        puzzle_input_commands.push(get_command(line));
    }
    puzzle_input_commands
}

fn apply_command(light_grid: &mut [Vec<usize>], command: &LighCommands) {
    for i in (command.start_coord.0)..(command.end_coord.0 + 1) {
        for j in (command.start_coord.1)..(command.end_coord.1 + 1) {
            if command.command == "off" {
                light_grid[i][j] = 0;
            } else if command.command == "on" {
                light_grid[i][j] = 1;
            } else if command.command == "toggle" {
                if light_grid[i][j] == 0 {
                    light_grid[i][j] = 1;
                } else if light_grid[i][j] == 1 {
                    light_grid[i][j] = 0;
                }
            }
        }
    }
}

fn apply_command_2(light_grid: &mut [Vec<usize>], command: &LighCommands) {
    for i in (command.start_coord.0)..(command.end_coord.0 + 1) {
        for j in (command.start_coord.1)..(command.end_coord.1 + 1) {
            if command.command == "off" {
                if light_grid[i][j] > 0 {
                    light_grid[i][j] -= 1;
                }
            } else if command.command == "on" {
                light_grid[i][j] += 1;
            } else if command.command == "toggle" {
                light_grid[i][j] += 2;
            }
        }
    }
}

fn main() -> Result<(), String> {
    let puzzle_input =
        read_input("input.txt").map_err(|e| format!("Could not read input file: {}", e))?;

    let puzzle_input_commands: Vec<LighCommands> = parse_input(&puzzle_input[0..]);

    let mut light_grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    for command in &puzzle_input_commands {
        apply_command(&mut light_grid, command);
    }

    let mut num_of_light_on: usize = 0;
    for row in &light_grid {
        for light_state in row {
            if *light_state == 1 {
                num_of_light_on += 1;
            }
        }
    }
    println!("Number of lights turned on: {}", num_of_light_on);

    let mut light_grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    for command in &puzzle_input_commands {
        apply_command_2(&mut light_grid, command);
    }

    let mut total_brightness: usize = 0;
    for row in &light_grid {
        for brightness in row {
            total_brightness += brightness;
        }
    }
    println!("Total Brightness: {}", total_brightness);

    Ok(())
}
