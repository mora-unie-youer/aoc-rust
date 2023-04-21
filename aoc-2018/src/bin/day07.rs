use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};

use aoc_2018::*;

const DAY: i32 = 7;
type Solution = String;

fn solve(
    successors: HashMap<char, Vec<char>>,
    mut dependencies: HashMap<char, HashSet<char>>,
    workers_count: usize,
    time_offset: i32,
) -> (String, i32) {
    let mut workers = BinaryHeap::new();
    let mut ready = dependencies
        .iter()
        .filter_map(|(&a, b)| if b.is_empty() { Some(a) } else { None })
        .collect::<BTreeSet<_>>();

    for r in ready.iter() {
        dependencies.remove(r);
    }

    let mut time = 0;
    let mut completed = String::new();
    while !(ready.is_empty() && workers.is_empty()) {
        while workers.len() < workers_count && !ready.is_empty() {
            let job = *ready.iter().next().unwrap();
            ready.remove(&job);
            let completion = time - time_offset - (job as i32 - i32::from(b'A') + 1);
            workers.push((completion, job));
        }

        let (t, j) = workers.pop().unwrap();
        time = t;
        completed.push(j);
        if let Some(succ) = successors.get(&j) {
            for &job in succ {
                let pred = dependencies.get_mut(&job).unwrap();
                pred.remove(&j);
                if pred.is_empty() {
                    ready.insert(job);
                }
            }
        }
    }

    (completed, -time)
}

fn main() {
    let input = get_input_text(DAY);
    let (successors, dependencies) = {
        let mut successors: HashMap<char, Vec<char>> = HashMap::new();
        let mut dependencies: HashMap<char, HashSet<char>> = HashMap::new();

        input
            .lines()
            .map(|line| line.as_bytes())
            .map(|line| (line[5] as char, line[36] as char))
            .for_each(|(dep, job)| {
                successors.entry(dep).or_default().push(job);
                dependencies.entry(job).or_default().insert(dep);
                dependencies.entry(dep).or_default();
            });

        (successors, dependencies)
    };

    let solution1: Solution = solve(successors.clone(), dependencies.clone(), 1, 0).0;
    let solution2: Solution = solve(successors, dependencies, 5, 60).1.to_string();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::solve;

    #[test]
    fn test_solve() {
        let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

        let (successors, dependencies) = {
            let mut successors: HashMap<char, Vec<char>> = HashMap::new();
            let mut dependencies: HashMap<char, HashSet<char>> = HashMap::new();

            input
                .lines()
                .map(|line| line.as_bytes())
                .map(|line| (line[5] as char, line[36] as char))
                .for_each(|(dep, job)| {
                    successors.entry(dep).or_default().push(job);
                    dependencies.entry(job).or_default().insert(dep);
                    dependencies.entry(dep).or_default();
                });

            (successors, dependencies)
        };

        let (completed, _) = solve(successors.clone(), dependencies.clone(), 1, 0);
        assert_eq!(completed, "CABDFE");

        let (completed, time) = solve(successors, dependencies, 2, 0);
        assert_eq!(completed, "CABFDE");
        assert_eq!(time, 15);
    }
}
