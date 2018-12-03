use std::collections::HashMap;
use std::collections::HashSet;

extern crate regex;
use self::regex::Regex;

#[derive(Copy)]
#[derive(Clone)]
enum Contents {
    Empty,
    Claimed(u32),
    Overlapping
}

#[test]
fn test_d3e() {
    let solutions = vec![("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2", 12)];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

pub fn solve(lines: impl Iterator<Item=String>) -> i32 {
    let area = [0 as u8; 4000000];
    let re = Regex::new("^#(\\d+) @ (\\d+),(\\d+): (\\d+)x(\\d+)$").unwrap();
    panic!();
/*    for line in lines {
        if let Some(cap) = re.captures(&line) {
        
        }
    }
    panic!();*/
}
