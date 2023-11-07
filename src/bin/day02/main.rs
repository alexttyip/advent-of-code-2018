use std::fs;
use std::time::Instant;

use itertools::Itertools;

type Int = u16;
type InputType = Vec<String>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day02/input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(str::to_string)
        .collect()
}

fn part1(input: InputType) -> Int {
    let (twos, threes) = input.iter().fold((0, 0), |(twos, threes), s| {
        let counts: Vec<(usize, char)> = s.chars().sorted_unstable().dedup_with_count().collect();

        (
            twos + Int::from(counts.iter().any(|(count, _)| count == &2)),
            threes + Int::from(counts.iter().any(|(count, _)| count == &3)),
        )
    });

    twos * threes
}

fn part2(input: InputType) -> String {
    let mut answer = String::with_capacity(input.first().unwrap().capacity());

    for i in 0..input.len() {
        'outer: for j in (i + 1)..input.len() {
            let x = input.get(i).unwrap();
            let y = input.get(j).unwrap();

            let mut difference = 0;

            answer.clear();

            for (xx, yy) in x.chars().zip_eq(y.chars()) {
                if xx == yy {
                    answer.push(xx);
                    continue;
                }

                difference += 1;

                if difference > 1 {
                    continue 'outer;
                }
            }

            return answer;
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

    println!("--- Day 02 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 7872);
    assert_eq!(part2, "tjxmoewpdkyaihvrndfluwbzc");
}
