use std::collections::HashMap;
use std::collections::HashSet;

extern crate regex;
use self::regex::Regex;

extern crate libc;

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
enum Contents {
    Empty,
    Claimed(usize),
    Overlapping
}

#[test]
fn test_d3h() {
    let solutions = vec![("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2", 3)];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

pub fn solve(lines: impl Iterator<Item=String>) -> i32 {
    let mut area: Vec<Vec<Contents>> = vec![vec![Contents::Empty; 1000]; 1000];
    let re = Regex::new("^#(\\d+) @ (\\d+),(\\d+): (\\d+)x(\\d+)$").unwrap();
    let mut overlapping_claims = HashSet::new();
    let mut all_claims = HashSet::new();
    for line in lines {
        if let Some(cap) = re.captures(&line) {
            let id = cap[1].parse::<usize>().unwrap();
            let left_offset = cap[2].parse::<usize>().unwrap();
            let top_offset = cap[3].parse::<usize>().unwrap();
            let width = cap[4].parse::<usize>().unwrap();
            let height = cap[5].parse::<usize>().unwrap();

            all_claims.insert(id);

            for x in left_offset..(left_offset + width) {
                for y in top_offset..(top_offset + height) {
                    area[x][y] = match area[x][y] {
                        Contents::Empty => Contents::Claimed(id),
                        Contents::Claimed(existing) => {
                            overlapping_claims.insert(existing);
                            overlapping_claims.insert(id);
                            Contents::Overlapping
                        },
                        Contents::Overlapping => {
                            overlapping_claims.insert(id);
                            Contents::Overlapping
                        }
                    };
                }
            }
        }
    }
    println!("Differences: {:#?}", all_claims.difference(&overlapping_claims));
    *all_claims.difference(&overlapping_claims).next().unwrap() as i32
}
