use std::{fs, io};

// Reads the input from the specified file into a String.
fn read_input(input_file: &str) -> Result<String, io::Error> {
    return  fs::read_to_string(input_file);
}

// Parses the input string into a vector of characters.
fn parse_input(puzzle_input: &String) -> Vec<char> {
    return puzzle_input.chars().collect();
}

fn main() -> Result<(), io::Error> {
    // input reading and parsing
    let input_file: &str = "input.txt";
    let puzzle_input = read_input(input_file)?;
    let puzzle_input_chars: Vec<char> = parse_input(&puzzle_input);

    // puzzle variables
    let mut floor: isize = 0;
    let mut basement_instruction_position: Option<usize> = None;

    // solving puzzle
    for (index, &input_char) in puzzle_input_chars.iter().enumerate() {
        match input_char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {
                eprintln!("Warning: Unexpected character '{}' in input.", input_char);
            }
        }
        if floor == -1 && basement_instruction_position.is_none() {
            basement_instruction_position = Some(index + 1);
        }
    }

    // puzzle solution
    println!("Puzzle 1 Solution: {}", floor);
    match basement_instruction_position {
        Some(pos) => println!("Puzzle 2 Solution: {}", pos),
        None => println!("Puzzle 2 Solution: Basement was never reached."),
    }

    Ok(())
}
