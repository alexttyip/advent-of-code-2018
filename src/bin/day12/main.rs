use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Int = i64;
type InputType = (Vec<bool>, [bool; NUM_MASKS]);

const NUM_MASKS: usize = 2usize.pow(5);

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day12/input.txt").unwrap();
    let mut lines = file.trim().lines();

    let state = lines
        .next()
        .and_then(|line| line.split_whitespace().nth_back(0))
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect_vec();

    let mut masks = [false; NUM_MASKS];

    for mask in lines.skip(1) {
        let (criteria, _, result) = mask
            .split_whitespace()
            .collect_tuple::<(&str, &str, &str)>()
            .unwrap();

        let idx = criteria
            .chars()
            .fold(0, |acc, c| (acc << 1) + usize::from(c == '#'));

        masks[idx] = result == "#";
    }

    (state, masks)
}

fn part1((mut state, masks): InputType) -> Int {
    let mut new_state = Vec::with_capacity(state.len());

    const N: Int = 20;

    for _ in 0..N {
        let state_iter = [false; 4].iter().chain(state.iter()).chain([&false; 4]);

        for window in state_iter.tuple_windows::<(_, _, _, _, _)>() {
            let idx = 16 * usize::from(*window.0)
                + 8 * usize::from(*window.1)
                + 4 * usize::from(*window.2)
                + 2 * usize::from(*window.3)
                + usize::from(*window.4);

            new_state.push(masks[idx]);
        }

        state = new_state.clone();
        new_state.clear();
    }

    state.iter().enumerate().fold(
        0,
        |acc, (i, curr)| {
            if *curr {
                acc + i as Int - N * 2
            } else {
                acc
            }
        },
    )
}

fn part2(_input: InputType) -> Int {
    0
}

pub fn main() {
    let input = read_input();

    let mut now = Instant::now();
    let part1 = part1(input.clone());
    let part1_elapsed = now.elapsed();
    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 12 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 3605);
    // assert_eq!(part2, 0);
}
