use std::collections::HashMap;
use std::collections::HashSet;

extern crate itertools;

extern crate regex;
use self::regex::Regex;

#[test]
fn between_works() {
    assert_eq!(vec![0, 1, 2, 59], between(59, 3));
    assert_eq!(vec![0, 1, 2, 3], between(0, 4));
}

#[test]
fn test_d4h() {
    let solutions = vec![(
        "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up",
        4455,
    )];
    for (input, expected) in solutions {
        assert_eq!(expected, solve(input.lines().map(|l| l.to_owned())));
    }
}

enum Action {
    FallsAsleep,
    WakesUp,
    BeginsShift(u32),
}

struct Event {
    time: (usize, usize, usize, usize, usize),
    action: Action,
}

pub fn solve(lines: impl Iterator<Item = String>) -> u32 {
    let overall = Regex::new("^\\[(\\d+)-(\\d+)-(\\d+) (\\d+):(\\d+)\\] (.+)$").unwrap();
    let shift = Regex::new("^Guard #(\\d+) begins shift$").unwrap();

    println!("Beginning parse.");
    let mut events = lines
        .filter_map(|line| {
            if let Some(overall_cap) = overall.captures(&line) {
                let time = (
                    overall_cap[1].parse().unwrap(),
                    overall_cap[2].parse().unwrap(),
                    overall_cap[3].parse().unwrap(),
                    overall_cap[4].parse().unwrap(),
                    overall_cap[5].parse().unwrap(),
                );
                let action_raw = &overall_cap[6];
                let action = if action_raw == "wakes up" {
                    Action::WakesUp
                } else if action_raw == "falls asleep" {
                    Action::FallsAsleep
                } else if let Some(a) = shift.captures(action_raw) {
                    Action::BeginsShift(a[1].parse().expect("Couldn't parse BeginsShift"))
                } else {
                    panic!()
                };
                Some(Event { time, action })
            } else {
                None
            }
        }).collect::<Vec<_>>();
    println!("Done parsing.");

    events.sort_by_key(|e| e.time);

    let mut current_guard = if let Action::BeginsShift(id) = events[0].action {
        id
    } else {
        panic!()
    };

    let mut last_time = (0, 0, 0, 0, 0);
    let mut guard_to_minutes_to_time_asleep = HashMap::new();

    for event in events.iter().skip(1) {
        let current_action = &event.action;
        match current_action {
            Action::WakesUp => {
                for minute in between(last_time.4, event.time.4) {
                    let stats = guard_to_minutes_to_time_asleep
                        .entry(current_guard)
                        .or_insert(HashMap::new());

                    let mut entry = stats.entry(minute).or_insert(0);
                    *entry += 1;
                }
            }
            Action::FallsAsleep => {
                last_time = event.time;
            }
            Action::BeginsShift(id) => {
                current_guard = *id;
            }
        };
    }
    let mut guards_and_minutes_to_time_asleep =
        guard_to_minutes_to_time_asleep.iter().collect::<Vec<_>>();

    guards_and_minutes_to_time_asleep
        .sort_by_key(|(_, minutes_to_time_asleep)| minutes_to_time_asleep.values().sum::<usize>());

    let (guard_id, minutes_to_time_asleep) = guards_and_minutes_to_time_asleep
        .iter()
        .max_by_key(|(_, minutes_to_time_asleep)| minutes_to_time_asleep.values().max().unwrap())
        .unwrap();

    println!(
        "Guard: {:#?}, mtta: {:#?}",
        guard_id, minutes_to_time_asleep
    );
    let (sleepiest_minute, _) = minutes_to_time_asleep.iter().max_by_key(|t| t.1).unwrap();
    **guard_id * *sleepiest_minute as u32
}

fn between(start: usize, finish: usize) -> Vec<usize> {
    if start < finish {
        (start..finish).collect()
    } else {
        itertools::merge((start..60), (0..finish)).collect()
    }
}
