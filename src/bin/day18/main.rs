use std::fs;
use std::mem::swap;
use std::time::Instant;

use itertools::Itertools;
use num::Complex;

type Int = usize;
type InputType = [[Acre; N]; N];
type Acre = Complex<i8>;

const N: Int = 50;
const OPEN: Acre = Complex::new(0, 0);
const TREES: Acre = Complex::new(1, 0);
const LUMBERYARD: Acre = Complex::new(0, 1);

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day18/input.txt").unwrap();

    let mut map = [[OPEN; N]; N];

    for (y, line) in file.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map[y][x] = match c {
                '.' => continue,
                '|' => Complex::new(1, 0),
                '#' => Complex::new(0, 1),
                _ => panic!(),
            }
        }
    }

    map
}

// fn change_acre(input: &InputType, x: Int, y: Int) -> Acre {
//     let mut trees_count = 0;
//     let mut lumberyard_count = 0;
//
//     for dy in -1..=1 {
//         for dx in -1..=1 {
//             if (dy, dx) == (0, 0) {
//                 continue;
//             }
//
//             let Ok(yy) = Int::try_from(y as isize + dy) else {
//                 continue;
//             };
//
//             let Ok(xx) = Int::try_from(x as isize + dx) else {
//                 continue;
//             };
//
//             if yy >= input.len() || xx >= input[0].len() {
//                 continue;
//             }
//
//             match input[yy][xx] {
//                 AcreType::Open => continue,
//                 AcreType::Trees => trees_count += 1,
//                 AcreType::Lumberyard => lumberyard_count += 1,
//             }
//         }
//     }
//
//     let curr = input[y][x];
//
//     match curr {
//         AcreType::Open if trees_count >= 3 => AcreType::Trees,
//         AcreType::Trees if lumberyard_count >= 3 => AcreType::Lumberyard,
//         AcreType::Lumberyard if trees_count < 1 || lumberyard_count < 1 => AcreType::Open,
//         _ => curr,
//     }
// }

fn part1(mut input: InputType) -> Int {
    let mut new_input = [[OPEN; N]; N];

    for _ in 0..10 {
        for y in 0..N {
            let mut running_total = input[y][0];

            if y != 0 {
                running_total += input[y - 1][0];
            }

            if y != N - 1 {
                running_total += input[y + 1][0];
            }

            for x in 0..N {
                running_total -= input[y][x];

                if y == 0 {
                    if x > 1 {
                        running_total -= input[y][x - 2] + input[y + 1][x - 2];
                    }

                    if x < N - 1 {
                        running_total += input[y][x + 1] + input[y + 1][x + 1];
                    }
                } else if y == N - 1 {
                    if x > 1 {
                        running_total -= input[y - 1][x - 2] + input[y][x - 2];
                    }

                    if x != N - 1 {
                        running_total += input[y - 1][x + 1] + input[y][x + 1];
                    }
                } else {
                    if x > 1 {
                        running_total -=
                            input[y - 1][x - 2] + input[y][x - 2] + input[y + 1][x - 2];
                    }

                    if x != N - 1 {
                        running_total +=
                            input[y - 1][x + 1] + input[y][x + 1] + input[y + 1][x + 1];
                    }
                }

                new_input[y][x] = match input[y][x] {
                    OPEN if running_total.re >= 3 => TREES,
                    TREES if running_total.im >= 3 => LUMBERYARD,
                    LUMBERYARD if running_total.re < 1 || running_total.im < 1 => OPEN,
                    _ => input[y][x],
                };

                running_total += input[y][x];
            }
        }

        // for y in 0..N {
        //     for x in 0..N {
        //         match input[y][x] {
        //             OPEN => print!("."),
        //             TREES => print!("|"),
        //             LUMBERYARD => print!("#"),
        //             _ => panic!(),
        //         }
        //     }
        //     println!();
        // }
        // println!();
        // println!();
        //

        swap(&mut input, &mut new_input);
    }

    let counts = input.iter().flatten().counts();

    counts.get(&TREES).unwrap() * counts.get(&LUMBERYARD).unwrap()
}

fn part2(_input: InputType) -> Int {
    0
}

pub fn main() {
    let mut now = Instant::now();
    let input = read_input();
    let input_elapsed = now.elapsed();

    now = Instant::now();
    let part1 = part1(input);
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 18 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 603098);
    // assert_eq!(part2, 0);
}
