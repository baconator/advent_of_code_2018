use rayon::prelude::*;

#[test]
fn test_d10e() {
//    let solutions: Vec<(isize, (isize, isize))> = vec![(42, (21, 61)), (18, (33, 45))];
//    for (input, expected) in solutions {
//        assert_eq!(expected, solve(input));
//    }
}

fn coord_value(serial_number: isize, (x, y): (isize, isize)) -> isize {
    ((x + 10) * y + serial_number) * (x + 10) / 100 % 10 - 5
}

fn convolution(serial_number: isize, square_size: isize, (start_x, start_y): (isize, isize)) -> isize {
    (start_x..=start_x + square_size)
        .flat_map(|x| (start_y..=start_y + square_size).map(move |y| (x, y) as (isize, isize))).map(|pos| coord_value(serial_number, pos)).sum()
}

pub fn solve(serial_number: isize) -> (isize, isize, isize) {
    (1..=300).flat_map(|square_size| {
        println!("Generating square_size: {}", square_size);
        (0..=(300-square_size)).flat_map(move |x| (0..=300).map(move |y| { (square_size, x, y) }))
    }).max_by_key(|(square_size, x, y)| convolution(serial_number, *square_size, (*x, *y))).unwrap()
}