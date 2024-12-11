use std::collections::HashMap;

use aoc_2024::*;

const DAY: i32 = 11;
type Solution = usize;

fn get_parts(x: usize) -> Option<(usize, usize)> {
    let s = x.to_string();
    if s.len() % 2 != 0 {
        return None;
    }

    let half = s.len() / 2;
    let (left, right) = (&s[..half], &s[half..]);
    Some((left.parse().unwrap(), right.parse().unwrap()))
}

fn update_state(state: &mut HashMap<usize, usize>) {
    let mut new_state = HashMap::new();

    for (&pebble, &count) in state.iter() {
        if pebble == 0 {
            *new_state.entry(1).or_default() += count;
        } else if let Some((left, right)) = get_parts(pebble) {
            *new_state.entry(left).or_default() += count;
            *new_state.entry(right).or_default() += count;
        } else {
            *new_state.entry(pebble * 2024).or_default() += count;
        }
    }

    *state = new_state;
}

fn main() {
    let input = get_input_text(DAY);

    let state: HashMap<usize, usize> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .map(|pebble| (pebble, 1))
        .collect();

    let solution1: Solution = {
        let mut state = state.clone();
        (0..25).for_each(|_| update_state(&mut state));
        state.values().sum()
    };

    let solution2: Solution = {
        let mut state = state.clone();
        (0..75).for_each(|_| update_state(&mut state));
        state.values().sum()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
