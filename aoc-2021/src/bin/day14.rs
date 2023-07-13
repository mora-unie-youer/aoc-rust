#![feature(array_windows)]

use std::collections::HashMap;

use aoc_2021::*;
use itertools::Itertools;

const DAY: i32 = 14;
type Solution = usize;

fn next_polymer(polymer: Vec<char>, rules: &HashMap<[char; 2], char>) -> Vec<char> {
    let mut new_polymer = vec![polymer[0]];

    for pair in polymer.array_windows() {
        new_polymer.push(rules[pair]);
        new_polymer.push(pair[1]);
    }

    new_polymer
}

fn main() {
    let input = get_input_text(DAY);
    let (polymer, rules) = input.split_once("\n\n").unwrap();
    let polymer: Vec<char> = polymer.trim().chars().collect();
    let rules: HashMap<[char; 2], char> = rules
        .lines()
        .filter_map(|line| line.split_once(" -> "))
        .map(|(pair, ch)| {
            (
                pair.chars().collect::<Vec<_>>().try_into().unwrap(),
                ch.chars().next().unwrap(),
            )
        })
        .collect();

    let solution1: Solution = {
        let new_polymer = (0..10).fold(polymer.clone(), |acc, _| next_polymer(acc, &rules));
        let counts = new_polymer.into_iter().counts();
        let (min, max) = counts.values().minmax().into_option().unwrap();
        max - min
    };

    let solution2: Solution = {
        let polymer = polymer;
        let last_ch = *polymer.last().unwrap();

        let pair_counts =
            (0..40).fold(polymer.into_iter().tuple_windows().counts(), |counts, _| {
                let mut next_counts = HashMap::new();

                for ((a, b), count) in counts {
                    let c = rules[&[a, b]];
                    *next_counts.entry((a, c)).or_default() += count;
                    *next_counts.entry((c, b)).or_default() += count;
                }

                next_counts
            });

        let mut char_counts: HashMap<char, usize> = HashMap::new();
        for ((a, _), count) in pair_counts {
            *char_counts.entry(a).or_default() += count;
        }
        // Add last symbol to count
        *char_counts.entry(last_ch).or_default() += 1;

        let (min, max) = char_counts.values().minmax().into_option().unwrap();
        max - min
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
