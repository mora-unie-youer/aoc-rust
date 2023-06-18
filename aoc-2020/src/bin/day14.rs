use std::collections::HashMap;

use aoc_2020::*;

const DAY: i32 = 14;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = {
        let mut memory = HashMap::new();
        let (mut set_mask, mut unset_mask) = (0, std::usize::MAX);
        for line in input.lines() {
            let (op1, op2) = line.split_once(" = ").unwrap();
            if op1.starts_with("mask") {
                set_mask = 0;
                unset_mask = 1;

                for ch in op2.chars() {
                    unset_mask |= 1;
                    match ch {
                        '1' => set_mask |= 1,
                        '0' => unset_mask ^= 1,
                        'X' => (),
                        _ => unreachable!(),
                    }

                    set_mask <<= 1;
                    unset_mask <<= 1;
                }

                // One extra shift
                set_mask >>= 1;
                unset_mask >>= 1;
            } else {
                let op2: usize = op2.parse().unwrap();
                let offset: usize = op1[4..op1.len() - 1].parse().unwrap();
                let value = (op2 | set_mask) & unset_mask;
                memory.insert(offset, value);
            }
        }

        memory.values().sum()
    };

    let solution2: Solution = {
        let mut memory = HashMap::new();
        let (mut set_mask, mut unset_mask) = (0, std::usize::MAX);
        let mut floating_mask = vec![];
        for line in input.lines() {
            let (op1, op2) = line.split_once(" = ").unwrap();
            if op1.starts_with("mask") {
                set_mask = 0;
                unset_mask = std::usize::MAX;
                floating_mask = vec![];
                for (i, ch) in op2.chars().rev().enumerate() {
                    match ch {
                        '1' => set_mask |= 1 << i,
                        'X' => {
                            unset_mask ^= 1 << i;
                            floating_mask.push(i)
                        }
                        '0' => (),
                        _ => unreachable!(),
                    }
                }
            } else {
                let op2: usize = op2.parse().unwrap();
                let offset: usize = op1[4..op1.len() - 1].parse().unwrap();
                let offset = (offset | set_mask) & unset_mask;

                for i in 0..(1 << floating_mask.len()) {
                    let bit_mask = floating_mask
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| i & (1 << j) != 0)
                        .fold(0, |acc, (_, bit)| acc | (1 << bit));
                    let offset = offset | bit_mask;
                    memory.insert(offset, op2);
                }
            }
        }

        memory.values().sum()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
