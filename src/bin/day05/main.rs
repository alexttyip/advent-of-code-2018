use std::fs;
use std::time::Instant;

type Int = usize;
type InputType = Vec<char>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day05/input.txt")
        .unwrap()
        .trim()
        .lines()
        .flat_map(str::chars)
        .collect()
}

fn part1(input: InputType) -> Int {
    let mut answer: Vec<char> = Vec::with_capacity(input.len());
    let mut index_to_check = 0;

    while index_to_check < input.len() {
        let Some(lhs) = answer.last() else {
            answer.push(*input.get(index_to_check).unwrap());
            index_to_check += 1;
            continue;
        };

        let rhs = input.get(index_to_check).unwrap();

        if lhs != rhs && lhs.eq_ignore_ascii_case(rhs) {
            answer.pop();
            index_to_check += 1;
            continue;
        }

        answer.push(*rhs);
        index_to_check += 1;
    }

    answer.len()
}

fn part2(input: InputType) -> Int {
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .fold(Int::MAX, |acc, curr| {
            part1(
                input
                    .iter()
                    .filter(|c| !c.eq_ignore_ascii_case(&curr))
                    .copied()
                    .collect(),
            )
            .min(acc)
        })
}

pub fn main() {
    let input = read_input();

    let mut now = Instant::now();
    let part1 = part1(input.clone());
    let part1_elapsed = now.elapsed();
    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 05 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 9704);
    assert_eq!(part2, 6942);
}
