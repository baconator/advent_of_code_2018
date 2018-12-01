use std::fs::File;
use std::io::{BufReader, BufRead};

mod challenge1;

fn main() -> Result<(), std::io::Error> {
    let easy_lines = BufReader::new(File::open("F:\\Projects\\advent_of_code_2018\\inputs\\1a")?).lines().map(|line| line.unwrap());
    println!("A: {}", challenge1::easy::run(easy_lines));

    let hard_lines = BufReader::new(File::open("F:\\Projects\\advent_of_code_2018\\inputs\\1b")?).lines().map(|line| line.unwrap());
    println!("B: {}", challenge1::hard::solve(hard_lines));
    Ok(())
}