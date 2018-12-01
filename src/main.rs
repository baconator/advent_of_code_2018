use std::fs::File;
use std::io::{BufReader, BufRead};

mod challenge1;

fn main() -> Result<(), std::io::Error> {
    let f = BufReader::new(File::open("F:\\Projects\\advent_of_code_2018\\inputs\\1a")?);

    let lines = f.lines().map(|line| line.unwrap());
    println!("A: {}", challenge1::easy::run(lines));
    Ok(())
}