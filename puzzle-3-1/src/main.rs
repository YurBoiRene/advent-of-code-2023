use clap::Parser;
use grid::Grid;
use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
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

    let mut sum = 0;

    println!("Final: {sum}");
}

#[derive(Debug, Default)]
struct Location {
    line: usize,
    index: usize,
    length: usize,
}

#[derive(Debug, Default)]
struct Number {
    value: u32,
    location: Location,
}

#[derive(Debug, Default)]
enum Item {
    Symbol(char),
    Number(Rc<Number>),
    #[default]
    Other,
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Rc<Number>>,
    items: Grid<Item>,
}

#[derive(Debug)]
struct ParseStrErr;
impl FromStr for Schematic {
    type Err = ParseStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = Grid::new(0, 0);
        let mut numbers = Vec::new();

        for line in s.split('\n') {
            let mut row: Vec<Item> = Vec::with_capacity(items.cols());
            let mut current_number_string = String::new();
            let mut current_number = Number::default();
            for character in line.chars() {
                let next_item = match character {
                    '.' => Item::Other,
                    '0'..='9' => {
                        current_number_string.push(character);
                        Item::default()
                    }
                    _ => Item::Symbol(character),
                };
                row.push(next_item);
                if !current_number_string.is_empty() && !matches!(next_item, Item::Number(..)) {
                    // No more number, need to parse
                    let number_value: u32 = current_number_string.parse().unwrap();
                }

                still_on_number = false;
            }
        }

        Ok(Schematic { numbers, items })
    }
}

impl Schematic {
    fn is_part_number(&self, number: &Number) {
        todo!()
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
