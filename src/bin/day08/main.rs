use std::collections::{BTreeMap, VecDeque};
use std::fs;
use std::time::Instant;

type Int = usize;
type InputType = VecDeque<Int>;

#[derive(Debug)]
struct Node {
    children: Vec<Int>,
    metadata: Vec<Int>,
}

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day08/input.txt")
        .unwrap()
        .split_whitespace()
        .flat_map(|s| s.parse::<Int>())
        .collect()
}

fn process(
    input: &mut InputType,
    mut tree: BTreeMap<Int, Node>,
    curr_idx: Int,
) -> BTreeMap<Int, Node> {
    let Some(n_nodes) = input.pop_front() else {
        return tree;
    };
    let Some(n_metadata) = input.pop_front() else {
        return tree;
    };

    if n_nodes == 0 {
        return BTreeMap::from([(
            curr_idx,
            Node {
                children: Vec::new(),
                metadata: input.drain(0..n_metadata).collect(),
            },
        )]);
    }

    let mut children_idx = vec![];

    for _ in 0..n_nodes {
        let child_idx = curr_idx + 1 + tree.len();
        let result = process(input, BTreeMap::new(), child_idx);

        tree.extend(result);
        children_idx.push(child_idx);
    }

    tree.insert(
        curr_idx,
        Node {
            children: children_idx,
            metadata: input.drain(0..n_metadata).collect(),
        },
    );

    tree
}

fn part1(mut input: InputType) -> Int {
    let tree = process(&mut input, BTreeMap::new(), 0);

    tree.iter()
        .fold(0, |acc, curr| acc + curr.1.metadata.iter().sum::<Int>())
}

fn calc_part2(tree: &BTreeMap<Int, Node>, curr_idx: &Int) -> Int {
    let Some(Node { children, metadata }) = tree.get(curr_idx) else {
        panic!();
    };

    if children.is_empty() {
        return metadata.iter().sum();
    }

    metadata
        .iter()
        .flat_map(|i| children.get(i - 1))
        .map(|i| calc_part2(tree, i))
        .sum()
}

fn part2(mut input: InputType) -> Int {
    let tree = process(&mut input, BTreeMap::new(), 0);

    calc_part2(&tree, &0)
}

pub fn main() {
    let input = read_input();

    let mut now = Instant::now();
    let part1 = part1(input.clone());
    let part1_elapsed = now.elapsed();
    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 08 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 36566);
    assert_eq!(part2, 30548);
}
