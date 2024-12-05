use std::collections::{HashMap, HashSet};

use aoc_2024::*;

const DAY: i32 = 5;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut rules_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for rule in rules.lines() {
        let (a, b) = rule.split_once('|').unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();
        rules_map.entry(b).or_default().insert(a);
    }

    let updates: Vec<Vec<usize>> = updates
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    let solution1: Solution = {
        let mut result = 0;
        'update: for pages in &updates {
            // Checking rules for update
            for i in 0..pages.len() {
                let current = pages[i];
                let rule = rules_map.entry(current).or_default();
                if pages[i + 1..].iter().any(|x| rule.contains(x)) {
                    continue 'update;
                }
            }

            // Update is correct
            result += pages[pages.len() / 2];
        }

        result
    };

    let solution2: Solution = {
        let mut result = 0;

        for pages in &updates {
            // Checking rules for update
            let is_correct = (0..pages.len() - 1).all(|i| {
                pages[i + 1..].iter().all(|x| {
                    rules_map
                        .get(&pages[i])
                        .map(|rule| !rule.contains(x))
                        .unwrap_or(false)
                })
            });

            if is_correct {
                continue;
            }

            // Update is incorrect, fixing
            let mut pages = pages.clone();
            let mut i = 0;
            while i < pages.len() {
                let current = pages[i];
                let rule = rules_map.entry(current).or_default();
                match pages
                    .iter()
                    .enumerate()
                    .skip(i + 1)
                    .rfind(|(_, x)| rule.contains(x))
                {
                    Some((j, _)) => {
                        pages.insert(j + 1, current);
                        pages.remove(i);
                    }
                    None => i += 1,
                }
            }

            result += pages[pages.len() / 2];
        }

        result
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
