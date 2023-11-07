use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Int = u32;
type InputType = BTreeMap<char, BTreeSet<char>>;

fn read_input() -> InputType {
    let pairs: Vec<(char, char)> = fs::read_to_string("./src/bin/day07/input.txt")
        .unwrap()
        .trim()
        .lines()
        .flat_map(|line| {
            let split: Vec<_> = line.split_whitespace().collect();
            Some((split.get(1)?.chars().next()?, split.get(7)?.chars().next()?))
        })
        .collect();

    let mut map: InputType = BTreeMap::from_iter(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .map(|c| (c, BTreeSet::new())),
    );

    for &(from, to) in &pairs {
        map.entry(to).and_modify(|set| {
            set.insert(from);
        });
    }

    map
}

fn get_first_available_step(map: &InputType) -> Option<char> {
    map.iter()
        .find_or_first(|(_, v)| v.is_empty())
        .map(|(k, _)| *k)
}

fn get_steps(mut map: InputType, mut seen: Vec<char>) -> Vec<char> {
    let Some(curr) = get_first_available_step(&map) else {
        return seen;
    };

    map.remove(&curr);

    if seen.contains(&curr) {
        return get_steps(map, seen);
    }

    seen.push(curr);

    map.iter_mut().for_each(|(_, v)| {
        v.retain(|&v| v != curr);
    });

    get_steps(map, seen)
}

fn part1(input: InputType) -> String {
    get_steps(input, vec![]).iter().collect()
}

const NUM_WORKERS: usize = 5;

fn get_available_steps(map: &InputType) -> Vec<char> {
    map.iter()
        .filter(|(_, v)| v.is_empty())
        .map(|(k, _)| *k)
        .collect()
}

fn part2(mut input: InputType) -> Int {
    let mut done_jobs = HashSet::with_capacity(30);

    let mut workers = [('¡', 0); NUM_WORKERS];

    for i in 0.. {
        // for j in 0..NUM_WORKERS {
        for (job, job_until) in workers.into_iter() {
            // let (job, job_until) = workers[j];

            if done_jobs.contains(&job) {
                continue;
            };

            if job_until <= i {
                done_jobs.insert(job);

                input.iter_mut().for_each(|(_, v)| {
                    v.retain(|&v| v != job);
                });
            }
        }

        for job in get_available_steps(&input) {
            for worker in workers.iter_mut() {
                if worker.1 > i {
                    continue;
                }

                *worker = (job, job as Int - 4 + i);
                input.remove(&job);

                break;
            }
        }

        if input.is_empty() && workers.iter().all(|(_, job_until)| job_until <= &i) {
            return i;
        }
    }

    panic!()
}

pub fn main() {
    let input = read_input();

    let mut now = Instant::now();
    let part1 = part1(input.clone());
    let part1_elapsed = now.elapsed();
    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 07 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, "JDEKPFABTUHOQSXVYMLZCNIGRW");
    assert_eq!(part2, 1048);
}
