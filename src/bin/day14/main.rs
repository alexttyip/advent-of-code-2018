use itertools::Itertools;
use num::Integer;
use std::fs;
use std::time::Instant;

type Int = usize;
type InputType = Int;

const N: usize = 500_000;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day14/input.txt")
        .unwrap()
        .trim()
        .parse::<Int>()
        .unwrap()
}

fn get_digits_from_int(mut int: Int) -> Vec<Int> {
    if int == 0 {
        return vec![0];
    }

    let mut digits = Vec::new();

    while int > 0 {
        let (q, r) = int.div_mod_floor(&10);

        digits.insert(0, r);
        int = q;
    }

    digits
}

fn part1(input: InputType) -> String {
    let mut scores = Vec::with_capacity(N);
    scores.push(3);
    scores.push(7);

    let mut pointer1 = 0;
    let mut pointer2 = 1;

    while scores.len() < input + 10 {
        let score1 = scores[pointer1];
        let score2 = scores[pointer2];
        let sum = score1 + score2;

        let digits = get_digits_from_int(sum);
        scores.extend(digits);

        pointer1 = (pointer1 + score1 + 1) % scores.len();
        pointer2 = (pointer2 + score2 + 1) % scores.len();
    }

    scores[input..input + 10].iter().join("")
}

fn part2(input: InputType) -> Int {
    let input = get_digits_from_int(input);

    let mut scores = Vec::with_capacity(N);
    scores.push(3);
    scores.push(7);

    let mut pointer1 = 0;
    let mut pointer2 = 1;

    loop {
        let score1 = scores[pointer1];
        let score2 = scores[pointer2];
        let sum = score1 + score2;

        let digits = get_digits_from_int(sum);

        for digit in digits {
            scores.push(digit);

            if scores.ends_with(&input) {
                return scores.len() - input.len();
            }
        }

        pointer1 = (pointer1 + score1 + 1) % scores.len();
        pointer2 = (pointer2 + score2 + 1) % scores.len();
    }
}

pub fn main() {
    let input = read_input();

    let mut now = Instant::now();
    let part1 = part1(input);
    let part1_elapsed = now.elapsed();
    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 14 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    // assert_eq!(part1, 0);
    // assert_eq!(part2, 0);
}
