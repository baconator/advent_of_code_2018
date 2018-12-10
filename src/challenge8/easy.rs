#[test]
fn test_d8e() {
    let solutions = vec![("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2",
138,
    )];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

fn parse(data: &[usize]) -> (usize, usize)  {
    let child_count = data[0];
    let metadata_count = data[1];
    let mut metadata_sum = 0;
    let mut offset = 2;
    for _ in 0..child_count {
        let (child_size, child_metadata_sum) = parse(&data[offset..]);
        metadata_sum += child_metadata_sum;
        offset += child_size;
    }
    for _ in 0..metadata_count {
        metadata_sum += data[offset];
        offset += 1;
    }
    (offset, metadata_sum)
}

pub fn solve(mut lines: impl Iterator<Item = String>) -> usize {
    let line = lines.next().unwrap();
    let data = line
        .split(' ')
        .filter_map(|c| c.parse::<usize>().ok())
        .collect::<Vec<_>>();
    parse(&data[..]).1
}

