use std::collections::hash_map::HashMap;

#[test]
fn test_d14e() {
    let solutions = vec![(5, 0124515891)];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input));
    }
}

trait Digits {
    fn digits(&self, base: usize) -> Vec<usize>;
}

impl Digits for usize {
    fn digits(&self, base: usize) -> Vec<usize> {
        let mut output = vec![];
        let mut current = *self;
        while current >= base {
            output.push(current % 10);
            current /= 10;
        }
        output.push(current);
        output.reverse();
        output
    }
}

pub fn solve(input: usize) -> usize {
    let mut state = vec![3, 7];
    let mut elves = vec![0, 1];
    let mut sum: usize = elves.iter().map(|i| state[*i]).sum();
    println!("{:?}", sum.digits(10));
    42
}
