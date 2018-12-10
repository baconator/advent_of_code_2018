#[test]
fn test_d8h() {
    let solutions = vec![("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2",
                          66,
    )];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

fn parse(data: &[usize]) -> (usize, usize) {
    let child_count = data[0];
    let metadata_count = data[1];
    let mut offset = 2;
    let mut child_sums = vec![0];
    for _ in 0..child_count {
        let (child_size, child_metadata_sum) = parse(&data[offset..]);
        offset += child_size;
        child_sums.push(child_metadata_sum);
    }
    let metadata = &data[offset..offset + metadata_count];
    let output = if child_count == 0 {
        (offset + metadata_count, metadata.iter().sum())
    } else {
        (offset + metadata_count, metadata.iter()
            .filter_map(|index| child_sums.get(*index)).sum())
    };
    println!("{:?}", &output);
    output
}

pub fn solve(mut lines: impl Iterator<Item=String>) -> usize {
    let line = lines.next().unwrap();
    let data = line
        .split(' ')
        .filter_map(|c| c.parse::<usize>().ok())
        .collect::<Vec<_>>();
    parse(&data[..]).1
}

