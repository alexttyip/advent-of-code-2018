use std::fs;
use std::iter::repeat;
use std::time::Instant;

use itertools::Itertools;
use regex::Regex;

pub use crate::run_instruction::day16::Int;
use crate::run_instruction::day16::{run_instruction, Opcode, OPCODES};

mod run_instruction;

type InputType = (Vec<Sample>, Vec<[Int; 4]>);

#[derive(Debug, Copy, Clone)]
struct Sample {
    before: [Int; 4],
    instruction: [Int; 4],
    after: [Int; 4],
}

fn state_to_array(s: &str) -> [Int; 4] {
    s.replace(',', "")
        .split_whitespace()
        .filter_map(|s| s.parse::<Int>().ok())
        .collect_vec()
        .try_into()
        .unwrap()
}

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day16/input.txt").unwrap();

    let (sample_str, instructions_str) = file.split("\n\n\n\n").collect_tuple().unwrap();

    let re = Regex::new(
        r"Before:\s+\[(?P<before>[\d,\s]+)\]\n(?P<instruction>[\d\s]+)\nAfter:\s+\[(?P<after>[\d,\s]+)\]",
    )
    .unwrap();

    let samples = re
        .captures_iter(sample_str)
        .map(|caps| {
            let before = caps.name("before").unwrap().as_str();
            let instruction = caps.name("instruction").unwrap().as_str();
            let after = caps.name("after").unwrap().as_str();

            Sample {
                before: state_to_array(before),
                instruction: state_to_array(instruction),
                after: state_to_array(after),
            }
        })
        .collect();

    let instructions = instructions_str
        .trim()
        .lines()
        .filter_map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<Int>().ok())
                .collect_vec()
                .try_into()
                .ok()
        })
        .collect_vec();

    (samples, instructions)
}

fn deduce_opcode_name(input: Vec<Sample>) -> [Opcode; 16] {
    let mut possible_opcodes = repeat(Vec::from(OPCODES)).take(16).collect_vec();

    for sample in input {
        let mut incorrect_opcodes = Vec::<Opcode>::with_capacity(16);
        for opcode in OPCODES {
            let result = run_instruction(sample.before, &opcode, sample.instruction);

            if result != sample.after {
                incorrect_opcodes.push(opcode);
            }
        }

        possible_opcodes[sample.instruction[0]].retain(|v| !incorrect_opcodes.contains(v));
    }

    let mut to_remove = Vec::<Opcode>::with_capacity(16);

    while possible_opcodes.iter().any(|vec| vec.len() > 1) {
        for x in possible_opcodes.iter_mut() {
            if x.len() > 1 {
                x.retain(|v| !to_remove.contains(v));
            }

            if x.len() == 1 {
                to_remove.push(x[0]);
            }
        }
    }

    possible_opcodes
        .iter()
        .map(|v| v[0])
        .collect_vec()
        .try_into()
        .unwrap()
}

fn part1((input, _): InputType) -> Int {
    let mut count = 0;

    for sample in input {
        let mut behaves_like: u8 = 0;
        for opcode in OPCODES {
            let result = run_instruction(sample.before, &opcode, sample.instruction);

            if result == sample.after {
                behaves_like += 1;
            }

            if behaves_like >= 3 {
                break;
            }
        }

        if behaves_like >= 3 {
            count += 1;
        }
    }

    count
}

fn part2((input, instructions): InputType) -> Int {
    let opcode_by_name = deduce_opcode_name(input);

    let mut register = [0; 4];

    for instruction in instructions {
        register = run_instruction(register, &opcode_by_name[instruction[0]], instruction);
    }

    register[0]
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

    println!("--- Day 16 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 547);
    assert_eq!(part2, 582);
}
