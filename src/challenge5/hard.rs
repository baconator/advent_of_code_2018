use std::collections::HashMap;
use std::collections::HashSet;

#[test]
fn test_d5h() {
    let solutions = vec![(
        "dabAcCaCBAcCcaDA",
        4,
    )];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

fn react(mut chars: Vec<&char>) -> Vec<&char> {
    let mut found_match = false;
    loop {
        found_match = false;
        for i in 0..chars.len()-1 {
            let first = chars[i];
            let second = chars[i+1];
            if first.eq_ignore_ascii_case(&second) {
                if first.is_uppercase() && second.is_lowercase() ||
                    first.is_lowercase() && second.is_uppercase() {
                    chars.remove(i);
                    chars.remove(i);
                    found_match = true;
                    break;
                }
            }
        }

        if !found_match {
            break;
        }
    }
    chars
}

pub fn solve(mut lines: impl Iterator<Item = String>) -> usize {
    let chars = lines.next().unwrap().chars().collect::<Vec<_>>();
    "abcdefghijklmnopqrstuvwxyz" 
        .chars()
        .map(|remove_c| react(chars
                  .iter()
                  .filter(|c| !c.eq_ignore_ascii_case(&remove_c))
                  .collect::<Vec<_>>()).len())
        .min()
        .unwrap()
}
