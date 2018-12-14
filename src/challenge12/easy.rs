extern crate regex;

use std::collections::hash_map::HashMap;

use self::regex::Regex;

#[test]
fn test_d12e() {
    let solutions = vec![("initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #",
                          325,
    )];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

enum Plant {
    Alive,
    Dead
}

fn parse_input(c: char) -> Option<Plant> {
    if c == '#' {
        Some(Plant::Alive)
    } else if c == '.' {
        Some(Plant::Dead)
    } else {
        panic!()
    }
}

pub fn solve(mut lines: impl Iterator<Item=String>) -> usize {
    let initial_state_re = Regex::new("^initial state: ([#|\\.]+)$").unwrap();
    let pattern_re = Regex::new("^([#|\\.]+) => ([#|\\.])$").unwrap();
    let initial_state_line = lines.next().unwrap();
    let initial_state = initial_state_re
        .captures(&initial_state_line)
        .unwrap()[1]
        .chars()
        .filter_map(parse_input)
        .collect::<Vec<_>>();
    lines.map(|line| {
        let cap = pattern_re.captures(&line).unwrap();
        let left = cap[1].chars().filter_map(parse_input).collect::<Vec<_>>();
        let right = if cap[2].chars().next().unwrap() == '#' {
            Plant::Alive
        } else {
            Plant::Dead
        };
        42
    });
    42
}
