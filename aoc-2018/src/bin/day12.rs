use std::collections::HashMap;

use aoc_2018::*;

const DAY: i32 = 12;
type Solution = isize;

struct Rule([bool; 5], bool);
impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (from, to) = value.split_once(" => ").unwrap();
        let from: Vec<_> = from.chars().map(|ch| ch == '#').collect();
        let to = to.starts_with('#');
        Self(from.try_into().unwrap(), to)
    }
}

const COINCIDENCE_LIMIT: usize = 15;
fn solve(input: &str, generations: isize) -> Solution {
    let (initial_state, rules) = input.split_once("\n\n").unwrap();
    let initial_state: Vec<_> = Some(initial_state)
        .map(|line| line.split_once(": ").unwrap().1)
        .unwrap()
        .chars()
        .map(|ch| ch == '#')
        .collect();
    let rules: HashMap<[bool; 5], bool> = rules
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" => ").unwrap();
            let from: Vec<_> = from.chars().map(|ch| ch == '#').collect();
            let to = to.starts_with('#');
            (from.try_into().unwrap(), to)
        })
        .collect();

    let mut deltas: HashMap<isize, usize> = HashMap::new();
    let mut last_score = 0;

    let mut state = vec![false; 3];
    state.extend(initial_state);
    state.extend([false; 3]);
    for generation in 1..=generations {
        let mut new_state = vec![false; 3];
        for i in 2..state.len() - 2 {
            let slice: [bool; 5] = state[i - 2..=i + 2].try_into().unwrap();
            let value = *rules.get(&slice).unwrap_or(&false);
            new_state.push(value);
        }
        new_state.extend([false; 3]);
        state = new_state;

        let score = state
            .iter()
            .enumerate()
            .filter(|(_, &pot)| pot)
            .map(|(i, _)| i as isize - (3 + generation))
            .sum();
        let count = deltas.entry(score - last_score).or_default();
        if *count > COINCIDENCE_LIMIT {
            return (generations - generation) * (score - last_score) + score;
        } else {
            *count += 1;
        }

        last_score = score;
    }

    last_score
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = solve(&input, 20);
    let solution2: Solution = solve(&input, 50_000_000_000);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";
        assert_eq!(solve(input, 20), 325);
    }
}
