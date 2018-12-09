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

pub fn solve(lines: impl Iterator<Item = String>) -> usize {
    let re = Regex::new("^(\\d+), (\\d+)$").unwrap();
    let coords = lines.enumerate().filter_map(|(id, line)|
        if let Some(cap) = re.captures(&line) {
            Some((
                    id,
                    cap[1].parse().unwrap(), 
                    cap[2].parse().unwrap(),
            ))
        } else {
            None
        }
    ).collect::<Vec<_>>();
    let width = (coords.iter().map(|p| p.1).max().unwrap() + 1) as i32;
    let height = (coords.iter().map(|p| p.2).max().unwrap() + 1) as i32;

    let mut id_to_count = HashMap::new();
    let mut infinite_ids = HashSet::new();

    let edge = 1; 
    for x in (-edge..=(width+edge)) {
        for y in (-edge..=(height+edge)) {
            let mut distances = coords
                .iter()
                .map(|(id, c_x, c_y)| (id, (c_x-x).abs()+(c_y-y).abs()))
                .collect::<Vec<_>>();
            distances.sort_by_key(|&(_, d)| d);
            let closest = distances[0];
            if closest != distances[1] {
                *id_to_count.entry(closest.0).or_insert(0) += 1;
                if y == -edge || x == width+edge || y == height+edge || x == -edge {
                    infinite_ids.insert(closest.0);
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
