use clap::Parser;
use std::path::PathBuf;

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

fn run_puzzle_file(filename: &PathBuf) -> usize {
    run_puzzle(&std::fs::read_to_string(filename).unwrap())
}

fn run_puzzle(input: &str) -> usize {
    let races = parse_races(input);
    races.iter().map(|race| race.get_wins().count()).product()
}

fn parse_races(input: &str) -> Vec<Race> {
    let (time_line, distance_line) = input.split_once('\n').unwrap();
    let times = time_line
        .split_whitespace()
        .skip(1)
        .map(|ts| ts.parse().unwrap());
    let distances = distance_line
        .split_whitespace()
        .skip(1)
        .map(|ts| ts.parse().unwrap());

    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn distance(&self, time_held: u64) -> u64 {
        time_held * (self.time - time_held)
    }
    fn is_win(&self, time_held: u64) -> bool {
        self.distance(time_held) > self.distance
    }

    fn get_wins(&self) -> std::ops::RangeInclusive<u64> {
        dbg!(self);
        let possible_times = 0..=self.time;
        let min_time = possible_times
            .clone()
            .take_while(|time| !self.is_win(*time))
            .count() as u64;
        dbg!(min_time);

        // Winning time is symmetric
        let max_time = self.time - min_time;

        dbg!(max_time);
        min_time..=max_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_result_test() {
        let result = run_puzzle_file(&PathBuf::from("tests/input"));
        assert_eq!(result, 35961505);
    }
}
