use std::collections::{HashMap, HashSet};

use aoc_2017::*;

const DAY: i32 = 6;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let memory_banks: Vec<usize> = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let solution1: Solution = {
        let mut memory_banks = memory_banks.clone();

        let mut seen_configurations = HashSet::new();
        let mut redistribution_cycles = 0;
        loop {
            let max_blocks = *memory_banks.iter().max().unwrap();
            let max_index = memory_banks.iter().position(|&n| n == max_blocks).unwrap();
            let mut blocks_to_redistribute = memory_banks[max_index];
            memory_banks[max_index] = 0;
            let mut current_index = max_index;
            while blocks_to_redistribute > 0 {
                current_index = (current_index + 1) % memory_banks.len();
                memory_banks[current_index] += 1;
                blocks_to_redistribute -= 1;
            }
            redistribution_cycles += 1;

            if !seen_configurations.insert(memory_banks.clone()) {
                break redistribution_cycles;
            }
        }
    };

    let solution2: Solution = {
        let mut memory_banks = memory_banks;

        let mut seen_configurations = HashMap::new();
        let mut redistribution_cycles = 0;
        loop {
            let max_blocks = *memory_banks.iter().max().unwrap();
            let max_index = memory_banks.iter().position(|&n| n == max_blocks).unwrap();
            let mut blocks_to_redistribute = memory_banks[max_index];
            memory_banks[max_index] = 0;
            let mut current_index = max_index;
            while blocks_to_redistribute > 0 {
                current_index = (current_index + 1) % memory_banks.len();
                memory_banks[current_index] += 1;
                blocks_to_redistribute -= 1;
            }
            redistribution_cycles += 1;

            if let Some(prev) =
                seen_configurations.insert(memory_banks.clone(), redistribution_cycles)
            {
                break redistribution_cycles - prev;
            }
        }
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
