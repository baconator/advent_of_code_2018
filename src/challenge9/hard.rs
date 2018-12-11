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

struct AwfulTree<T> {
    data: Vec<Vec<T>>
}

const MAX_SIZE: usize = 1000;
impl <T> AwfulTree<T> {
    fn len(&self) -> usize {
        self.data.iter().map(|d| d.len()).sum()
    }

    fn insert(&mut self, mut index: usize, item: T) {
        let mut data_index = 0;
        let mut data_index_to_split = None;
        let mut found = false;
        for mut data in self.data.iter_mut() {
            if index < data.len() {
               data.insert(index, item); 
               if data.len() >= MAX_SIZE {
                   data_index_to_split = Some(data_index);
               }
               found = true;
               break;
            } else {
                index -= data.len();
            }
            data_index += 1;
        }
        if !found { panic!(); }
        if let Some(di) = data_index_to_split {
            let spleet = self.data[di].split_off(MAX_SIZE);
            self.data.insert(di, spleet);
        }
    }

    fn remove(&mut self, mut index: usize) -> T {
        for mut data in self.data.iter_mut() {
            if index < data.len() {
                return data.remove(index);
            } else {
                index -= data.len();
            }
        }
        panic!();
    }
}

pub fn solve(mut lines: impl Iterator<Item=String>) -> u64 {
    let re = Regex::new("^(\\d+) players; last marble is worth (\\d+) points$").unwrap();
    let line = lines.next().unwrap();
    let cap = re.captures(&line).unwrap();
    let player_count = cap[1].parse::<u64>().unwrap();
    let final_marble = cap[2].parse::<u64>().unwrap();

    let mut placed_marbles = AwfulTree { data: vec![vec![0]] };
    let mut current_marble_index = 0;
    let mut player_scores = HashMap::new();
    
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut last_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    for mut marble in 1..=(final_marble*100) {
        if marble % 100000 == 0 {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            println!("marble {} took {:?}", marble, now-last_time);
            last_time = now;
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
