use clap::Parser;
use std::{collections::HashMap, path::PathBuf, str::FromStr};

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
    GamesList::from_str(input).unwrap().score()
}

#[derive(Debug)]
struct Game {
    id: usize,
    winning: Vec<u8>,
    picks: Vec<u8>,
}

impl Game {
    fn score(&self, totals: &mut HashMap<usize, u32>) {
        let points = self
            .picks
            .iter()
            .filter(|pick| self.is_winning(**pick))
            .count();

        for idx in 1..=points {
            let mut value = *totals.get(&(idx + self.id)).unwrap();
            value += totals[&self.id];
            totals.insert(idx + self.id, value);
        }
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

        Ok(Game { id, winning, picks })
    }
}

struct GamesList {
    games: Vec<Game>,
}

impl GamesList {
    fn score(&self) -> u32 {
        let mut sums = HashMap::from_iter((1..=self.games.len()).map(|idx| (idx, 1)));
        for game in self.games.iter() {
            println!("Current head {}", game.id);

            game.score(&mut sums)
        }
        sums.into_values().sum()
    }
}

impl FromStr for GamesList {
    type Err = ParseStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GamesList {
            games: s
                .lines()
                .map(|line| Game::from_str(line).unwrap())
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_result_test() {
        let result = run_puzzle_file(&PathBuf::from("tests/input"));
        assert_eq!(result, 5921508);
    }
}
