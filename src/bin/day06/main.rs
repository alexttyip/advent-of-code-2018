use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Int = i32;
type InputType = (Vec<(Int, Int)>, Int);

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day06/input.txt").unwrap();

    let mut coords = vec![];
    let mut max = 0;

    for line in file.trim().lines() {
        let Some((x, y)) = line
            .split(',')
            .flat_map(|n| n.trim().parse::<Int>())
            .collect_tuple()
        else {
            continue;
        };

        max = max.max(x).max(y);
        coords.push((x, y))
    }

    (coords, max)
}

fn part1((input, max): InputType) -> i32 {
    let mut counts = HashMap::new();
    let mut excluded = HashSet::new();

    for xx in 0..max {
        for yy in 0..max {
            let (min_coord, _) =
                input
                    .iter()
                    .fold((None, Int::MAX), |(min_coord, min_dist), (x, y)| {
                        let dist = (xx as Int - x).abs() + (yy as Int - y).abs();

                        match dist.cmp(&min_dist) {
                            Ordering::Less => (Some((x, y)), dist),
                            Ordering::Equal => (None, dist),
                            Ordering::Greater => (min_coord, min_dist),
                        }
                    });

            let Some(min_coord) = min_coord else {
                continue;
            };

            if xx == 0 || xx == max - 1 || yy == 0 || yy == max - 1 {
                excluded.insert(min_coord);
            }

            counts
                .entry(min_coord)
                .and_modify(|acc| *acc += 1)
                .or_insert(1);
        }
    }

    *counts
        .iter()
        .filter(|(key, _)| !excluded.contains(key))
        .max_by(|(_, dist1), (_, dist2)| dist1.cmp(dist2))
        .map(|(_, dist)| dist)
        .unwrap()
}

fn part2((input, max): InputType) -> Int {
    let max = max as usize;
    let mut x_distances = vec![0; max];
    let mut y_distances = vec![0; max];

    for (x, y) in input {
        for i in 0..max {
            *x_distances.get_mut(i).unwrap() += (i as Int - x).abs();
            *y_distances.get_mut(i).unwrap() += (i as Int - y).abs();
        }
    }

    let mut count = 0;
    for x in x_distances {
        for y in y_distances.clone() {
            if x + y < 10_000 {
                count += 1;
            }
        }
    }

    count
}

pub fn main() {
    let input = read_input();

    let mut now = Instant::now();
    let part1 = part1(input.clone());
    let part1_elapsed = now.elapsed();
    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 06 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 4166);
    assert_eq!(part2, 42250);
}
