use clap::Parser;
use regex::Regex;
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
        let line2 = line.unwrap();
        if let Ok(num) = parse_line(&line2) {
            sum += num;
        } else {
            println!("Error parsing {line2}");
        }
    }

    println!("Final: {sum}");
}

fn parse_line(line: &str) -> anyhow::Result<u32> {
    let re = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re2 = Regex::new(r"([1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
    let line2: String = line.to_owned().chars().rev().collect();
    let matches2: Vec<_> = re2.find_iter(&line2).map(|m| m.as_str()).collect();

    let a = parse_number(matches[0])?;
    let fuck: String = matches2[0].chars().rev().collect();
    let b = parse_number(&fuck)?;
    let rtn = (a * 10) + b;
    // println!("a: {a}, b: {b}, rtn: {rtn}");
    Ok(rtn)
}

fn parse_number(number: &str) -> anyhow::Result<u32> {
    if number.len() == 1 {
        Ok(number.parse()?)
    } else {
        match number {
            "one" => Ok(1),
            "two" => Ok(2),
            "three" => Ok(3),
            "four" => Ok(4),
            "five" => Ok(5),
            "six" => Ok(6),
            "seven" => Ok(7),
            "eight" => Ok(8),
            "nine" => Ok(9),
            _ => Err(ReneError)?,
        }
    }
}

#[derive(thiserror::Error, Debug, Clone)]
#[error("rene error")]
struct ReneError;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
