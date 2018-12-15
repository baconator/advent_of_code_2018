extern crate aoc2018;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use std::thread;

const STACK_SIZE: usize = 32 * 1024 * 1024;

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

    if cfg!(feature = "day7") {
        println!("D7 Easy: {:#?}", aoc2018::challenge7::easy::solve(read_problem("7")).iter().collect::<String>());
    }

    if cfg!(feature = "day8") {
        println!("D8 Easy: {:#?}", aoc2018::challenge8::easy::solve(read_problem("8")));
        // Spawn thread with explicit stack size
        let child = thread::Builder::new()
            .stack_size(STACK_SIZE)
            .spawn(|| {
                println!("D8 Hard: {:#?}", aoc2018::challenge8::hard::solve(read_problem("8")));
            })
            .unwrap();

        // Wait for thread to join
        child.join().unwrap();
    }

    if cfg!(feature = "day9") {
        println!("D9 Easy: {:#?}", aoc2018::challenge9::easy::solve(read_problem("9")));
        println!("D9 Hard: {:#?}", aoc2018::challenge9::hard::solve(read_problem("9")));
    }

    if cfg!(feature = "day10") {
        println!("D10 Easy: {:#?}", aoc2018::challenge10::easy::solve(read_problem("10")));
    }

    if cfg!(feature = "day11") {
        println!("D11 Easy: {:#?}", aoc2018::challenge11::easy::solve(6878));
    }
    
    if cfg!(feature = "day12") {
        println!("D12 Easy: {:#?}", aoc2018::challenge12::easy::solve(read_problem("12")));
    }
    
    if cfg!(feature = "day13") {
        println!("D13 Easy: {:#?}", aoc2018::challenge13::easy::solve(read_problem("13")));
    }
   
    Ok(())
}
