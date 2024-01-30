/// Problem 2.1
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
    if let Ok(lines) = read_lines("./data.txt") {
        let mut sum: u32 = 0;
        for str in lines.flatten() {
            let s = str.split(":").collect::<Vec<&str>>();
            let game_num = s[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().expect("Not a valid number");
            let game = s[1].split(";").collect::<Vec<&str>>();
            sum += game_num;  // Assume game is possible
            'game_loop: for pull in &game {
                let pull = pull.split(",").map(str::trim).collect::<Vec<&str>>();
                for color in pull {
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

