#[test]
fn test_d8h() {
    let solutions = vec![("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2",
    66,
    )];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

fn parse(data: &[usize]) -> (usize, usize)  {
    let child_count = data[0];
    let metadata_count = data[1];
    let mut offset = 2;
    let mut child_sums = vec![0];
    child_sums.extend((0..child_count).map(|_| {
        let (child_size, child_metadata_sum) = parse(&data[offset..]);
        offset += child_size;
        child_metadata_sum
    }));
    println!("Child sums: {:?}", child_sums);
    let output = if child_count == 0 {
        let mut metadata_sum = 0;
        for _ in 0..metadata_count {
            metadata_sum += data[offset];
            offset += 1;
        }
        (offset, metadata_sum)
    } else {
        let mut output_sum = 0;
        for _ in 0..metadata_count {
            println!("Getting sum from offset {}", offset);
            if let Some(v) = child_sums.get(data[offset]) {
                output_sum += v;
                offset += 1;
            }
        }
        (offset, output_sum)
    };
    println!("{:?}", &output);
    output
}

pub fn solve(mut lines: impl Iterator<Item = String>) -> usize {
    let line = lines.next().unwrap();
    let data = line
        .split(' ')
        .filter_map(|c| c.parse::<usize>().ok())
        .collect::<Vec<_>>();
    parse(&data[..]).1
}

