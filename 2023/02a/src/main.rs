/// Problem 2a
/// This problem is about parsing strings and finding numbers in them
/// We are given a list of games, each game has a number and a list of pulls
/// Each pull has a number of balls of a certain color
/// We have a limited number of balls of each color
/// We must find the number of games that are possible

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;


fn main() {
    // assert that the test data is correct
    assert_eq!(process_data("./test_data.txt"), 8);
    println!("{}", process_data("./data.txt"));
}

fn process_data(file_name: &str) -> u32 {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines(file_name) {
        for str in lines.flatten() {
            // split the string into the game number and the pulls
            // string example: `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green`
            let s = str.split(":").collect::<Vec<&str>>();
            let game_num = s[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().expect("Not a valid number");
            // split the game into individual pulls: `3 blue, 4 red` and `1 red, 2 green, 6 blue` and `2 green`
            let game = s[1].split(";").collect::<Vec<&str>>();
            sum += game_num;  // Assume game is possible
            'game_loop: for pull in &game {
                // split each game into individual colors: `3 blue` and `4 red`
                let colors = pull.split(",").map(str::trim).collect::<Vec<&str>>();
                for color in colors {
                    let color = color.split(" ").collect::<Vec<&str>>();
                    let balls = color[0].parse::<u32>().expect("Not a valid number");
                    let color = color[1].to_string();
                    if (color == "red" && balls > RED)
                        || (color == "blue" && balls > BLUE)
                        || (color == "green" && balls > GREEN) {
                        sum -= game_num;  // if game is not possible, subtract from sum
                        break 'game_loop
                    }
                }
            }
        }
    }
    return sum;
}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

