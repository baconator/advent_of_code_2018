extern crate regex;
use self::regex::Regex;

use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;

#[test]
fn test_d6e() {
    let solutions = vec![(
"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
17,
    )];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
    id: usize
}

pub fn solve(lines: impl Iterator<Item = String>) -> usize {
    let re = Regex::new("^(\\d+), (\\d+)$").unwrap();
    let coords = lines.enumerate().filter_map(|(id, line)|
        if let Some(cap) = re.captures(&line) {
            Some(Position{ 
                x: cap[1].parse().unwrap(), 
                y: cap[2].parse().unwrap(),
                id
            })
        } else {
            None
        }
    ).collect::<Vec<_>>();
    let width = (coords.iter().map(|p| p.x).max().unwrap() + 1) as i32;
    let height = (coords.iter().map(|p| p.y).max().unwrap() + 1) as i32;

    let mut id_to_count = HashMap::new();
    let mut infinite_ids = HashSet::new();

    for x in (-1..=(width+1)) {
        for y in (-1..=(height+1)) {
            let mut distances = coords.clone();
            distances.sort_by_key(|c| (x-(c.x as i32)).abs() + (y-(c.y as i32)).abs());
            if distances[0] != distances[1] {
                let entry = id_to_count.entry(distances[0].clone()).or_insert(0);
                *entry += 1;
                if y == -1 || x == width+1 || y == height+1 || x == 0 {
                    infinite_ids.insert(distances[0].clone());
                }
            }
        }
    }

    let mut id_and_counts = id_to_count
        .iter()
        .filter(|p| !(infinite_ids.contains(p.0)))
        .collect::<Vec<_>>();
    id_and_counts.sort_by_key(|p| p.1);
    println!("{:#?}", id_and_counts);
    *id_and_counts.last().unwrap().1
}
