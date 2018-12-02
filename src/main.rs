use std::fs::File;
use std::io::{BufRead, BufReader};

mod challenge1;
mod challenge2;

fn main() -> Result<(), std::io::Error> {
    {
        let easy_lines = BufReader::new(File::open("F:\\Projects\\advent_of_code_2018\\inputs\\1a")?).lines().map(|line| line.unwrap());
        println!("A: {}", challenge1::easy::run(easy_lines));

        let hard_lines = BufReader::new(File::open("F:\\Projects\\advent_of_code_2018\\inputs\\1b")?).lines().map(|line| line.unwrap());
        println!("B: {}", challenge1::hard::solve(hard_lines));
    }

    let easy_lines = BufReader::new(File::open("F:\\Projects\\advent_of_code_2018\\inputs\\2a")?).lines().map(|line| line.unwrap());
    println!("A: {}", challenge2::easy::solve(easy_lines));
    Ok(())
}