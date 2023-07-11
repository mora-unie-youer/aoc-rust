use std::collections::HashMap;

use aoc_2021::*;

const DAY: i32 = 12;
type Solution = usize;

struct Graph {
    start: usize,
    end: usize,
    is_small: Vec<bool>,
    connections: Vec<Vec<usize>>,
}

impl From<&str> for Graph {
    fn from(input: &str) -> Self {
        let mut mapping = HashMap::new();
        let mut is_small = vec![];
        let mut connections = vec![];

        for connection in input.lines() {
            let (a, b) = connection.split_once('-').unwrap();

            // Map A to index
            let a_index = {
                let maybe_index = mapping.len();
                *mapping.entry(a).or_insert(maybe_index)
            };

            // If A index is new, then we need to append arrays
            if a_index == is_small.len() {
                is_small.push(a.chars().next().unwrap().is_ascii_lowercase());
                connections.push(Vec::new());
            }

            // Map B to index
            let b_index = {
                let maybe_index = mapping.len();
                *mapping.entry(b).or_insert(maybe_index)
            };

            // If B index is new, then we need to append arrays
            if b_index == is_small.len() {
                is_small.push(b.chars().next().unwrap().is_ascii_lowercase());
                connections.push(Vec::new());
            }

            // Push connections between A and B (with filter for part2)
            [(a, b), (b, a)]
                .into_iter()
                .filter(|&(a, b)| a != "end" && b != "start")
                .map(|(a, b)| (mapping[a], mapping[b]))
                .for_each(|(a, b)| connections[a].push(b));
        }

        Graph {
            start: mapping["start"],
            end: mapping["end"],
            is_small,
            connections,
        }
    }
}

fn count_paths(
    graph: &Graph,
    position: usize,
    mut visited: Vec<bool>,
    can_visit_again: bool,
) -> Solution {
    if position == graph.end {
        return 1;
    }

    visited[position] = graph.is_small[position];
    graph.connections[position]
        .iter()
        .filter(|&&pos| can_visit_again || !visited[pos])
        .map(|&pos| {
            count_paths(
                graph,
                pos,
                visited.clone(),
                can_visit_again && !visited[pos],
            )
        })
        .sum()
}

fn main() {
    let input = get_input_text(DAY);
    let graph = Graph::from(input.trim());

    let visited = vec![false; graph.connections.len()];
    let solution1: Solution = count_paths(&graph, graph.start, visited.clone(), false);
    let solution2: Solution = count_paths(&graph, graph.start, visited, true);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
