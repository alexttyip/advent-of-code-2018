use std::fs;
use std::time::Instant;

type Int = i64;
const N: usize = 300;
type InputType = [[Int; N]; N];

fn read_input() -> InputType {
    let serial: Int = fs::read_to_string("./src/bin/day11/input.txt")
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let mut grid = [[0; N]; N];

    for (x, col) in grid.iter_mut().enumerate() {
        for (y, cell) in col.iter_mut().enumerate() {
            let rack_id = (x as Int + 1) + 10;
            *cell = rack_id * (y as Int + 1);
            *cell += serial;
            *cell *= rack_id;
            *cell /= 100;
            *cell %= 10;
            *cell -= 5;
        }
    }

    grid
}

fn do_sums(input: &InputType, part1: bool) -> (usize, usize, usize) {
    let mut max_value = Int::MIN;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    let mut sums = *input;

    for z in 1..=N {
        if part1 && z > 3 {
            return (max_x, max_y, max_z);
        }

        for x in 0..N - (z - 1) {
            for y in 0..N - (z - 1) {
                if x == 0 && y == 0 {
                    if z == 1 {
                        continue;
                    }

                    for zz in 0..z {
                        sums[0][0] += input[zz][z - 1];
                        sums[0][0] += input[z - 1][zz];
                    }

                    sums[0][0] -= input[z - 1][z - 1];

                    continue;
                }

                let sum = if x == 0 {
                    let mut sum = sums[x][y - 1];

                    for zz in 0..z {
                        sum -= input[x + zz][y - 1];
                        sum += input[x + zz][y + z - 1];
                    }

                    sum
                } else {
                    let mut sum = sums[x - 1][y];

                    for zz in 0..z {
                        sum -= input[x - 1][y + zz];
                        sum += input[x + z - 1][y + zz];
                    }

                    sum
                };

                if sum > max_value && (z == 3 || !part1) {
                    max_value = sum;
                    max_x = x;
                    max_y = y;
                    max_z = z;
                }

                sums[x][y] = sum;
            }
        }
    }

    (max_x, max_y, max_z)
}

fn part1(input: InputType) -> (usize, usize) {
    let (x, y, _) = do_sums(&input, true);

    (x + 1, y + 1)
}

fn part2(input: InputType) -> (usize, usize, usize) {
    let (x, y, z) = do_sums(&input, false);

    (x + 1, y + 1, z)
}

pub fn main() {
    let input = read_input();

    let mut now = Instant::now();
    let part1 = part1(input);
    let part1_elapsed = now.elapsed();
    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 11 ---");
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, (243, 34));
    assert_eq!(part2, (90, 214, 15));
}
