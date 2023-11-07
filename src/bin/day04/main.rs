use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::num::ParseIntError;
use std::time::Instant;

type Int = u64;
type InputType = HashMap<u16, [u16; 60]>;

fn read_input() -> InputType {
    let re = Regex::new(r"^\[.+ \d+:(\d+)\] (.+)$").unwrap();

    fs::read_to_string("./src/bin/day04/input.txt")
        .unwrap()
        .trim()
        .lines()
        .sorted_unstable()
        .flat_map(|line| re.captures(line))
        .flat_map(|cap| -> Result<(u8, String), ParseIntError> {
            Ok((cap[1].parse()?, cap[2].to_string()))
        })
        .fold(
            (HashMap::new() as InputType, 0, 0),
            |(mut guards, mut curr_guard, mut curr_time_start), (minute, message)| {
                match message.as_str() {
                    "falls asleep" => curr_time_start = minute,
                    "wakes up" => {
                        guards.entry(curr_guard).and_modify(|line| {
                            for i in curr_time_start..minute {
                                line[usize::from(i)] += 1
                            }
                        });
                    }
                    _ => {
                        curr_guard = message.split_whitespace().nth(1).unwrap()[1..]
                            .parse()
                            .unwrap();

                        guards.entry(curr_guard).or_insert([0; 60]);
                    }
                }

                (guards, curr_guard, curr_time_start)
            },
        )
        .0
}

fn part1(input: InputType) -> Int {
    let mut guard_id = 0;
    let mut max_total_sleep = 0;
    let mut asleep_most_minute = 0;

    for (guard, sleep) in input {
        let guard_total_sleep = sleep.iter().sum();

        if guard_total_sleep > max_total_sleep {
            guard_id = guard;
            max_total_sleep = guard_total_sleep;
            asleep_most_minute = sleep.iter().position_max().unwrap();
        }
    }

    guard_id as Int * asleep_most_minute as Int
}

fn part2(input: InputType) -> Int {
    let mut guard_id = 0;
    let mut peak_minute = 0;
    let mut peak_minute_freq = 0;

    for (guard, sleep) in input {
        let guard_peak_minute = sleep.iter().position_max().unwrap();

        if sleep[guard_peak_minute] > peak_minute_freq {
            guard_id = guard;
            peak_minute = guard_peak_minute;
            peak_minute_freq = sleep[guard_peak_minute];
        }
    }

    guard_id as Int * peak_minute as Int
}

pub fn main() {
    let input = read_input();

    let mut now = Instant::now();
    let part1 = part1(input.clone());
    let part1_elapsed = now.elapsed();
    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 04 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 14346);
    assert_eq!(part2, 5705);
}
