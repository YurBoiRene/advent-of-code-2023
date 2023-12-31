use clap::Parser;
use regex::Regex;
use std::{
    collections::HashMap,
    ops::{Add, Sub},
    path::PathBuf,
    rc::Rc,
    str::FromStr,
};

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
    let schematic = Schematic::from_str(input).unwrap();

    schematic
        .numbers
        .iter()
        .filter(|n| n.is_part_number(&schematic))
        .map(|n| n.value)
        .sum()
}

#[derive(Debug)]
struct Number {
    value: u32,
    location: Point,
    length: usize,
}

impl Number {
    /// Returns true if number is adjacent to a symbol
    fn is_part_number(&self, schematic: &Schematic) -> bool {
        let locations = self.locations();
        let mut checks: Vec<Point> = Vec::new();

        for location in locations {
            // Check line below
            checks.push(location + (1_usize, 0).into());

            if self.location.row > 0 {
                // Check line above
                checks.push(location - (1_usize, 0).into())
            }
        }

        if self.location.col > 0 {
            // Check line to left
            checks.push(self.location - (0, 1_usize).into());
            if self.location.row > 0 {
                // Top left corner
                checks.push(self.location - (1_usize, 1).into());
            }
            // Bottom left corner
            checks.push(self.location + (1_usize, 0).into() - (0, 1_usize).into());
        }

        let rightmost_location = *self.locations().last().unwrap();
        // Check line to right
        checks.push(rightmost_location + (0, 1_usize).into());
        if self.location.row > 0 {
            // Top right corner
            checks.push(rightmost_location - (1_usize, 0).into() + (0, 1_usize).into())
        }
        // Bottom right corner
        checks.push(rightmost_location + (1, 1_usize).into());

        // If any location around the number is a symbol
        checks
            .iter()
            .any(|location| schematic.get(location).is_symbol())
    }

    /// Returns vec of locations of digits making up the number
    fn locations(&self) -> Vec<Point> {
        (0..self.length)
            .map(|idx| self.location + (0, idx).into())
            .collect()
    }
}

#[derive(Debug, Default)]
enum Item {
    Symbol(char),
    Number(Rc<Number>),
    #[default]
    Other,
}

impl Item {
    fn is_symbol(&self) -> bool {
        matches!(self, Item::Symbol(_))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: usize,
    col: usize,
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, other: Point) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Self;

    fn sub(self, other: Point) -> Self {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

impl<T> From<(T, T)> for Point
where
    T: Into<usize>,
{
    fn from(value: (T, T)) -> Self {
        Point {
            row: value.0.into(),
            col: value.1.into(),
        }
    }
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Rc<Number>>,
    items: HashMap<Point, Item>,
}

impl Schematic {
    fn get(&self, point: &Point) -> &Item {
        self.items.get(point).unwrap_or(&Item::Other)
    }
}

#[derive(Debug)]
struct ParseStrErr;
impl FromStr for Schematic {
    type Err = ParseStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = HashMap::new();
        let mut numbers = Vec::new();

        for (idx, line) in s.lines().enumerate() {
            numbers.extend(schematic_parse_line(idx, line, &mut items));
        }

        Ok(Schematic { numbers, items })
    }
}

fn schematic_parse_line(
    line_index: usize,
    line: &str,
    items: &mut HashMap<Point, Item>,
) -> Vec<Rc<Number>> {
    let mut numbers = Vec::new();
    // This matches a group of periods OR a group of digits OR a group of symbols
    let re = Regex::new(r"[0-9]+|[^.^0-9]+|[.]+").unwrap();

    let mut idx = 0;
    for match_str in re.find_iter(line) {
        let location = Point {
            row: line_index,
            col: idx,
        };

        let current_char = match_str.as_str().chars().next().unwrap();
        match current_char {
            '.' => (),
            '0'..='9' => {
                let value = match_str
                    .as_str()
                    .parse()
                    .expect("Could not parse number in schematic");

                let length = match_str.len();

                let new_number = Rc::new(Number {
                    value,
                    location,
                    length,
                });

                for number_location in new_number.locations() {
                    items.insert(number_location, Item::Number(new_number.clone()));
                }
                numbers.push(new_number)
            }
            _ => {
                // Should be a single char symbol (#, $, _, etc.)
                items.insert(location, Item::Symbol(current_char));
            }
        }
        idx += match_str.len();
    }
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks computed result with correct result
    #[test]
    fn puzzle_result_test() {
        let input_filename = "tests/input";
        let result = run_puzzle_file(&PathBuf::from(input_filename));
        assert_eq!(result, 554003);
    }
}
