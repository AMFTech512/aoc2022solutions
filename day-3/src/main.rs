use std::fs;

const INPUT_FILE_PATH: &str =
    "/home/austin/Documents/coding-competitions/advent-of-code/2022/day-3/input.txt";

fn read_line(chars: &mut dyn Iterator<Item = char>) -> Option<String> {
    let mut line = String::new();
    let mut current_char: char;
    
    match chars.next() {
        Some(c) => current_char = c,
        None => return None
    };

    while current_char != '\n' {
        line.push(current_char);
        match chars.next() {
            Some(c) => current_char = c,
            None => return Some(line)
        };
    }

    Some(line)
}

fn get_priority(c: char) -> i32 {
    if c.is_ascii_uppercase() {
        (c as u8 - 0x40 + 26) as i32
    } else {
        (c as u8 - 0x60) as i32
    }
}

fn split_str(string: &String) -> (String, String) {
    let mut str1 = String::new();
    let mut str2 = String::new();
    let string_len = string.len();

    for (idx, c) in string.chars().enumerate() {
        if idx < string_len / 2 {
            str1.push(c);
        } else {
            str2.push(c);
        }
    }

    (str1, str2)
}

fn find_dup (str1: &String, str2: &String) -> Option<char> {
    for c in str1.chars() {
        if let Some(_) = str2.find(c) {
            return Some(c);
        }
    }

    None
}

fn find_dup_3 (str1: &String, str2: &String, str3: &String) -> Option<char> {
    for c in str1.chars() {
        if let Some(_) = str2.find(c) {
            if let Some(_) = str3.find(c) {
                return Some(c);
            }
        }
    }

    None
}

fn part_1 (input: &String) {
    let contents_iter = &mut input.chars();
    let mut current_line = read_line(contents_iter);
    let mut sum = 0;
    while let Some(line) = current_line {
        let (part1, part2) = split_str(&line);
        let dup = find_dup(&part1, &part2).expect("String parts didn't have a duplicate :(");
        sum += get_priority(dup);

        current_line = read_line(contents_iter);
    }

    println!("Result part 1: {}", sum);
}

fn part_2 (input: &String) {
    let input_iter = &mut input.chars();
    let mut sum = 0;

    loop {
        let (
            line_1_res,
            line_2_res,
            line_3_res
        ) = (
            read_line(input_iter),
            read_line(input_iter),
            read_line(input_iter),
        );

        // stop when we reach the end of the input
        if let (None, None, None) = (&line_1_res, &line_2_res, &line_3_res) {
            break;
        } else if let (
            Some(line1),
            Some(line2),
            Some(line3)
        ) = (
            &line_1_res,
            &line_2_res,
            &line_3_res
        ) {
            

            let badge = find_dup_3(&line1, &line2, &line3).expect("Didn't find badge :(");
            sum += get_priority(badge);
        }

    }

    println!("Result part 2: {}", sum);
}

fn main() {
    
    let contents = fs::read_to_string(INPUT_FILE_PATH).expect("Should be able to read the file.");

    part_1(&contents);
    part_2(&contents);
}
