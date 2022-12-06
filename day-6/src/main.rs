use std::fs;
use std::collections::HashSet;

const INPUT_FILE_PATH: &str =
    "/home/austin/Documents/coding-competitions/advent-of-code/2022/day-6/input.txt";

const MARKER_SIZE: usize = 14;

fn chars_are_unique(string: &str) -> bool {
    let mut set: HashSet<char> = HashSet::new();

    for c in string.as_bytes() {
        set.insert(*c as char);
    }
    
    set.len() == string.len()
}

fn main() {
    let contents = fs::read_to_string(INPUT_FILE_PATH).expect("Should be able to read the file.");

    for i in 0..contents.len() - MARKER_SIZE {
        let buff = &contents[i..i+MARKER_SIZE];

        if chars_are_unique(buff) {
            println!("Result: {}", i + MARKER_SIZE);
            break;
        }
    }
}
