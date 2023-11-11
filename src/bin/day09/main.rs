use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

type Int = usize;
type InputType = (Int, Int);

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day09/input.txt")
        .unwrap()
        .trim()
        .lines()
        .flat_map(|s| -> Option<(Int, Int)> {
            let mut words = s.split_whitespace();

            Some((words.next()?.parse().ok()?, words.nth(5)?.parse().ok()?))
        })
        .next()
        .unwrap()
}

fn play((n_players, n_marbles): InputType) -> Int {
    let mut players = vec![Vec::<Int>::new(); n_players];
    let mut circle = VecDeque::<Int>::with_capacity(n_marbles);

    circle.push_back(0);

    for round in 1..=n_marbles {
        if round % 23 == 0 {
            circle.rotate_right(7);

            let curr_player = players.get_mut(round % n_players).unwrap();

            curr_player.push(round);
            curr_player.push(circle.pop_front().unwrap());

            continue;
        }

        circle.rotate_left(2 % circle.len());
        circle.push_front(round);
    }

    players
        .iter()
        .map(|marbles| marbles.iter().sum())
        .max()
        .unwrap()
}
fn part1(input: InputType) -> Int {
    play(input)
}

fn part2(input: InputType) -> Int {
    play((input.0, input.1 * 100))
}

pub fn main() {
    let input = read_input();

    let mut now = Instant::now();
    let part1 = part1(input);
    let part1_elapsed = now.elapsed();
    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 09 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 371284);
    assert_eq!(part2, 3038972494);
}
