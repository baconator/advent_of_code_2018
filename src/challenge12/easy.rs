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

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
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

fn print_state(state: &Vec<Plant>) {
    for s in state.iter() {
        print!("{}", match s {
            Plant::Alive => "#",
            Plant::Dead => "."
        });
    }
    print!("\n");
}

fn next_state(states: &Vec<Plant>, patterns: &HashMap<Vec<Plant>, Plant>) -> 
Vec<Plant> {
    states
        .iter()
        .enumerate()
        .map(|(i, state)| {
            if i == 0 || i == 1 || i == states.len()-1 || i == states.len()-2 {
                Plant::Dead
            } else {
                patterns
                    .iter()
                    .filter_map(|pattern| {
                        if pattern.0[0..] == states[i-2..i+2] {
                            Some(pattern.1.clone())
                        } else {
                            None
                        }
                    })
                    .next()
                    .unwrap_or(states[i].clone())
                    .clone()
            }
        })
        .collect::<Vec<_>>()
}

pub fn solve(mut lines: impl Iterator<Item=String>) -> usize {
    let initial_state_re = Regex::new("^initial state: ([#|\\.]+)$").unwrap();
    let pattern_re = Regex::new("^([#|\\.]+) => ([#|\\.])$").unwrap();
    let initial_state_line = lines.next().unwrap();
    let mut state = initial_state_re
        .captures(&initial_state_line)
        .unwrap()[1]
        .chars()
        .filter_map(parse_input)
        .collect::<Vec<_>>();
    for _ in 0..2 { state.insert(0, Plant::Dead); }
    lines.next();
    let patterns = lines.map(|line| {
        let cap = pattern_re.captures(&line).unwrap();
        let left = cap[1].chars().filter_map(parse_input).collect::<Vec<_>>();
        let right = parse_input(cap[2].chars().next().unwrap()).unwrap();
        (left, right)
    }).collect::<HashMap<_, _>>();
    for _ in 0..20 {
        print_state(&state);
        print_state(&next_state(&state, &patterns)); 
    }
    42
}
