use std::collections::HashSet;

#[test]
fn test() {
    let samples = vec![("+1\n-2\n+3\n+1", 2)];
    for (input, expected) in samples {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

pub fn solve(input: impl Iterator<Item=String>) -> i64 {
    let values = input
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut seen_sums = HashSet::new();
    let mut current_sum = 0;
    loop {
        for value in values.iter() {
            current_sum += value;
            if seen_sums.contains(&current_sum) {
                return current_sum
            } else {
                seen_sums.insert(current_sum);
            }
        }
    }
}