extern crate regex;
use self::regex::Regex;

use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;

#[test]
fn test_d7e() {
    let solutions = vec![(
"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.",
vec!['C', 'A', 'B', 'D', 'F', 'E'],
    )];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

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

pub fn solve(lines: impl Iterator<Item = String>) -> Vec<char> {
    let (roots, nodes, reqs) = parse(lines);
    let mut output = vec![];
    let mut next_up: Vec<char> = roots.clone();
    next_up.sort_by_key(|c| Reverse(*c));
    while !next_up.is_empty() {
        let current = next_up.pop().unwrap();
        if !reqs_met(&current, &reqs, &output) {
            panic!();
        }
        if output.contains(&current) { continue };
        output.push(current.clone());
        if let Some(expanded) = nodes.get(&current) {
            let mut t = expanded.iter()
                .filter(|c| !output.contains(c) && !next_up.contains(c))
                .map(|c| c.clone())
                .collect::<Vec<_>>();
            next_up.append(&mut t);
        }
        next_up.sort_by_key(|k| (reqs_met(k, &reqs, &output) as u32, Reverse(*k)));
    }
    output
}
