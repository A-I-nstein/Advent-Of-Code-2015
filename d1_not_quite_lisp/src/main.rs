use std::fs;

fn read_input(puzzle_input: &mut String, input_file: &str) {
    puzzle_input.clear();
    puzzle_input.push_str(&fs::read_to_string(input_file).expect("Could not read file."));
}

fn parse_input(puzzle_input: &String) -> Vec<char> {
    return puzzle_input.chars().collect();
}

fn main() {
    // input reading and parsing
    let input_file: &str = "input.txt";
    let mut puzzle_input: String = String::new();
    read_input(&mut puzzle_input, input_file);
    let puzzle_input: Vec<char> = parse_input(&puzzle_input);

    // puzzle variables
    let mut floor: isize = 0_isize;
    let mut basement_instruction_position: usize = 0_usize;

    // puzzle helper variables
    let mut crossed_basement_already: bool = false;

    // solving puzzle
    for (ind, inp_char) in puzzle_input.iter().enumerate() {
        if inp_char == &'(' {
            floor += 1;
        } else if inp_char == &')' {
            floor -= 1;
        }
        if (floor == -1) && (crossed_basement_already) == false {
            crossed_basement_already = true;
            basement_instruction_position = ind + 1;
        }
    }

    // puzzle solution
    println!("Puzzle 1 Solution: {}", floor);
    println!("Puzzle 2 Solution: {}", basement_instruction_position);
}
