use std::collections::BTreeMap;
use std::fs;
use std::mem::take;
use std::time::Instant;

type Int = usize;
type InputType = (BTreeMap<Coord, Cart>, [[Option<RailSegment>; N]; N]);

const N: usize = 150;

type Coord = (Int, Int);

#[derive(Debug, Copy, Clone, PartialEq)]
enum RailSegment {
    Curve,
    BackCurve,
    Vertical,
    Horizontal,
    Intersection,
}

#[derive(Copy, Clone)]
enum CartDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone)]
enum TurnDirection {
    Left,
    Straight,
    Right,
}

impl TurnDirection {
    fn increment(&self) -> TurnDirection {
        match self {
            TurnDirection::Left => TurnDirection::Straight,
            TurnDirection::Straight => TurnDirection::Right,
            TurnDirection::Right => TurnDirection::Left,
        }
    }
}

impl CartDirection {
    fn turn(&self, turn_direction: TurnDirection) -> CartDirection {
        match (self, turn_direction) {
            (CartDirection::Up, TurnDirection::Left) => CartDirection::Left,
            (CartDirection::Up, TurnDirection::Right) => CartDirection::Right,
            (CartDirection::Right, TurnDirection::Left) => CartDirection::Up,
            (CartDirection::Right, TurnDirection::Right) => CartDirection::Down,
            (CartDirection::Down, TurnDirection::Left) => CartDirection::Right,
            (CartDirection::Down, TurnDirection::Right) => CartDirection::Left,
            (CartDirection::Left, TurnDirection::Left) => CartDirection::Down,
            (CartDirection::Left, TurnDirection::Right) => CartDirection::Up,
            (_, TurnDirection::Straight) => *self,
        }
    }
}

#[derive(Copy, Clone)]
struct Cart {
    direction: CartDirection,
    next_turn: TurnDirection,
}

fn read_input() -> InputType {
    let mut carts = BTreeMap::<Coord, Cart>::new();

    let mut rails = [[None; N]; N];

    let file = fs::read_to_string("./src/bin/day13/input.txt").unwrap();

    for (y, line) in file.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            rails[x][y] = match c {
                '|' => Some(RailSegment::Vertical),
                '-' => Some(RailSegment::Horizontal),
                '/' => Some(RailSegment::Curve),
                '\\' => Some(RailSegment::BackCurve),
                '+' => Some(RailSegment::Intersection),

                '^' => {
                    carts.insert(
                        (y, x),
                        Cart {
                            direction: CartDirection::Up,
                            next_turn: TurnDirection::Left,
                        },
                    );
                    Some(RailSegment::Vertical)
                }
                '>' => {
                    carts.insert(
                        (y, x),
                        Cart {
                            direction: CartDirection::Right,
                            next_turn: TurnDirection::Left,
                        },
                    );
                    Some(RailSegment::Horizontal)
                }
                'v' => {
                    carts.insert(
                        (y, x),
                        Cart {
                            direction: CartDirection::Down,
                            next_turn: TurnDirection::Left,
                        },
                    );
                    Some(RailSegment::Vertical)
                }
                '<' => {
                    carts.insert(
                        (y, x),
                        Cart {
                            direction: CartDirection::Left,
                            next_turn: TurnDirection::Left,
                        },
                    );
                    Some(RailSegment::Horizontal)
                }

                _ => None,
            }
        }
    }

    (carts, rails)
}

fn get_new_coord(y: usize, x: usize, direction: CartDirection) -> (usize, usize) {
    match direction {
        CartDirection::Up => (y - 1, x),
        CartDirection::Right => (y, x + 1),
        CartDirection::Down => (y + 1, x),
        CartDirection::Left => (y, x - 1),
    }
}

fn get_resulting_direction(
    Cart {
        direction,
        next_turn,
    }: Cart,
    rail_segment: RailSegment,
) -> CartDirection {
    match rail_segment {
        RailSegment::Vertical | RailSegment::Horizontal => direction,
        RailSegment::Curve => match direction {
            CartDirection::Up => CartDirection::Right,
            CartDirection::Right => CartDirection::Up,
            CartDirection::Down => CartDirection::Left,
            CartDirection::Left => CartDirection::Down,
        },
        RailSegment::BackCurve => match direction {
            CartDirection::Up => CartDirection::Left,
            CartDirection::Right => CartDirection::Down,
            CartDirection::Down => CartDirection::Right,
            CartDirection::Left => CartDirection::Up,
        },
        RailSegment::Intersection => direction.turn(next_turn),
    }
}

fn simulate((mut carts, rails): InputType, part1: bool) -> (usize, usize) {
    let mut new_carts = BTreeMap::new();

    while carts.len() > 1 {
        while let Some(((y, x), cart)) = carts.pop_first() {
            let new_coord = get_new_coord(y, x, cart.direction);

            if carts.remove(&new_coord).is_some() || new_carts.remove(&new_coord).is_some() {
                if part1 {
                    return (new_coord.1, new_coord.0);
                }

                continue;
            }

            let rail_segment =
                rails[new_coord.1][new_coord.0].expect("cart should always move to a rail segment");

            let direction = get_resulting_direction(cart, rail_segment);
            let next_turn = if rail_segment == RailSegment::Intersection {
                cart.next_turn.increment()
            } else {
                cart.next_turn
            };

            new_carts.insert(
                new_coord,
                Cart {
                    direction,
                    next_turn,
                },
            );
        }

        carts = take(&mut new_carts);
    }

    let ((y, x), _) = carts.pop_first().expect("there should be one last cart");

    (x, y)
}

fn part1(input: InputType) -> (Int, Int) {
    simulate(input, true)
}

fn part2(input: InputType) -> (Int, Int) {
    simulate(input, false)
}

pub fn main() {
    let input = read_input();

    let mut now = Instant::now();
    let part1 = part1(input.clone());
    let part1_elapsed = now.elapsed();
    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 13 ---");
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, (74, 87));
    assert_eq!(part2, (29, 74));
}
