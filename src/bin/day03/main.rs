use std::collections::{HashMap, HashSet};
use std::fs;
use std::num::ParseIntError;
use std::time::Instant;

use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Debug)]
struct Claim {
    id: Int,
    x: Int,
    y: Int,
    dx: Int,
    dy: Int,
}

type Int = usize;
type InputType = Vec<Claim>;

fn read_input() -> InputType {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    fs::read_to_string("./src/bin/day03/input.txt")
        .unwrap()
        .trim()
        .lines()
        .flat_map(|line| re.captures(line))
        .flat_map(|cap| -> Result<Claim, ParseIntError> {
            Ok(Claim {
                id: cap[1].parse()?,
                x: cap[2].parse()?,
                y: cap[3].parse()?,
                dx: cap[4].parse()?,
                dy: cap[5].parse()?,
            })
        })
        .collect()
}

fn part1(input: InputType) -> Int {
    let mut fabric: HashMap<(usize, usize), usize> = HashMap::with_capacity(500_000);

    for claim in input {
        for x in claim.x..(claim.x + claim.dx) {
            for y in claim.y..(claim.y + claim.dy) {
                fabric
                    .entry((x, y))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }

    fabric.values().filter(|&&value| value >= 2).count()
}

fn part2(input: InputType) -> usize {
    let mut fabric: HashMap<(usize, usize), usize> = HashMap::with_capacity(500_000);
    let mut isolated: HashSet<usize> = HashSet::with_capacity(input.len());

    for claim in &input {
        isolated.insert(claim.id);

        for x in claim.x..(claim.x + claim.dx) {
            for y in claim.y..(claim.y + claim.dy) {
                if let Some(&id) = fabric.get(&(x, y)) {
                    isolated.remove(&id);
                    isolated.remove(&claim.id);
                    continue;
                }

                fabric.insert((x, y), claim.id);
            }
        }
    }

    *isolated.iter().exactly_one().unwrap()
}

pub fn main() {
    let input = read_input();

    let mut now = Instant::now();
    let part1 = part1(input.clone());
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 03 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 101565);
    assert_eq!(part2, 656);
}
