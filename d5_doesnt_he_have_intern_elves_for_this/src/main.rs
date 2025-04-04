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
    chars.windows(2).any(|window| window[0] == window[1])
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

fn check_condn_2char_repeat(string: &str) -> bool {
    let chars: Vec<char> = string.chars().collect();
    chars.windows(2).enumerate().any(|(i, window1)| {
        chars
            .windows(2)
            .skip(i + 2)
            .any(|window2| window1 == window2)
    })
}

fn check_condn_xyx(string: &str) -> bool {
    let chars: Vec<char> = string.chars().collect();
    chars.windows(3).any(|window| window[0] == window[2])
}

fn main() -> Result<(), String> {
    let puzzle_input: String =
        read_input("input.txt").map_err(|e| format!("Could not read input file: {}", e))?;
    let puzzle_input_strings = parse_input(&puzzle_input);
    let mut nice_string_count_puzzle_1: u32 = 0;
    let mut nice_string_count_puzzle_2: u32 = 0;

    for string in puzzle_input_strings {
        if check_condn_vowel(string)
            && check_condn_char_repeat(string)
            && !check_condn_invalid_str(string)
        {
            nice_string_count_puzzle_1 += 1;
        }

        if check_condn_2char_repeat(string) && check_condn_xyx(string) {
            nice_string_count_puzzle_2 += 1;
        }
    }

    println!(
        "\nPuzzle 1: {} strings are nice.",
        nice_string_count_puzzle_1
    );
    println!(
        "\nPuzzle 2: {} strings are nice.",
        nice_string_count_puzzle_2
    );
    Ok(())
}
