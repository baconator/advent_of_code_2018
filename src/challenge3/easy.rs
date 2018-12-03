use std::collections::HashMap;
use std::collections::HashSet;

#[test]
fn test() {
    let solutions = vec![("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab", 12)];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

pub fn solve(lines: impl Iterator<Item=String>) -> i32 {
    let mut letter_frequency_to_word_count = HashMap::new();
    for line in lines {
        let mut letter_to_frequencies = HashMap::new();
        for c in line.chars() {
            *letter_to_frequencies.entry(c).or_insert(0) += 1;
        }
        let frequencies: HashSet<_> = letter_to_frequencies.values().collect();
        for frequency in frequencies {
            *letter_frequency_to_word_count.entry(*frequency).or_insert(0) += 1;
        }
    }
    *letter_frequency_to_word_count.entry(2).or_insert(0) * *letter_frequency_to_word_count.entry(3).or_insert(0)
}