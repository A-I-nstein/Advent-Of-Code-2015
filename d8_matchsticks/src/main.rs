use std::{fs, io};

fn read_input(input_file: &str) -> Result<String, io::Error> {
    fs::read_to_string(input_file)
}

fn parse_input(santa_list: String) -> Vec<String> {
    let santa_list_items: Vec<String> = santa_list.lines().map(|s| s.to_string()).collect();
    return santa_list_items;
}

fn calculate_string_chars(santa_list_items: &Vec<String>) -> usize {
    let mut total_string_chars: usize = 0;
    for item in santa_list_items {
        total_string_chars += item.len();
        // println!("{}", item.len());
    }
    return total_string_chars;
}

fn calculate_memory_chars(santa_list_items: &Vec<String>) -> usize {
    let total_memory_chars: usize = 0;
    for _item in santa_list_items {
        // TODO: Calculate memory characters count
    }
    return total_memory_chars;
}

fn main() -> Result<(), String> {
    let santa_list: String =
        read_input("input.txt").map_err(|e| format!("Could not read input file: {}", e))?;
    let santa_list_items: Vec<String> = parse_input(santa_list);
    // println!("{} {:?}", santa_list_items.len(), santa_list_items);

    let total_string_chars: usize = calculate_string_chars(&santa_list_items);
    let total_memory_chars: usize = calculate_memory_chars(&santa_list_items);

    println!("{}", total_string_chars - total_memory_chars);

    Ok(())
}
