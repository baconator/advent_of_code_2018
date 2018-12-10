extern crate regex;
use self::regex::Regex;

use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;

fn parse(lines: impl Iterator<Item = String>) -> 
(Vec<char>, HashMap<char, HashSet<char>>, HashMap<char, HashSet<char>>) {
    let re = Regex::new("^Step (.).*step (.) can .*$").unwrap();
    let mut reqs = HashMap::new();
    let mut nodes = HashMap::new();
    let mut non_root_nodes = HashSet::new();
    for line in lines {
        if let Some(cap) = re.captures(&line) {
            let from_node = cap[1].chars().next().unwrap();
            let to_node = cap[2].chars().next().unwrap();
            nodes.entry(from_node)
                .or_insert(HashSet::new())
                .insert(to_node);
            non_root_nodes.insert(to_node);
            reqs.entry(to_node)
                .or_insert(HashSet::new())
                .insert(from_node);
        }
    }
    let all_nodes = nodes.keys()
        .map(|k| k.clone())
        .collect::<HashSet<_>>();
    let roots = all_nodes
        .difference(&non_root_nodes)
        .map(|c| c.clone())
        .collect::<Vec<_>>();
    (roots, nodes, reqs)
}

use std::cmp::Reverse;

fn reqs_met(current: &char, 
            reqs: &HashMap<char, HashSet<char>>, 
            output: &Vec<char>) -> bool {
    reqs.get(current)
        .unwrap_or(&HashSet::new())
        .iter()
        .filter(|r| !output.contains(r))
        .count() == 0
}

type Worker = (usize, Option<char>);
fn available_workers(workers: &Vec<Worker>) -> Vec<&Worker> {
    workers.iter().filter(|(n, o)| *n == 0).collect::<Vec<_>>()
}

fn increment_workers(workers: &mut Vec<Worker>) {
    for worker in workers {
        if worker.0 > 0 {
            worker.0 -= 1;
        }
    }
}

fn workers_contains(workers: &Vec<Worker>, c: char) -> bool {
    workers.iter().any(|w| if let (_, Some(other_c)) = w {
        *other_c == c
    } else {
        false
    })
}

pub fn solve(lines: impl Iterator<Item = String>) -> Vec<char> {
    let (roots, nodes, reqs) = parse(lines);
    let mut output = vec![];
    let mut next_up = roots.clone();
    let mut workers = vec![(0, None); 5];
    let mut seconds_taken = 0;
    while !available_workers(&workers).is_empty() || !next_up.is_empty() {
        let current = next_up.pop().unwrap();
        if let Some(expanded) = nodes.get(&current) {
            let next_workers = available_workers(&workers);
            let mut t = expanded.iter()
                .filter(|c| !output.contains(c) && 
                        !next_up.contains(c) &&
                        !workers_contains(&workers, **c))
                .map(|c| c.clone())
                .collect::<Vec<_>>();
            next_up.append(&mut t);
        }
        next_up.sort_by_key(|k| (reqs_met(k, &reqs, &output) as u32, Reverse(*k)));
        increment_workers(&mut workers);
        seconds_taken += 1;
    }
    output
}
