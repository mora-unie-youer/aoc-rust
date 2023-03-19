#![feature(array_windows)]

use std::collections::HashMap;

use aoc_2016::*;
use itertools::Itertools;
use pathfinding::directed::bfs;

const DAY: i32 = 24;
type Solution = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(usize, usize);
impl Pos {
    fn neighbors(&self, map: &Vec<Vec<bool>>) -> Vec<Self> {
        [
            Pos(self.0 - 1, self.1),
            Pos(self.0 + 1, self.1),
            Pos(self.0, self.1 - 1),
            Pos(self.0, self.1 + 1),
        ]
        .into_iter()
        .filter(|pos| !map[pos.1][pos.0])
        .collect()
    }
}

fn main() {
    let input = get_input_text(DAY);

    let mut map = vec![];
    let mut targets = vec![Pos(0, 0)];
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, ch) in line.char_indices() {
            if ch == '#' {
                row.push(true);
            } else {
                if ch == '0' {
                    targets[0] = Pos(x, y);
                } else if ch != '.' {
                    targets.push(Pos(x, y));
                }

                row.push(false);
            }
        }
        map.push(row);
    }

    let mut target_distances = HashMap::new();
    for (&t1, &t2) in targets.iter().tuple_combinations() {
        let result = bfs::bfs(&t1, |pos| pos.neighbors(&map), |&pos| pos == t2).unwrap();
        let distance = result.len() - 1;
        target_distances.insert((t1, t2), distance);
        target_distances.insert((t2, t1), distance);
    }
    let start = targets.remove(0);

    let solution1: Solution = targets
        .iter()
        .permutations(targets.len())
        .map(|path| {
            let to_first = target_distances[&(start, *path[0])];
            path.array_windows().fold(to_first, |acc, [t1, t2]| {
                acc + target_distances[&(**t1, **t2)]
            })
        })
        .min()
        .unwrap();

    let solution2: Solution = targets
        .iter()
        .permutations(targets.len())
        .map(|path| {
            let to_first = target_distances[&(start, *path[0])];
            let to_start = target_distances[&(start, **path.last().unwrap())];
            path.array_windows()
                .fold(to_first + to_start, |acc, [t1, t2]| {
                    acc + target_distances[&(**t1, **t2)]
                })
        })
        .min()
        .unwrap();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
