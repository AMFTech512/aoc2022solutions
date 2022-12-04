use regex::Regex;
use std::fs;

const INPUT_FILE_PATH: &str =
    "/home/austin/Documents/coding-competitions/advent-of-code/2022/day-4/input.txt";

type SectionBounds = (i32, i32);
type SectionPair = (SectionBounds, SectionBounds);

fn read_line(chars: &mut dyn Iterator<Item = char>) -> Option<String> {
    let mut line = String::new();
    let mut current_char: char;
    
    // return None if we've reached the end of the file
    match chars.next() {
        Some(c) => current_char = c,
        None => return None
    };

    while current_char != '\n' {
        line.push(current_char);
        match chars.next() {
            Some(c) => current_char = c,
            // return what we have for the current line if we reach the end of the file
            None => return Some(line)
        };
    }

    Some(line)
}

fn get_sections(line: &String) -> SectionPair {
    let regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let captures = regex.captures(line).expect("Couldn't process sections in the current line");
    let result = (
        (
            captures[1].parse().unwrap(),
            captures[2].parse().unwrap()
        ),
        (
            captures[3].parse().unwrap(),
            captures[4].parse().unwrap()
        )
    );

    result
}

// task 1
fn contains_other(section_pair: SectionPair) -> bool {
    (section_pair.0.0 <= section_pair.1.0 && section_pair.0.1 >= section_pair.1.1) ||
    (section_pair.1.0 <= section_pair.0.0 && section_pair.1.1 >= section_pair.0.1)
}

// task 2
fn does_overlap(section_pair: SectionPair) -> bool {
    (section_pair.0.0 <= section_pair.1.0 && section_pair.0.1 >= section_pair.1.0) ||
    (section_pair.0.0 <= section_pair.1.1 && section_pair.0.1 >= section_pair.1.1) ||
    (section_pair.1.0 <= section_pair.0.0 && section_pair.1.1 >= section_pair.0.0) ||
    (section_pair.1.0 <= section_pair.0.1 && section_pair.1.1 >= section_pair.0.1)
}

fn main() {
    let contents = fs::read_to_string(INPUT_FILE_PATH).expect("Should be able to read the file.");

    let line_iter = &mut contents.chars();

    let mut contains_count = 0;
    let mut overlap_count = 0;
    while let Some(line) = read_line(line_iter) {
        let section_pair = get_sections(&line);

        if contains_other(section_pair) {
            contains_count += 1;
        }

        if does_overlap(section_pair) {
            overlap_count += 1;
        }
        
    }

    println!("Contains count: {}", contains_count);
    println!("Overlap count: {}", overlap_count);

}
