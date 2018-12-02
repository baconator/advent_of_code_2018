#[test]
fn test() {
    let samples = vec![
        ("+1\n+1\n+1", 3),
        ("+1\n+1\n-2", 0),
        ("-1\n-2\n-3", -6)
    ];
    for (input, expected) in samples {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

pub fn solve(input: impl Iterator<Item=String>) -> i64 {
    input.map(|line| line.parse::<i64>().unwrap()).sum()
}