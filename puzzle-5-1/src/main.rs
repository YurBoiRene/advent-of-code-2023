use clap::Parser;
use itertools::Itertools;
use std::{ops::Range, path::PathBuf, str::FromStr};

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

fn run_puzzle_file(filename: &PathBuf) -> i64 {
    run_puzzle(&std::fs::read_to_string(filename).unwrap())
}

fn run_puzzle(input: &str) -> i64 {
    let (seed_line, other_lines) = input.split_once('\n').unwrap();
    let seeds = get_seeds(seed_line);
    let almanac = Almanac::from_str(other_lines.trim()).unwrap();

    seeds
        .iter()
        .map(|s| almanac.map_to_location(*s))
        .min()
        .unwrap()
}

fn get_seeds(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .skip(1)
        .map(|ss| ss.parse().unwrap())
        .collect()
}

#[derive(Debug)]
struct MapEntry {
    source_range: Range<i64>,
    _dest_range: Range<i64>,
    difference: i64,
}

impl MapEntry {
    fn map(&self, source_value: i64) -> Option<i64> {
        if self.source_range.contains(&source_value) {
            Some(source_value + self.difference)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct ParseStrErr;
impl FromStr for MapEntry {
    type Err = ParseStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dest_start, source_start, length) = s
            .split_whitespace()
            .map(|ns| ns.parse().unwrap())
            .collect_tuple()
            .unwrap();

        let source_range = source_start..(source_start + length);
        let dest_range = dest_start..(dest_start + length);
        let difference = dest_start - source_start;

        Ok(MapEntry {
            source_range,
            _dest_range: dest_range,
            difference,
        })
    }
}

#[derive(Debug)]
struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn map(&self, source_value: i64) -> i64 {
        self.entries
            .iter()
            .map(|e| e.map(source_value))
            .filter(|v| v.is_some())
            .at_most_one() // There should 0-1 matching ranges
            .unwrap()
            .unwrap_or(Some(source_value)) // If no matching range, use source value
            .unwrap()
    }
}

impl FromStr for Map {
    type Err = ParseStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s
            .lines()
            .skip(1)
            .map(|line| line.parse().unwrap())
            .collect();
        Ok(Map { entries })
    }
}

struct Almanac {
    maps: Vec<Map>,
}

impl FromStr for Almanac {
    type Err = ParseStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let maps = s.split("\n\n").map(|s| s.parse().unwrap()).collect();
        Ok(Almanac { maps })
    }
}

impl Almanac {
    fn map_to_location(&self, seed: i64) -> i64 {
        self.maps.iter().fold(seed, |s, map| map.map(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_result_test() {
        let result = run_puzzle_file(&PathBuf::from("tests/input"));
        assert_eq!(result, 178159714);
    }
}
