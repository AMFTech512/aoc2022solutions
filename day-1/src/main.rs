use std::fs;

const INPUT_FILE_PATH: &str =
    "/home/austin/Documents/coding-competitions/advent-of-code/2022/day-1/input.txt";

fn main() {
    let contents = fs::read_to_string(INPUT_FILE_PATH).expect("Should be able to read the file.");

    let mut current_line = String::new();
    let mut largest_sum = 0;
    let mut current_sum = 0;
    let mut was_last_newline = false;
    for c in contents.chars() {
        if c == '\n' {

            // check if this sum is larger than the largest sum
            if was_last_newline {
                if current_sum > largest_sum {
                    largest_sum = current_sum;
                }
                current_sum = 0;
            } else {
                // get the number and add it to the sum
                let num: i32 = current_line.parse().expect("This should've been a number :(");
                current_sum += num;
                current_line.clear();
            }

            was_last_newline = true;
            continue;
        } else {
            was_last_newline = false;
            current_line.push(c);
        }
    }

    println!("Result: {}", largest_sum);

}
