use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use regex::Regex;
use std::fs;
use std::time::Instant;

type Int = i32;
type Point = (Int, Int, Int, Int);
type InputType = Vec<Point>;

fn read_input() -> InputType {
    let re = Regex::new(r"^position=<(.+),(.+)> velocity=<(.+),(.+)>$").unwrap();

    fs::read_to_string("./src/bin/day10/input.txt")
        .unwrap()
        .trim()
        .lines()
        .flat_map(|line| re.captures(line))
        .flat_map(|cap| {
            cap.iter()
                .skip(1)
                .take(4)
                .flat_map(|s| s?.as_str().trim().parse::<Int>().ok())
                .collect_tuple::<Point>()
        })
        .collect()
}

fn part1(input: InputType) -> Int {
    for i in 0.. {
        let new_points: Vec<(Int, Int)> = input
            .iter()
            .map(|(x, y, dx, dy)| (x + dx * i, y + dy * i))
            .collect();

        let MinMax(min_x, max_x) = new_points.iter().map(|point| point.0).minmax() else {
            panic!();
        };
        let MinMax(min_y, max_y) = new_points.iter().map(|point| point.1).minmax() else {
            panic!();
        };

        if max_y - min_y < 10 {
            for y in min_y..=max_y {
                'outer: for x in min_x..=max_x {
                    for &(xx, yy) in &new_points {
                        if x == xx && y == yy {
                            print!("#");
                            continue 'outer;
                        }
                    }
                    print!(".");
                }
                println!();
            }
            break;
        }
    }

    0
}

fn part2(input: InputType) -> Int {
    for i in 0.. {
        let new_points: Vec<(Int, Int)> = input
            .iter()
            .map(|(x, y, dx, dy)| (x + dx * i, y + dy * i))
            .collect();

        let MinMax(min_y, max_y) = new_points.iter().map(|point| point.1).minmax() else {
            panic!();
        };

        if max_y - min_y < 10 {
            return i;
        }
    }

    panic!();
}

pub fn main() {
    let input = read_input();

    let mut now = Instant::now();
    let part1 = part1(input.clone());
    let part1_elapsed = now.elapsed();
    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 10 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    // assert_eq!(part1, 0);
    assert_eq!(part2, 10101);
}
