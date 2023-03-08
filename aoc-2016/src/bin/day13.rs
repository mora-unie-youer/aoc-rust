use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use aoc_2016::*;
use pathfinding::directed::bfs;

const DAY: i32 = 13;
type Solution = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(usize, usize);

impl Default for Pos {
    fn default() -> Self {
        Self(1, 1)
    }
}

impl Pos {
    fn is_wall(&self, code: usize) -> bool {
        let Pos(x, y) = self;
        let magic = x * x + 3 * x + 2 * x * y + y + y * y;
        (magic + code).count_ones() % 2 == 1
    }

    fn all_neighbors(&self) -> Vec<Pos> {
        let mut neighbors = vec![Pos(self.0 + 1, self.1), Pos(self.0, self.1 + 1)];

        if self.0 > 0 {
            neighbors.push(Pos(self.0 - 1, self.1));
        }

        if self.1 > 0 {
            neighbors.push(Pos(self.0, self.1 - 1));
        }

        neighbors
    }

    fn open_neighbors(&self, code: usize) -> Vec<Pos> {
        self.all_neighbors()
            .into_iter()
            .filter(|pos| !pos.is_wall(code))
            .collect()
    }
}

#[derive(PartialEq, Eq)]
struct State(usize, Pos);

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = get_input_text(DAY);
    let code = input.trim().parse().unwrap();

    let start_pos = Pos::default();

    let solution1: Solution = {
        let result = bfs::bfs(
            &start_pos,
            |pos| pos.open_neighbors(code),
            |pos| *pos == Pos(31, 39),
        )
        .expect("Couldn't find solution");

        result.len() - 1
    };

    let solution2: Solution = {
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();
        visited.insert(start_pos);
        queue.push(State(1, start_pos));

        while let Some(State(steps, pos)) = queue.pop() {
            if steps > 50 {
                break;
            }

            pos.open_neighbors(code).into_iter().for_each(|pos| {
                if visited.insert(pos) {
                    queue.push(State(steps + 1, pos));
                }
            })
        }

        visited.len()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
