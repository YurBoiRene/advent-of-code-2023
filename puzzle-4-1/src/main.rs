use clap::Parser;
use std::{path::PathBuf, str::FromStr};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    input: PathBuf,
}

fn main() {
    let input_filename = Cli::parse().input;
    let result = run_puzzle_file(&input_filename);

    println!("Final: {result}");
}

fn run_puzzle_file(filename: &PathBuf) -> u32 {
    run_puzzle(&std::fs::read_to_string(filename).unwrap())
}

fn run_puzzle(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Game::from_str(line).unwrap().score())
        .sum()
}

struct Game {
    _id: u32,
    winning: Vec<u8>,
    picks: Vec<u8>,
}

impl Game {
    fn score(&self) -> u32 {
        self.picks
            .iter()
            .filter(|pick| self.is_winning(**pick))
            .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
    }
    fn is_winning(&self, pick: u8) -> bool {
        self.winning.contains(&pick)
    }
}

#[derive(Debug)]
struct ParseStrError;
impl FromStr for Game {
    type Err = ParseStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card, numbers) = s.split_once(':').unwrap();
        let id = card.split_whitespace().last().unwrap().parse().unwrap();

        let (winning_str, picks_str) = numbers.split_once('|').unwrap();
        let winning = winning_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let picks = picks_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Game {
            _id: id,
            winning,
            picks,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_result_test() {
        let result = run_puzzle_file(&PathBuf::from("tests/input"));
        assert_eq!(result, 18653);
    }
}
