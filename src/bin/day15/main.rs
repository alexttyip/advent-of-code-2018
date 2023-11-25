use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::fs;
use std::num::TryFromIntError;
use std::time::Instant;

type Int = usize;
type InputType = (BTreeMap<Coord, BTreeSet<Coord>>, BTreeMap<Coord, Unit>);
type Coord = (Int, Int);

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct Unit {
    unit_type: UnitType,
    hp: Int,
}

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day15/input.txt").unwrap();

    let mut map = Vec::<Vec<char>>::new();

    for line in file.trim().lines() {
        let mut row = vec![];

        for c in line.chars() {
            row.push(c);
        }

        map.push(row);
    }

    let mut cavern = BTreeMap::<Coord, BTreeSet<Coord>>::new();
    let mut units = BTreeMap::<Coord, Unit>::new();

    for y in 0..map.len() {
        for (x, c) in map[y].iter().enumerate() {
            let mut neighbours = BTreeSet::new();

            match c {
                '.' => {}
                'E' => {
                    units.insert(
                        (y, x),
                        Unit {
                            unit_type: UnitType::Elf,
                            hp: 200,
                        },
                    );
                }
                'G' => {
                    units.insert(
                        (y, x),
                        Unit {
                            unit_type: UnitType::Goblin,
                            hp: 200,
                        },
                    );
                }
                _ => continue,
            };

            for delta in [(1, 0), (0, 1), (-1, 0), (0isize, -1isize)] {
                let Ok((yy, xx)) = move_coord(&(y, x), delta) else {
                    continue;
                };

                let Some(neighbour) = map.get(xx).and_then(|row| row.get(yy)) else {
                    continue;
                };

                if neighbour == &'#' {
                    continue;
                }

                neighbours.insert((yy, xx));
            }

            cavern.insert((y, x), neighbours);
        }
    }

    (cavern, units)
}

fn move_coord(&(y, x): &Coord, (dx, dy): (isize, isize)) -> Result<Coord, TryFromIntError> {
    usize::try_from(x as isize + dx).and_then(|xx| Ok((usize::try_from(y as isize + dy)?, xx)))
}

fn get_new_location(
    &coord: &Coord,
    map: &BTreeMap<Coord, BTreeSet<Coord>>,
    units: &BTreeMap<Coord, Unit>,
    &target_type: &UnitType,
) -> Option<Coord> {
    let mut dist = HashMap::from([(coord, 0)]);
    let mut pred = HashMap::new();
    let mut queue = VecDeque::<Coord>::from([coord]);

    while let Some(curr) = queue.pop_front() {
        for delta in [(0isize, -1isize), (-1, 0), (1, 0), (0, 1)] {
            let Ok((yy, xx)) = move_coord(&curr, delta) else {
                continue;
            };

            let neighbour = (yy, xx);
            if dist.contains_key(&neighbour) || !map.contains_key(&neighbour) {
                continue;
            }

            dist.insert(neighbour, *dist.get(&curr).unwrap() + 1);
            pred.insert(neighbour, curr);

            match units.get(&neighbour) {
                Some(neighbour_unit) if neighbour_unit.unit_type == target_type => {
                    let mut a = curr;
                    let mut b = curr;

                    while let Some(&predecessor) = pred.get(&a) {
                        b = a;
                        a = predecessor;
                    }

                    return Some(b);
                }
                Some(_) => {}
                None => {
                    queue.push_back(neighbour);
                }
            }
        }
    }

    None
}

fn simulate((map, mut units): InputType, elf_attack: Int, part1: bool) -> Option<Int> {
    for round in 0.. {
        for coord in units.clone().keys() {
            let target_type = match units.get(coord).map(|unit| unit.unit_type) {
                Some(UnitType::Elf) => UnitType::Goblin,
                Some(UnitType::Goblin) => UnitType::Elf,
                _ => continue,
            };

            let coord = get_new_location(coord, &map, &units, &target_type)
                .map(|new_coord| {
                    let unit_type = units.remove(coord).unwrap();
                    units.insert(new_coord, unit_type);

                    new_coord
                })
                .unwrap_or(*coord);

            // Attack
            let mut min_target_hp = Int::MAX;
            let mut target_coord = None;

            for delta in [(0isize, -1isize), (-1, 0), (1, 0), (0, 1)] {
                let Ok(neighbour_coord) = move_coord(&coord, delta) else {
                    continue;
                };

                if let Some(neighbour) = units.get(&neighbour_coord) {
                    if neighbour.unit_type == target_type && neighbour.hp < min_target_hp {
                        target_coord = Some(neighbour_coord);
                        min_target_hp = neighbour.hp;
                    }
                }
            }

            if let Some(target_coord) = target_coord {
                let mut target = units.remove(&target_coord).unwrap();

                let attack = if target_type == UnitType::Goblin {
                    elf_attack
                } else {
                    3
                };

                if target.hp > attack {
                    target.hp -= attack;
                    units.insert(target_coord, target);
                } else if target.unit_type == UnitType::Elf && !part1 {
                    return None;
                }
            }
        }

        if units.values().map(|unit| unit.unit_type).all_equal() {
            dbg!(&round, &units.values().map(|b| b.hp).sum::<Int>());
            return Some(round * units.values().map(|b| b.hp).sum::<Int>());
        }
    }

    panic!()
}

fn part1(input: InputType) -> Int {
    simulate(input.clone(), 3, true).unwrap()
}

fn part2(input: InputType) -> Int {
    for elf_attack in 4.. {
        if let Some(ans) = simulate(input.clone(), elf_attack, false) {
            return ans;
        }
    }

    0
}

pub fn main() {
    let mut now = Instant::now();
    let input = read_input();
    let input_elapsed = now.elapsed();

    now = Instant::now();
    let part1 = part1(input.clone());
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 15 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 198744);
    assert_ne!(part2, 61572);
    assert_ne!(part2, 63038);
    assert_ne!(part2, 64504);
    // assert_eq!(part2, 0);
}
