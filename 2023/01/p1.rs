/// Problem 1 of advent of code 2023
/// This problem is about parsing strings and finding numbers in them
/// Some numbers are written in words, some are written in digits
/// You must find the first and last digit of each string and concatenate them
/// Then sum all the numbers
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const NUM_WORDS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const DIGITS: [i8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];


fn main() {
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum = 0;
        for str in lines.flatten() {

            let mut first_digit = char::default();
            let mut last_digit = char::default();
            let mut first_digit_found = false;

            // let mut last_digit_found = false;
            for (i, c) in str.chars().enumerate() {
                if c.is_digit(10) {
                    if !first_digit_found {
                        first_digit = c;
                        first_digit_found = true;
                    }
                    last_digit = c;
                }
                else {
                    for num in NUM_WORDS.iter() {
                        let l = num.len();
                        if i + l <= str.len() {
                            let s = parse_substring(&str[i..i + l]);
                            if s != -1 {
                                if !first_digit_found {
                                    first_digit = s.to_string().chars().next().unwrap();
                                    first_digit_found = true;
                                }
                                last_digit = s.to_string().chars().next().unwrap();
                                break
                            }
                        }
                    }
                }
            }
            let both_digits = format!("{}{}", first_digit, last_digit).parse::<u32>().expect("Not a valid number");
            sum += both_digits;

        }
        println!("{}", sum);
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// parse substring to digit, return -1 if not found
fn parse_substring(substring: &str) -> i8 {
    if NUM_WORDS.contains(&substring) {
        return DIGITS[NUM_WORDS.iter().position(|&r| r == substring).unwrap()];
    }
    return -1;
}
