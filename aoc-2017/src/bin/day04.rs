use std::collections::HashSet;

use aoc_2017::*;

const DAY: i32 = 4;
type Solution = usize;

trait Normalize {
    fn normalize(&self) -> String;
}

impl Normalize for &str {
    fn normalize(&self) -> String {
        let mut chars: Vec<_> = self.chars().collect();
        chars.sort();
        chars.iter().collect()
    }
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input
        .lines()
        .filter(|line| {
            let mut words = HashSet::new();
            !line.split_whitespace().any(|word| !words.insert(word))
        })
        .count();

    let solution2: Solution = input
        .lines()
        .filter(|line| {
            let mut words = HashSet::new();
            !line.split_whitespace().any(|word| !words.insert(word.normalize()))
        })
        .count();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
