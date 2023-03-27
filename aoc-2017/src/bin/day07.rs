use std::collections::{HashMap, HashSet};

use aoc_2017::*;

const DAY: i32 = 7;
type Solution = String;

struct Program<'input> {
    name: &'input str,
    weight: isize, // We will have negative deltas
    children: Vec<&'input str>,
}

impl<'input> From<&'input str> for Program<'input> {
    fn from(input: &'input str) -> Self {
        let parts: Vec<_> = input.split([' ', ',']).collect();
        let name = parts[0];
        let weight = {
            let text = parts[1];
            text[1..text.len() - 1].parse().unwrap()
        };

        let children = parts[2..]
            .iter()
            .skip(1)
            .cloned()
            .filter(|s| !s.is_empty())
            .collect();

        Self {
            name,
            weight,
            children,
        }
    }
}

impl Program<'_> {
    fn weight(&self, programs: &HashMap<&str, Program>) -> isize {
        let children_weight: isize = self
            .children
            .iter()
            .map(|child| programs.get(child).unwrap().weight(programs))
            .sum();
        self.weight + children_weight
    }
}

fn main() {
    let input = get_input_text(DAY);
    let programs: Vec<_> = input.lines().map(Program::from).collect();

    let solution1: Solution = {
        let children: HashSet<_> = programs
            .iter()
            .flat_map(|program| program.children.clone())
            .collect();

        programs
            .iter()
            .find(|program| !children.contains(program.name))
            .unwrap()
            .name
            .to_owned()
    };

    let solution2: Solution = {
        let programs: HashMap<&str, Program> = programs
            .into_iter()
            .map(|program| (program.name, program))
            .collect();
        let weights: HashMap<&str, isize> = programs
            .iter()
            .map(|(&name, program)| (name, program.weight(&programs)))
            .collect();

        let mut root = programs.get(solution1.as_str()).unwrap();
        let mut delta = 0;
        let result = loop {
            let mut children: Vec<_> = root
                .children
                .iter()
                .map(|child| (child, weights.get(child).unwrap()))
                .collect();
            children.sort_by_key(|child| child.1);

            let first = children.first().unwrap();
            let middle = children[1];
            let last = children.last().unwrap();

            if first.1 == last.1 {
                // Root is incorrect
                break root.weight + delta;
            } else if first.1 == middle.1 {
                // Last child is incorrect
                root = programs.get(last.0).unwrap();
                delta = children[1].1 - last.1;
            } else {
                // First child is incorrect
                root = programs.get(first.0).unwrap();
                delta = children[1].1 - first.1;
            }
        };

        result.to_string()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
