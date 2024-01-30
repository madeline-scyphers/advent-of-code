/// Problem 1 of advent of code 2023
/// This problem is about parsing strings and finding numbers in them
/// Some numbers are written in words, some are written in digits
/// You must find the first and last digit of each string and concatenate them
/// Then sum all the numbers
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


fn main() {
    let numbers = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum = 0;
        for str in lines.flatten() {

            let mut first_digit = &0;
            let mut last_digit = &0;
            let mut first_digit_found = false;

            for (i, _c) in str.chars().enumerate() {

                for l in 1..6 {
                    if i + l <= str.len() {
                        if numbers.contains_key(&str[i..i + l]) {
                            let mtch = numbers.get(&str[i..i + l]).expect("Key Error");
                            if !first_digit_found {
                                first_digit = mtch;
                                first_digit_found = true;
                            }
                            last_digit = mtch;
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
