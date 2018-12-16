use std::collections::hash_map::HashMap;

#[test]
fn test_d14e() {
    let solutions = vec![(5, vec![0,1,2,4,5,1,5,8,9,1]),
                         (18, vec![9,2,5,1,7,1,0,8,5])];
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

pub fn solve(input: usize) -> Vec<usize> {
    let mut state = vec![3, 7];
    let mut elves = vec![0, 1];
    let target = input;
    while state.len() < target+10 {
        let sum: usize = elves.iter().map(|i| state[*i]).sum();
        state.append(&mut sum.digits(10));
        for elf_index in elves.iter_mut() {
            *elf_index = (*elf_index + 1 + state[*elf_index]) % state.len();
        }
    }
    state[target..target+10].to_vec()
}
