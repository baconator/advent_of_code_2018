extern crate itertools;
extern crate regex;

use self::itertools::Itertools;
use self::regex::Regex;

#[test]
fn test_d10e() {
    let solutions = vec![include_str!("../../inputs/10t")];
    for input in solutions {
        solve(input.lines().map(|l| l.to_owned()));
    }
}

type PositionAndVelocity = ((isize, isize), (isize, isize));

fn next_positions(old: &Vec<PositionAndVelocity>) -> Vec<PositionAndVelocity> {
    old.iter().map(|((x_pos, y_pos), (x_vel, y_vel))| {
        ((x_pos + x_vel, y_pos + y_vel), (*x_vel, *y_vel))
    }).collect()
}

fn score_positions(pos_and_vels: &Vec<PositionAndVelocity>) -> usize {
    let mut in_line = 0;
    let all_positions = &pos_and_vels.iter().map(|(p, _)| p).group_by(|(x, _)| x);
    for (_, positions) in all_positions {
        let mut sorted_positions = positions.map(|(_, y)| y).collect::<Vec<_>>();
        sorted_positions.sort_unstable();
        for y in 0..sorted_positions.len() {
            let before = if y == 0 { None } else { sorted_positions.get(y - 1) };
            let current = sorted_positions[y];
            let after = sorted_positions.get(y + 1);
            let mut add = false;
            if let Some(v) = before {
                if **v + 1 == *current {
                    add = true;
                }
            }
            if let Some(v) = after {
                if **v - 1 == *current {
                    add = true;
                }
            }
            if add {
                in_line += 1;
            }
        }
    }
    in_line
}

use std::collections::hash_set::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn print_state(pos_and_vels: &Vec<PositionAndVelocity>) {
    let min_y = pos_and_vels.iter().map(|((_, y), _)| y).min().unwrap();
    let min_x = pos_and_vels.iter().map(|((x, _), _)| x).min().unwrap();
    let max_y = pos_and_vels.iter().map(|((_, y), _)| y).max().unwrap();
    let max_x = pos_and_vels.iter().map(|((x, _), _)| x).max().unwrap();

    let f = File::create("foo.txt").unwrap();
    let mut writer = BufWriter::new(f);

    let positions = pos_and_vels.iter().map(|(pos, _)| pos).collect::<HashSet<_>>();
    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            if positions.contains(&(x, y)) {
                writer.write("#".as_bytes());
            } else {
                writer.write(".".as_bytes());
            }
        }
        writer.write("\n".as_bytes());
    }
}

pub fn solve(mut lines: impl Iterator<Item=String>) {
    let re = Regex::new("^position=<\\s*(-?)(\\d+),\\s*(-?)(\\d+)> velocity=<\\s*(-?)(\\d+),\\s*(-?)(\\d+)>$").unwrap();
    let mut positions_and_velocities = lines.map(|line| {
        let caps = re.captures(&line).unwrap();
        (
            ((caps[1].to_owned() + &caps[2]).parse::<isize>().unwrap(), (caps[3].to_owned() + &caps[4]).parse::<isize>().unwrap()),
            ((caps[5].to_owned() + &caps[6]).parse::<isize>().unwrap(), (caps[7].to_owned() + &caps[8]).parse::<isize>().unwrap())
        )
    }).collect::<Vec<_>>();
    positions_and_velocities.sort_unstable();
    let total = positions_and_velocities.len();
    let mut current_positions_and_velocities = positions_and_velocities.clone();
    for seconds in 0..100000 {
        if score_positions(&current_positions_and_velocities) > 24 {
            use std::io;

            println!("Hit any key to continue (second: {}).", seconds);
            print_state(&current_positions_and_velocities);
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
        }
        current_positions_and_velocities = next_positions(&current_positions_and_velocities);
        assert_eq!(total, current_positions_and_velocities.len());
    }
}
