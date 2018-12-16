extern crate regex;

use std::collections::hash_map::HashMap;

#[test]
fn test_d13e() {
    let solutions = vec![("/->-\\        
|   |  /----\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/   ",
                          (7,3),
    )];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug)]
enum Track {
    Straight,
    Corner,
    Intersection
}

fn next_state(map: &HashMap<(usize, usize), Track>, 
              carts: ((usize, usize), Direction)) -> HashMap<(usize, usize), Track> {
    let carts = map.iter()
        .filter_map(|(loc, elem)| if let Track::Cart(_) = elem { 
            Some(loc) 
        } else { 
            None });
    let mut output = HashMap::new();
    for cart in carts {
        let new_position = if let Track::Cart(direction) = map[cart] {
            match direction {
                Direction::North => (cart.0, cart.1-1),
                Direction::South => (cart.0, cart.1+1),
                Direction::East => (cart.0+1, cart.1),
                Direction::West => (cart.0-1, cart.1)
            }
        } else { panic!() };
    }

}

extern crate either;
use self::either::Either;

pub fn solve(mut lines: impl Iterator<Item=String>) -> (usize, usize) {
    let map_with_carts = lines.enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, c)| {
            match c {
                    '/' | '\\' => Some(((x, y), Either::Left(Track::Corner))),
                    '+' => Some(((x, y), Either::Left(Track::Intersection))),
                    '>' => Some(((x, y), Either::Right(Direction::East))),
                    'v' => Some(((x, y), Either::Right(Direction::South))),
                    '<' => Some(((x, y), Either::Right(Direction::West))),
                    '^' => Some(((x, y), Either::Right(Direction::North))),
                    '|' | '-' => Some(((x, y), Either::Left(Track::Straight)))
                    _ => None
                }
            }).collect::<Vec<_>>()
    }).collect::<HashMap<_, _>>();

    let map = map_with_carts.iter().map(|(pos, elem)| {
        match elem {
            Either::Left(track) => (pos, track),
            Either::Right(cart_direction) => {
                let adjacent = vec![
                    (pos.0-1, pos.1), 
                    (pos.0+1, pos.1), 
                    (pos.0, pos.1-1), 
                    (pos.0, pos.1+1)].iter()
                        .filter_map(|p| if let Either::Left(t) = p {
                            Some(t)
                        } else {
                            None
                        } )
                        .collect::<Vec<_>>();
                let straights = adjacent.iter()
                    .filter(|a| a == Track::Straight)
                    .count();
                let corners = adjacent.iter()
                    .filter(|a| a == Track::Corner)
                    .count();
                let intersections = adjacent.iter()
                    .filter(|a| a == Track::Intersection)
                    .count();

                if straights == 2 {
                    Track::Straight
                }
            }
        }
    }
    })
    println!("{:?}", map);
    (42, 42)
}
