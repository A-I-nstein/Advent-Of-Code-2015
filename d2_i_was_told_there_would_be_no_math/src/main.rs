use std::{fs, io, num::ParseIntError};

struct Box {
    l: usize,
    w: usize,
    h: usize,
}

impl Box {
    fn new(l: &str, w: &str, h: &str) -> Result<Self, ParseIntError> {
        let l = l.parse()?;
        let w = w.parse()?;
        let h = h.parse()?;
        Ok(Box { l, w, h })
    }

    fn calculate_wrapper_size(&self) -> usize {
        let smallest_side: usize = (self.l * self.w).min(self.w * self.h).min(self.h * self.l);
        let surface_area: usize = 2 * ((self.l * self.w) + (self.w * self.h) + (self.h * self.l));
        return smallest_side + surface_area;
    }

    fn calculate_ribbon_size(&self) -> usize {
        let smallest_side: usize = (self.l + self.w).min(self.w + self.h).min(self.h + self.l);
        let volume: usize = self.l * self.w * self.h;
        return 2 * smallest_side + volume;
    }
}

fn read_input(input_file: &str) -> Result<String, io::Error> {
    return fs::read_to_string(input_file);
}

fn parse_input(puzzle_input: &String) -> Result<Vec<Box>, ParseIntError> {
    let mut puzzle_input_boxes: Vec<Box> = Vec::new();
    for line in puzzle_input.lines() {
        let dimensions: Vec<&str> = line.split('x').collect();
        puzzle_input_boxes.push(Box::new(dimensions[0], dimensions[1], dimensions[2])?);
    }
    return Ok(puzzle_input_boxes);
}

fn main() -> Result<(), String> {
    let puzzle_input: String =
        read_input("input.txt").map_err(|e| format!("Could not read input file: {}", e))?;
    let puzzle_input_boxes: Vec<Box> =
        parse_input(&puzzle_input).map_err(|e| format!("Could not parse dimensions: {}", e))?;
    let total_wrapper_size: usize = puzzle_input_boxes
        .iter()
        .map(|present_box| present_box.calculate_wrapper_size())
        .sum();
    let total_ribbon_length: usize = puzzle_input_boxes
        .iter()
        .map(|present_box| present_box.calculate_ribbon_size())
        .sum();
    println!("Total wrapping paper area: {} sqft", total_wrapper_size);
    println!("Total ribbon length: {} ft", total_ribbon_length);

    Ok(())
}
