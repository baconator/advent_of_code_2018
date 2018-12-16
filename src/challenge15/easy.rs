use std::collections::hash_map::HashMap;

#[test]
fn test_d15e() {
    let solutions = vec![("
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
", 36334)];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().into_iter().map(|l| l.to_owned())));
    }
}

pub fn solve(mut lines: impl Iterator<Item = String>) -> usize {
    42
}
