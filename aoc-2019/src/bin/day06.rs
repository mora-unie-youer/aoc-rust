use std::collections::{HashMap, HashSet};

use aoc_2019::*;

const DAY: i32 = 6;
type Solution = usize;

fn count_orbits<'input>(
    orbits: &HashMap<&'input str, &'input str>,
    obj: &'input str,
    cache: &mut HashMap<&'input str, usize>,
) -> usize {
    if let Some(&count) = cache.get(obj) {
        return count;
    }

    let count = orbits
        .get(obj)
        .map_or(0, |&parent| 1 + count_orbits(orbits, parent, cache));
    cache.insert(obj, count);
    count
}

fn main() {
    let input = get_input_text(DAY);

    let orbits: HashMap<&str, &str> = input
        .lines()
        .map(|line| line.split_once(')').unwrap())
        .map(|(a, b)| (b, a))
        .collect();

    let solution1: Solution = orbits
        .keys()
        .map(|orbit| count_orbits(&orbits, orbit, &mut HashMap::new()))
        .sum();

    let solution2: Solution = {
        let parent = |x: &&str| orbits.get(x).cloned();
        // Full paths from start to SAN/YOU
        let san: HashSet<_> = std::iter::successors(Some("SAN"), parent).collect();
        let you: HashSet<_> = std::iter::successors(Some("YOU"), parent).collect();
        // Finding intersection
        let common = san.intersection(&you).count();
        san.len() + you.len() - 2 * (common + 1)
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
