use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod challenge1;
mod challenge2;

fn main() -> Result<(), std::io::Error> {
    {
        let easy_lines = BufReader::new(File::open(Path::new("./inputs/1a"))?).lines().map(|line| line.unwrap());
        println!("D1 Easy: {}", challenge1::easy::run(easy_lines));

        let hard_lines = BufReader::new(File::open(Path::new("./inputs/1a"))?).lines().map(|line| line.unwrap());
        println!("D1 Hard: {}", challenge1::hard::solve(hard_lines));
    }

    {
        let easy_lines = BufReader::new(File::open(Path::new("./inputs/2a"))?).lines().map(|line| line.unwrap());
        println!("D2 Easy: {}", challenge2::easy::solve(easy_lines));

        let hard_lines = BufReader::new(File::open(Path::new("./inputs/2a"))?).lines().map(|line| line.unwrap());
        println!("D2 Hard: {}", challenge2::hard::solve(hard_lines));
    }
    Ok(())
}