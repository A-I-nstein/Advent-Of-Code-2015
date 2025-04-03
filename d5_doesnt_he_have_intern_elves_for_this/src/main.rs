use regex::Regex;
use std::{fs, io};

fn read_input(input_file: &str) -> Result<String, io::Error> {
    return fs::read_to_string(input_file);
}

fn parse_input(puzzle_input: &String) -> Vec<&str> {
    let mut puzzle_input_strings: Vec<&str> = Vec::new();
    for line in puzzle_input.lines() {
        puzzle_input_strings.push(line);
    }
    return puzzle_input_strings;
}

fn check_condn_vowel(string: &str) -> bool {
    let regex_vowel = Regex::new(r"[a|e|i|o|u]").unwrap();
    let count = regex_vowel.find_iter(string).count();
    if count >= 3 {
        return true;
    } else {
        return false;
    }
}

fn check_condn_char_repeat(string: &str) -> bool {
    let chars: Vec<char> = string.chars().collect();
    for window in chars.windows(2) {
        if window[0] == window[1] {
            return true;
        }
    }
    false
}
fn check_condn_invalid_str(string: &str) -> bool {
    let regex_invalid_str = Regex::new(r"ab|cd|pq|xy").unwrap();
    let count = regex_invalid_str.find_iter(string).count();
    if count >= 1 {
        return true;
    } else {
        return false;
    }
}

fn main() -> Result<(), String> {
    let puzzle_input: String =
        read_input("input.txt").map_err(|e| format!("Could not read input file: {}", e))?;
    let puzzle_input_strings = parse_input(&puzzle_input);
    let mut nice_string_count: u32 = 0;

    for string in puzzle_input_strings {
        let condn_vowel = check_condn_vowel(string);
        let condn_char_repeat = check_condn_char_repeat(string);
        let condn_invalid_str = check_condn_invalid_str(string);

        if condn_vowel && condn_char_repeat && !condn_invalid_str {
            nice_string_count += 1;
        }
    }

    println!("\nPuzzle 1: {} strings are nice.", nice_string_count);
    Ok(())
}
