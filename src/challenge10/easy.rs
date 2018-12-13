#[test]
fn test_d10e() {
    let solutions = vec![include_str!("../../inputs/10t")];
    for input in solutions {
        solve(input.lines().map(|l| l.to_owned()));
    }
}

extern crate regex;
use self::regex::Regex;

type PositionAndVelocity = ((isize, isize), (isize, isize));
fn next_positions(old: &Vec<PositionAndVelocity>) -> Vec<PositionAndVelocity> {
    old.iter().map(|((x_pos, y_pos), (x_vel, y_vel))| {
        ((x_pos + x_vel, y_pos + y_vel), (*x_vel, *y_vel))
    }).collect()
}

extern crate itertools;
use self::itertools::Itertools;

fn score_positions(pos_and_vels: &Vec<PositionAndVelocity>) -> usize {
    let mut in_line = 0;
    let all_positions = &pos_and_vels.iter().map(|(p, _)| p).group_by(|(x, _)| x);
    for (_, positions) in all_positions {
        let mut sorted_positions = positions.collect::<Vec<_>>();
        sorted_positions.sort_unstable();
        let mut 
        for (_, y) in sorted_positions.iter() {
            
        }
    }
    in_line
}

pub fn solve(mut lines: impl Iterator<Item=String>) {
    let re = Regex::new("^position=<\\s*(-?)(\\d+),\\s*(-?)(\\d+)> velocity=<\\s*(-?)(\\d+),\\s*(-?)(\\d+)>$").unwrap();
    let mut positions = lines.map(|line| {
        let caps = re.captures(&line).unwrap();
        (
            ((caps[1].to_owned() + &caps[2]).parse::<isize>().unwrap(), (caps[3].to_owned() + &caps[4]).parse::<isize>().unwrap()),
            ((caps[5].to_owned() + &caps[6]).parse::<isize>().unwrap(), (caps[7].to_owned() + &caps[8]).parse::<isize>().unwrap())
        )
    }).collect::<Vec<_>>();
    positions.sort_unstable();
    println!("{:#?}", score_positions(&positions));
}
