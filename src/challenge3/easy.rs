extern crate regex;
use self::regex::Regex;

extern crate libc;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Contents {
    Empty,
    Claimed(usize),
    Overlapping,
}

#[test]
fn test_d3e() {
    let solutions = vec![("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2", 4)];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

pub fn solve(lines: impl Iterator<Item = String>) -> i32 {
    let mut area: Vec<Vec<Contents>> = vec![vec![Contents::Empty; 1000]; 1000];
    let re = Regex::new("^#(\\d+) @ (\\d+),(\\d+): (\\d+)x(\\d+)$").unwrap();
    for line in lines {
        if let Some(cap) = re.captures(&line) {
            let id = cap[1].parse::<usize>().unwrap();
            let left_offset = cap[2].parse::<usize>().unwrap();
            let top_offset = cap[3].parse::<usize>().unwrap();
            let width = cap[4].parse::<usize>().unwrap();
            let height = cap[5].parse::<usize>().unwrap();

            for x in left_offset..(left_offset + width) {
                for y in top_offset..(top_offset + height) {
                    area[x][y] = match area[x][y] {
                        Contents::Empty => Contents::Claimed(id),
                        Contents::Claimed(_) => Contents::Overlapping,
                        Contents::Overlapping => Contents::Overlapping,
                    };
                }
            }
        }
    }
    area.iter()
        .map(|row| row.iter().filter(|c| **c == Contents::Overlapping).count())
        .sum::<usize>() as i32
}
