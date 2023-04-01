use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_2017::*;

const DAY: i32 = 12;
type Solution = usize;

fn solve(start: usize, connections: &HashMap<usize, Vec<usize>>, visited: &mut HashSet<usize>) {
    let mut queue = BinaryHeap::new();
    visited.insert(start);
    queue.push(start);

    while let Some(program) = queue.pop() {
        let neighbors = connections.get(&program).unwrap();
        for &neighbor in neighbors {
            if visited.insert(neighbor) {
                queue.push(neighbor);
            }
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let connections: HashMap<usize, Vec<usize>> = input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" <-> ").unwrap();
            let from = from.parse().unwrap();
            let to = to.split(", ").map(|v| v.parse().unwrap()).collect();
            (from, to)
        })
        .collect();

    let solution1: Solution = {
        let mut visited = HashSet::new();
        solve(0, &connections, &mut visited);
        visited.len()
    };

    let solution2: Solution = {
        let mut visited = HashSet::new();
        let mut groups = 0;

        for start in connections.keys() {
            if !visited.contains(start) {
                solve(*start, &connections, &mut visited);
                groups += 1;
            }
        }

        groups
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
