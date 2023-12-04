use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    input: PathBuf,
}

fn main() {
    let input_filename = Cli::parse().input;

    let mut games: Vec<Game> = Vec::new();
    for line in read_lines(input_filename).unwrap() {
        games.push(line.unwrap().parse().unwrap())
    }

    let sum: u32 = games
        .iter()
        .map(|game| {
            let max_red = game.pulls.iter().max_by_key(|pull| pull.red).unwrap().red;
            let max_blue = game.pulls.iter().max_by_key(|pull| pull.blue).unwrap().blue;
            let max_green = game
                .pulls
                .iter()
                .max_by_key(|pull| pull.green)
                .unwrap()
                .green;
            (max_red, max_blue, max_green)
        })
        .map(|maxes| maxes.0 * maxes.1 * maxes.2)
        .sum();

    println!("Final: {sum}");
}

type Pulls = u32;
#[derive(Default, Debug)]
struct Pull {
    red: Pulls,
    blue: Pulls,
    green: Pulls,
}

#[derive(Debug)]
struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

#[derive(Debug)]
struct ParsePullErr;

impl FromStr for Pull {
    type Err = ParsePullErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red: Pulls = 0;
        let mut blue: Pulls = 0;
        let mut green: Pulls = 0;
        let s = s.trim();

        for count_color in s.split(", ") {
            let mut a = count_color.split(' ');
            let count: Pulls = a.next().unwrap().parse().unwrap();
            let color = a.next().unwrap();
            match color {
                "red" => red = count,
                "blue" => blue = count,
                "green" => green = count,
                _ => return Err(ParsePullErr),
            }
        }

        Ok(Pull { red, blue, green })
    }
}

#[derive(Debug)]
struct ParseGameErr;

impl FromStr for Game {
    type Err = ParseGameErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut pulls = Vec::new();

        let mut a = s.split(':');
        let first_game = a.next().unwrap();
        let pulls_str = a.next().unwrap();

        let id = first_game.strip_prefix("Game ").unwrap().parse().unwrap();

        for pull_str in pulls_str.split(';') {
            pulls.push(pull_str.parse().unwrap());
        }

        Ok(Game { id, pulls })
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
