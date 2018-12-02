use std::collections::HashSet;

#[test]
fn test() {
    let solutions = vec![("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz", "fgij")];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

pub fn solve(lines: impl Iterator<Item=String>) -> String {
    let mut seen_keys: HashSet<(String, usize)> = HashSet::new();
    for line in lines {
        for i in 0..line.len() {
            let key = (format!("{}{}", line[0..i].to_owned(), line[i+1..line.len()].to_owned()), i);
            if seen_keys.contains(&key) {
                return key.0;
            } else {
                seen_keys.insert(key);
            }
        }
    }
    panic!();
}