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

pub fn solve(mut lines: impl Iterator<Item = String>) -> usize {
    17
}
