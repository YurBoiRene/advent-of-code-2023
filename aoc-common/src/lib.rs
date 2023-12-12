use clap::Parser;
use std::{fmt::Display, fs, path::PathBuf, time::Instant};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    input: PathBuf,
}

fn input_file() -> PathBuf {
    Cli::parse().input
}

pub fn run_puzzle_file<F, T>(f: F)
where
    F: FnOnce(PathBuf) -> T,
    T: Display,
{
    run_puzzle(|| f(input_file()));
}

pub fn run_puzzle_str<F, T>(f: F)
where
    F: FnOnce(&str) -> T,
    T: Display,
{
    run_puzzle(|| f(&fs::read_to_string(input_file()).unwrap()));
}

fn run_puzzle<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
    T: Display,
{
    println!("Running puzzle...");
    let start = Instant::now();
    let result = f();
    let elapsed = Instant::now() - start;
    println!("Result: {}", result);
    println!("Elapsed time: {:#?}", elapsed);

    result
}
