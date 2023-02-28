#![feature(control_flow_enum)]

use std::{cmp::Ordering, collections::HashSet, ops::ControlFlow};

use aoc_2015::*;
use itertools::Itertools;

const DAY: i32 = 24;
type Solution = usize;

fn minimal_count(weights: &[Solution], split_size: usize) -> usize {
    weights
        .iter()
        .rev()
        .try_fold((0, 0), |(count, mut sum), weight| {
            sum += weight;
            if sum >= split_size {
                ControlFlow::Break(count)
            } else {
                ControlFlow::Continue((count + 1, sum))
            }
        })
        .break_value()
        .unwrap()
}

fn solve(weights: &[Solution], splits: usize) -> Solution {
    let split_size = weights.iter().sum::<usize>() / splits;
    let min_count = minimal_count(weights, split_size);
    let max_count = weights.len() / splits;

    let mut choices: HashSet<(usize, Solution)> = HashSet::new();
    for count in min_count..max_count {
        for combination in weights.iter().combinations(count) {
            let (mut sum, mut product) = (0, 1);
            for (i, &&weight) in combination.iter().enumerate() {
                sum += weight;
                product *= weight;

                match sum.cmp(&split_size) {
                    Ordering::Equal => {
                        choices.insert((i + 1, product));
                    }
                    Ordering::Greater => break,
                    _ => (),
                }
            }
        }

        if !choices.is_empty() {
            break;
        }
    }

    let min_count = choices.iter().min_by_key(|(count, _)| count).unwrap().0;
    choices
        .iter()
        .filter(|&&(count, _)| count == min_count)
        .min_by_key(|(_, product)| product)
        .unwrap()
        .1
}

fn main() {
    let input = get_input_text(DAY);
    let weights: Vec<Solution> = input.lines().map(|line| line.parse().unwrap()).collect();

    let solution1: Solution = solve(&weights, 3);
    let solution2: Solution = solve(&weights, 4);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
