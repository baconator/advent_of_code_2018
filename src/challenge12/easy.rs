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

pub fn solve(mut lines: impl Iterator<Item=String>) -> usize {
    42
}
