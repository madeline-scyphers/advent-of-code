/// Problem 2b
/// This problem is about parsing strings and finding numbers in them
/// We are given a list of games, each game has a number and a list of pulls
/// Each pull has a number of balls of a certain color
/// We need to find the minimum number of balls of each color to make the game possible
/// Then multiply each minimum color together per game and sum them

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



fn main() {
    assert_eq!(process_data("./test_data.txt"), 2286);
    println!("{}", process_data("./data.txt"));
}

fn process_data(filename: &str) -> u32 {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for str in lines.flatten() {
            // split the string the pulls
            // string example: `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green`
            let game = str.split(":").collect::<Vec<&str>>()[1].split(";").collect::<Vec<&str>>();
            let mut min_red = 0;
            let mut min_blue = 0;
            let mut min_green = 0;
            for pull in &game {
                // split each game into individual colors: `3 blue` and `4 red`
                let colors = pull.split(",").map(str::trim).collect::<Vec<&str>>();
                for color in colors {
                    // split each color into the number of balls and the color: `3` and `blue
                    let color = color.split(" ").collect::<Vec<&str>>();
                    let balls = color[0].parse::<u32>().expect("Not a valid number");
                    let color = color[1].to_string();
                    // if the number of balls is greater than the current minimum, inc the minimum
                    if color == "red" && balls > min_red {
                        min_red = balls;
                    } else if color == "blue" && balls > min_blue {
                        min_blue = balls;
                    } else if color == "green" && balls > min_green {
                        min_green = balls;
                    }
                }
            }
            // multiply the minimums together and add to the sum, b/c that is the rules of the problem
            sum += min_red * min_blue * min_green;
        }
    }
    return sum
}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

