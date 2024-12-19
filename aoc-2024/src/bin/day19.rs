use std::collections::HashMap;

use aoc_2024::*;
use itertools::Itertools;

const DAY: i32 = 19;
type Solution = usize;

fn buildable(towels: &[&str], design: &str) -> usize {
    fn buildable_inner<'input>(
        towels: &[&'input str],
        design: &'input str,
        memo: &mut HashMap<&'input str, usize>,
    ) -> usize {
        if let Some(&count) = memo.get(design) {
            return count;
        }

        if design.is_empty() {
            return 1;
        }

        let mut count = 0;
        for &towel in towels {
            if !design.starts_with(towel) {
                continue;
            }

            count += buildable_inner(towels, &design[towel.len()..], memo);
        }

        memo.insert(design, count);
        count
    }

    buildable_inner(towels, design, &mut HashMap::new())
}

fn main() {
    let input = get_input_text(DAY);
    let (towels, designs) = input.split_once("\n\n").unwrap();

    let towels = towels.split(", ").collect_vec();
    let designs = designs.lines().collect_vec();

    let solution1: Solution = designs
        .iter()
        .filter(|&&design| buildable(&towels, design) != 0)
        .count();

    let solution2: Solution = designs
        .iter()
        .map(|&design| buildable(&towels, design))
        .sum();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
