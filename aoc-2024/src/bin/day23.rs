use std::collections::{HashMap, HashSet};

use aoc_2024::*;
use itertools::Itertools;

const DAY: i32 = 23;
type Solution = String;

fn main() {
    let input = get_input_text(DAY);

    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        connections.entry(a).or_default().insert(b);
        connections.entry(b).or_default().insert(a);
    }

    let solution1: Solution = {
        let mut visited = HashSet::new();

        for (source, dests) in &connections {
            for a in dests {
                for b in dests {
                    if a == b || !connections[a].contains(b) {
                        continue;
                    }

                    let mut arr = [source, a, b];
                    arr.sort();
                    visited.insert(arr);
                }
            }
        }

        visited
            .into_iter()
            .filter(|arr| arr.iter().any(|node| node.starts_with('t')))
            .count()
            .to_string()
    };

    let solution2: Solution = {
        let mut largest_set = vec![];

        for (&node, neighbors) in &connections {
            let mut set = vec![node];

            for neighbor in neighbors {
                if set.iter().all(|n| connections[n].contains(neighbor)) {
                    set.push(neighbor);
                }
            }

            if largest_set.len() < set.len() {
                largest_set = set;
            }
        }

        largest_set.into_iter().sorted().join(",")
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
