#[test]
fn test_d10e() {
    let solutions: Vec<(isize, (isize, isize))> = vec![(42, (21, 61)), (18, (33, 45))];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input));
    }
}

fn coord_value(serial_number: isize, (x, y): (isize, isize)) -> isize {
    ((x + 10) * y + serial_number) * (x + 10) / 100 % 10 - 5
}

fn convolution(serial_number: isize, (start_x, start_y): (isize, isize)) -> isize {
    (start_x..=start_x + 2)
        .flat_map(|x| (start_y..=start_y + 2).map(move |y| (x, y) as (isize, isize))).map(|pos| coord_value(serial_number, pos)).sum()
}

pub fn solve(serial_number: isize) -> (isize, isize) {
    (0..=300).flat_map(|x| (0..=300).map(move |y| { (x, y) })).max_by_key(|(x, y)| convolution(serial_number, (*x, *y))).unwrap()
}