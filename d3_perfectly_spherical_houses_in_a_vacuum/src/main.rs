use std::{collections::HashMap, fs, io};

fn read_input(input_file: &str) -> Result<String, io::Error> {
    return fs::read_to_string(input_file);
}

fn parse_input(puzzle_input: &String) -> Vec<char> {
    return puzzle_input.chars().collect();
}

fn puzzle_1(puzzle_input_chars: &Vec<char>) {
    let mut current_location: (isize, isize) = (0, 0);
    let mut locations_visited: HashMap<(isize, isize), usize> = HashMap::new();

    for direction in puzzle_input_chars.iter() {
        let no_of_times_visited = locations_visited.entry(current_location).or_insert(1);
        *no_of_times_visited += 1;
        match direction {
            '^' => current_location.1 += 1,
            '>' => current_location.0 += 1,
            'v' => current_location.1 -= 1,
            '<' => current_location.0 -= 1,
            other => eprintln!("Warning: Unexpected character '{}' in input.", other),
        }
    }

    println!("Puzzle 1 Solution: {}", locations_visited.len());
}

fn puzzle_2(puzzle_input_chars: &Vec<char>) {
    let mut santa_current_location: (isize, isize) = (0, 0);
    let mut robo_current_location: (isize, isize) = (0, 0);
    let mut locations_visited: HashMap<(isize, isize), usize> = HashMap::new();

    let no_of_times_visited = locations_visited.entry(santa_current_location).or_insert(1);
    *no_of_times_visited += 1;
    let no_of_times_visited = locations_visited.entry(robo_current_location).or_insert(1);
    *no_of_times_visited += 1;

    for (index, direction) in puzzle_input_chars.iter().enumerate() {
        let current_location = if index % 2 == 0 {
            &mut santa_current_location
        } else {
            &mut robo_current_location
        };
        match direction {
            '^' => current_location.1 += 1,
            '>' => current_location.0 += 1,
            'v' => current_location.1 -= 1,
            '<' => current_location.0 -= 1,
            other => eprintln!("Warning: Unexpected character '{}' in input.", other),
        }
        let no_of_times_visited = locations_visited.entry(*current_location).or_insert(1);
        *no_of_times_visited += 1;
    }

    println!("Puzzle 2 Solution: {}", locations_visited.len());
}

fn main() -> Result<(), io::Error> {
    let input_file: &str = "input.txt";
    let puzzle_input = read_input(input_file)?;
    let puzzle_input_chars: Vec<char> = parse_input(&puzzle_input);

    puzzle_1(&puzzle_input_chars);
    puzzle_2(&puzzle_input_chars);

    Ok(())
}
