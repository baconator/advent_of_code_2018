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
(char, HashMap<char, HashSet<char>>) {
    let re = Regex::new("^Step (.).*step (.) can .*$").unwrap();
    let mut nodes = HashMap::new();
    let mut non_root_nodes = HashSet::new();
    for line in lines {
        if let Some(cap) = re.captures(&line) {
            let to_node = cap[2].chars().next().unwrap();
            nodes.entry(cap[1].chars().next().unwrap())
                .or_insert(HashSet::new())
                .insert(to_node);
            non_root_nodes.insert(to_node);
        }
    }
    let all_nodes = nodes.keys()
        .map(|k| k.clone())
        .collect::<HashSet<_>>();
    let root = all_nodes.difference(&non_root_nodes).next().unwrap();
    (*root, nodes)
}

pub fn solve(lines: impl Iterator<Item = String>) -> Vec<char> {
    let (root, nodes) = parse(lines);
    vec![]
}

