use std::fs;
use std::time::Instant;

type Int = i32;
type InputType = Vec<Int>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day01/input.txt")
        .unwrap()
        .trim()
        .lines()
        .flat_map(|s| s.parse::<Int>())
        .collect()
}

fn part1(input: InputType) -> Int {
    input.iter().sum()
}

const MAX_VALUE: usize = 1_000_000;

fn part2(input: InputType) -> Int {
    let mut current = MAX_VALUE as Int;
    let mut history = [false; MAX_VALUE * 2];

    for i in input.iter().cycle() {
        current += i;

        if history[current as usize] {
            return current - MAX_VALUE as Int;
        }

        history[current as usize] = true;
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

    println!("--- Day 01 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 516);
    assert_eq!(part2, 71892);
}
