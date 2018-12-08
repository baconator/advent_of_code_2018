extern crate aoc2018;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_problem(input_name: &str) -> impl Iterator<Item=String> {
    let path = format!("./inputs/{}", input_name);
    BufReader::new(File::open(Path::new(&path)).unwrap())
        .lines()
        .map(|line| line.unwrap())
}

fn main() -> Result<(), std::io::Error> {
    if cfg!(feature = "day1") {
        println!("D1 Easy: {}", aoc2018::challenge1::easy::solve(read_problem("1a")));
        println!("D1 Hard: {}", aoc2018::challenge1::hard::solve(read_problem("1a")));
    }

    if cfg!(feature = "day2") {
        println!("D2 Easy: {}", aoc2018::challenge2::easy::solve(read_problem("2a")));
        println!("D2 Hard: {}", aoc2018::challenge2::hard::solve(read_problem("2a")));
    }

    if cfg!(feature = "day3") {
        println!("D3 Easy: {}", aoc2018::challenge3::easy::solve(read_problem("3")));
        println!("D3 Hard: {}", aoc2018::challenge3::hard::solve(read_problem("3")));
    }

    if cfg!(feature = "day4") {
        println!("D4 Easy: {}", aoc2018::challenge4::easy::solve(read_problem("4")));
        println!("D4 Hard: {}", aoc2018::challenge4::hard::solve(read_problem("4")));
    }
    
    if cfg!(feature = "day5") {
        println!("D5 Easy: {:#?}", aoc2018::challenge5::easy::solve(read_problem("5")));
        println!("D5 Hard: {:#?}", aoc2018::challenge5::hard::solve(read_problem("5")));
    }
    
    if cfg!(feature = "day6") {
        println!("D6 Easy: {:#?}", aoc2018::challenge6::easy::solve(read_problem("6")));
        //println!("D6 Hard: {:#?}", aoc2018::challenge6::hard::solve(read_problem("6")));
    }
    Ok(())
}
