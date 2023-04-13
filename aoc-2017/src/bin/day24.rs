use std::collections::HashSet;

use aoc_2017::*;

const DAY: i32 = 24;
type Solution = usize;

#[derive(Clone, Copy, Debug)]
struct Component(usize, usize);

impl From<&str> for Component {
    fn from(value: &str) -> Self {
        let (a, b) = value.split_once('/').unwrap();
        Self(a.parse().unwrap(), b.parse().unwrap())
    }
}

fn solve(
    current: usize,
    components: &[Component],
    visited: &mut HashSet<usize>,
    part2: bool,
) -> (Solution, Solution) {
    components
        .iter()
        .enumerate()
        .filter(|(_, component)| component.0 == current || component.1 == current)
        .filter_map(|(i, component)| {
            if visited.contains(&i) {
                return None;
            }

            visited.insert(i);
            let (length, strength) = solve(
                component.0 + component.1 - current,
                components,
                visited,
                part2,
            );
            visited.remove(&i);

            Some((
                length + part2 as usize,
                strength + component.0 + component.1,
            ))
        })
        .max()
        .unwrap_or((0, 0))
}

fn main() {
    let input = get_input_text(DAY);
    let components: Vec<_> = input.lines().map(Component::from).collect();

    let solution1: Solution = solve(0, &components, &mut HashSet::new(), false).1;
    let solution2: Solution = solve(0, &components, &mut HashSet::new(), true).1;

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
