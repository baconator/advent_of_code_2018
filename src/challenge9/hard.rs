extern crate regex;

use std::collections::hash_map::HashMap;

use self::regex::Regex;

#[test]
fn test_d9h() {
    let solutions = vec![("9 players; last marble is worth 25 points",
                          32,
    )];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

#[test]
fn test_stuff() {
    assert_eq!(4, 5-((-1 as i32).abs()%5));
}

use std::collections::LinkedList;

pub fn solve(mut lines: impl Iterator<Item=String>) -> usize {
    let re = Regex::new("^(\\d+) players; last marble is worth (\\d+) points$").unwrap();
    let line = lines.next().unwrap();
    let cap = re.captures(&line).unwrap();
    let player_count = cap[1].parse::<usize>().unwrap();
    let final_marble = cap[2].parse::<usize>().unwrap();

    let mut placed_marbles = vec![0];
    let mut current_marble_index = 0;
    let mut player_scores = HashMap::new();

    for mut marble in 1..=(final_marble*100) {
        use std::time::SystemTime;
        if marble % 100000 == 0 {
            println!("marble {} at {:#?}", marble, SystemTime::now());
        }
        if (marble % 23) == 0 && marble != 0 {
            let popped_marble_index = {
                let t = current_marble_index as i64 - 7;
                if t < 0 {
                    (placed_marbles.len() as i64 - (t.abs() % placed_marbles.len() as i64)) as usize
                } else {
                    t as usize
                }
            };
            let popped_marble = placed_marbles.remove(popped_marble_index);
            let player = marble % player_count;
            let score = player_scores.entry(player).or_insert(0);
            *score += (popped_marble + marble);
            current_marble_index = popped_marble_index % placed_marbles.len();
        } else {
            let next_marble_index = (current_marble_index + 2) % placed_marbles.len();
            placed_marbles.insert(next_marble_index, marble);
            current_marble_index = next_marble_index;
        }
    }
    *player_scores.values().max().unwrap()
}