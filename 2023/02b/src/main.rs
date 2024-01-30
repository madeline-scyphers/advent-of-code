/// Problem 2b
/// This problem is about parsing strings and finding numbers in them
/// We are given a list of games, each game has a number and a list of pulls
/// Each pull has a number of balls of a certain color
/// We need to find the minimum number of balls of each color to make the game possible
/// Then multiply each color together per game and sum them

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



fn main() {
    if let Ok(lines) = read_lines("./data.txt") {
        let mut sum: u32 = 0;
        for str in lines.flatten() {
            let game = str.split(":").collect::<Vec<&str>>()[1].split(";").collect::<Vec<&str>>();
            let mut min_red = 0;
            let mut min_blue = 0;
            let mut min_green = 0;
            for pull in &game {
                let pull = pull.split(",").map(str::trim).collect::<Vec<&str>>();
                for colors in pull {
                    let colors = colors.split(" ").collect::<Vec<&str>>();
                    let balls = colors[0].parse::<u32>().expect("Not a valid number");
                    let color = colors[1].to_string();
                    if color == "red" && balls > min_red {
                        min_red = balls;
                    }
                    else if color == "blue" && balls > min_blue {
                        min_blue = balls;
                    }
                    else if color == "green" && balls > min_green {
                        min_green = balls;
                    }

                }
            }
            sum += min_red * min_blue * min_green;
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

