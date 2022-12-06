use std::fs;
use regex::Regex;

type StacksRow = Vec<Option<char>>;
type Stack = Vec<char>;
type Yard = Vec<Stack>;

// move # from # to #
type Instruction = (usize, usize, usize);

const INPUT_FILE_PATH: &str =
    "/home/austin/Documents/coding-competitions/advent-of-code/2022/day-5/input.txt";

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

fn parse_stack_row(line: &String) -> Option<StacksRow> {
    let regex = Regex::new(r"\[([A-Z])\]").unwrap();
    if !regex.is_match(&line) {
        return None;
    }

    let mut result: StacksRow = Vec::new();
    for find in regex.find_iter(line) {
        let (start_idx, end_idx) = (find.start(), find.end());
        let chr = line.as_str()[start_idx+1..end_idx-1].as_bytes()[0] as char;

        // fill any gaps with None
        let num_empty_slots = start_idx / 4 - result.len();
        for _ in 0..num_empty_slots {
            result.push(None);
        }

        result.push(Some(chr));
    }
    
    Some(result)
}

// this function assumes that rows has a length of at least 1
fn yard_from_rows(rows: &Vec<StacksRow>) -> Yard {
    // initialize yard
    let yard_size = rows[0].len();
    let mut result: Yard = Vec::with_capacity(yard_size);

    for _ in 0..yard_size {
        result.push(Vec::new());
    }

    // fill the yard
    for row in rows {
        for (stack_idx, o_crate) in row.iter().enumerate() {
            if let Some(chr) = o_crate {
                let stack = &mut result[stack_idx];
                stack.push(*chr);
            }
        }
    }

    result
}

fn read_stacks(chars: &mut dyn Iterator<Item = char>) -> Vec<StacksRow> {
    
    // read in the lines from the input as StacksRows
    let mut stacks_rows: Vec<StacksRow> = Vec::new();
    while let Some(line) = read_line(chars) {
        if let Some(stack_row) = parse_stack_row(&line) {
            stacks_rows.push(stack_row);
        } else {
            // read until we reach the bottow of the yard
            break;
        }
    }

    stacks_rows
}

fn parse_instruction(line: &String) -> Instruction {
    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let capture = regex.captures(line).expect("Expected valid instruction string.");
    (
        capture[1].parse().unwrap(),
        capture[2].parse().unwrap(),
        capture[3].parse().unwrap(),
    )
}

fn execute_instruction_9000(yard: &mut Yard, inx: Instruction) {
    let num_moves = inx.0;
    // move as many crates as it says to move
    for _ in 0..num_moves {
        let from_stack = &mut yard[inx.1 - 1];
        let chr = from_stack[0];
        from_stack.remove(0);

        let to_stack = &mut yard[inx.2 - 1];
        to_stack.insert(0, chr);
    }
}

fn execute_instruction_9001(yard: &mut Yard, inx: Instruction) {
    let num_moves = inx.0;
    // simulate moving multiple at a time by starting with the i'th crate
    for i in (0..num_moves).rev() {
        let from_stack = &mut yard[inx.1 - 1];
        let chr = from_stack[i];
        from_stack.remove(i);

        let to_stack = &mut yard[inx.2 - 1];
        to_stack.insert(0, chr);
    }
}

fn print_top(yard: &Yard) {
    for stack in yard {
        print!("{}", stack[0]);
    }
    println!();
}

fn main() {
    let contents = fs::read_to_string(INPUT_FILE_PATH).expect("Should be able to read the file.");
    let contents_iter = &mut contents.chars();

    let stacks_rows = read_stacks(contents_iter);
    let mut yard = yard_from_rows(&stacks_rows);

    // read the empty line between the yard and the instructions
    read_line(contents_iter);

    while let Some(line) = read_line(contents_iter) {
        let inx = parse_instruction(&line);
        execute_instruction_9001(&mut yard, inx);
    }

    print!("Result: ");
    print_top(&yard);

}
