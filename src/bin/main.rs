extern crate aoc2018;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


fn main() -> Result<(), std::io::Error> {
    if false {
        let easy_lines = BufReader::new(File::open(Path::new("./inputs/1a"))?).lines().map(|line| line.unwrap());
        println!("D1 Easy: {}", aoc2018::challenge1::easy::solve(easy_lines));

        let hard_lines = BufReader::new(File::open(Path::new("./inputs/1a"))?).lines().map(|line| line.unwrap());
        println!("D1 Hard: {}", aoc2018::challenge1::hard::solve(hard_lines));
    }

    if false {
        let easy_lines = BufReader::new(File::open(Path::new("./inputs/2a"))?).lines().map(|line| line.unwrap());
        println!("D2 Easy: {}", aoc2018::challenge2::easy::solve(easy_lines));

        let hard_lines = BufReader::new(File::open(Path::new("./inputs/2a"))?).lines().map(|line| line.unwrap()).collect::<Vec<_>>();
        println!("D2 Hard: {}", aoc2018::challenge2::hard::solve(&hard_lines));
    }

    if false {
        let easy_lines = BufReader::new(File::open(Path::new("./inputs/3"))?).lines().map(|line| line.unwrap());
        println!("D3 Easy: {}", aoc2018::challenge3::easy::solve(easy_lines));

        let hard_lines = BufReader::new(File::open(Path::new("./inputs/3"))?).lines().map(|line| line.unwrap());
        println!("D3 Hard: {}", aoc2018::challenge3::hard::solve(hard_lines));
    }
 
    {
        let easy_lines = BufReader::new(File::open(Path::new("./inputs/4"))?).lines().map(|line| line.unwrap());
        println!("D4 Easy: {}", aoc2018::challenge4::easy::solve(easy_lines));

        let hard_lines = BufReader::new(File::open(Path::new("./inputs/4"))?).lines().map(|line| line.expect("A line?"));
        println!("D4 Hard: {}", aoc2018::challenge4::hard::solve(hard_lines));
    }
    Ok(())
}
