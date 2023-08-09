use std::collections::HashMap;

use aoc_2022::*;
use itertools::Itertools;

const DAY: i32 = 16;
type Solution = usize;

#[derive(Debug)]
struct Valve<'input> {
    name: &'input str,
    rate: usize,
    links: Vec<&'input str>,
}

impl<'input> From<&'input str> for Valve<'input> {
    fn from(value: &'input str) -> Self {
        let mut parts = value.split([' ', ',', '=', ';', ':']);

        let name = parts.nth(1).unwrap();
        let rate = parts.nth(3).unwrap().parse().unwrap();
        let links = parts.skip(5).step_by(2).collect();

        Self { name, rate, links }
    }
}

type Graph = Vec<Vec<usize>>;

#[allow(clippy::too_many_arguments)]
fn simulation(
    valves: &[Valve],
    non_zero_valves: &[usize],
    graph: &Graph,
    total_flow: usize,
    minutes: usize,
    state: usize,
    position: usize,
    memo: &mut HashMap<usize, usize>,
) -> Solution {
    let memoized_flow = memo.entry(state).or_insert(total_flow);
    *memoized_flow = total_flow.max(*memoized_flow);

    let mut max_total_flow = total_flow;
    for &i in non_zero_valves {
        if state & (1 << i) == 0 {
            continue;
        }

        let remaining_minutes = minutes.saturating_sub(graph[position][i] + 1);
        if remaining_minutes == 0 {
            continue;
        }

        let new_state = state & !(1 << i);
        let new_total_flow = total_flow + (remaining_minutes * valves[i].rate);
        let new_max_total_flow = simulation(
            valves,
            non_zero_valves,
            graph,
            new_total_flow,
            remaining_minutes,
            new_state,
            i,
            memo,
        );

        max_total_flow = max_total_flow.max(new_max_total_flow);
    }

    max_total_flow
}

fn main() {
    let input = get_input_text(DAY);

    let valves = input.lines().map(Valve::from).collect_vec();
    let mut graph = vec![vec![std::usize::MAX / 2; valves.len()]; valves.len()];

    // Creating links between valves
    for (i, valve) in valves.iter().enumerate() {
        for link in &valve.links {
            let j = valves.iter().position(|valve| &valve.name == link).unwrap();
            graph[i][j] = 1;
        }
    }

    // Floyd-Warshall algorithm
    for i in 0..graph.len() {
        for j in 0..graph.len() {
            for k in 0..graph.len() {
                graph[j][k] = graph[j][k].min(graph[j][i] + graph[i][k]);
            }
        }
    }

    // Preparing data for simulation
    let non_zero_valves = valves
        .iter()
        .enumerate()
        .filter(|(_, valve)| valve.rate > 0)
        .map(|(i, _)| i)
        .collect_vec();
    let initial_position = valves.iter().position(|valve| valve.name == "AA").unwrap();
    let initial_state = (1 << valves.len()) - 1;

    let solution1: Solution = simulation(
        &valves,
        &non_zero_valves,
        &graph,
        0,
        30,
        initial_state,
        initial_position,
        &mut HashMap::new(),
    );

    let solution2: Solution = {
        let mut memo = HashMap::new();
        let _ = simulation(
            &valves,
            &non_zero_valves,
            &graph,
            0,
            26,
            initial_state,
            initial_position,
            &mut memo,
        );

        memo.iter().fold(0, |max_flow, (&elf_mask, elf_flow)| {
            memo.iter()
                .fold(max_flow, |max_flow, (&elephant_mask, elephant_flow)| {
                    if !elephant_mask & !elf_mask & initial_state == 0 {
                        max_flow.max(elf_flow + elephant_flow)
                    } else {
                        max_flow
                    }
                })
        })
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
