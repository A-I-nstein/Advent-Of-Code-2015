use hex;
use md5::compute;

fn check_for_hash(candidate_string: &str, starting_string: &str) -> bool {
    let digest = compute(candidate_string.as_bytes());
    let hex_digest = hex::encode(&*digest);
    if hex_digest.starts_with(starting_string) {
        println!("{}: {:x}", candidate_string, digest);
        return true;
    }
    false
}

fn find_hash_with_prefix(puzzle_input: &str, starting_string: &str) -> Option<usize> {
    for i in 0..usize::MAX {
        let candidate_string = format!("{}{}", puzzle_input, i);
        if check_for_hash(&candidate_string, starting_string) {
            return Some(i);
        }
    }
    None
}

fn main() {
    let puzzle_input: &str = "bgvyzdsv";

    let starting_string_part1: &str = "00000";
    println!("Searching for hash starting with '{}'...", starting_string_part1);
    if let Some(result1) = find_hash_with_prefix(puzzle_input, starting_string_part1) {
        println!("Found the first number for '{}': {}", starting_string_part1, result1);
    } else {
        println!("Could not find a number for '{}'.", starting_string_part1);
    }

    println!();
    
    let starting_string_part2: &str = "000000";
    println!("Searching for hash starting with '{}'...", starting_string_part2);
    if let Some(result2) = find_hash_with_prefix(puzzle_input, starting_string_part2) {
        println!("Found the first number for '{}': {}", starting_string_part2, result2);
    } else {
        println!("Could not find a number for '{}'.", starting_string_part2);
    }
}