use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    input: PathBuf,
}

fn main() {
    let input_filename = Cli::parse().input;
    let mut sum: u32 = 0;
    for line in read_lines(input_filename).unwrap() {
        sum += parse_line(&line.unwrap());
    }

    println!("Final: {sum}");
}

fn parse_line(line: &str) -> u32 {
    let digits: Vec<char> = line.chars().filter(|&c| c.is_ascii_digit()).collect();

    let mut full_number = digits[0].to_string();
    full_number.push(digits[digits.len() - 1]);
    full_number.parse().unwrap()
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
