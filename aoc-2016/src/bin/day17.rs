use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    hash::Hash,
};

use aoc_2016::*;

use crypto::{digest::Digest, md5::Md5};
use pathfinding::directed::bfs;

const DAY: i32 = 17;
type Solution = String;

trait IsOpen {
    fn is_open(&self) -> bool;
}

impl IsOpen for char {
    fn is_open(&self) -> bool {
        matches!(*self, 'b'..='f')
    }
}

#[derive(Clone, PartialEq, Eq)]
struct State<'input> {
    passcode: &'input str,
    code: String,
    x: usize,
    y: usize,
}

impl Hash for State<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.code.len().cmp(&self.code.len())
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'input> State<'input> {
    fn new(passcode: &'input str) -> Self {
        Self {
            passcode,
            code: String::new(),
            x: 0,
            y: 3,
        }
    }

    fn doors(&self) -> [bool; 4] {
        let mut digest = Md5::new();
        digest.input_str(self.passcode);
        digest.input_str(&self.code);
        digest.result_str()[..=3]
            .chars()
            .map(|ch| ch.is_open())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn next_neighbor(&self, dir: char) -> Option<Self> {
        let mut new_state = self.clone();
        new_state.code.push(dir);

        match dir {
            'U' if self.y < 3 => new_state.y = self.y + 1,
            'D' if self.y > 0 => new_state.y = self.y - 1,
            'L' if self.x > 0 => new_state.x = self.x - 1,
            'R' if self.x < 3 => new_state.x = self.x + 1,
            _ => return None,
        }

        Some(new_state)
    }

    fn neighbors(&self) -> Vec<Self> {
        const DIRECTIONS: [char; 4] = ['U', 'D', 'L', 'R'];
        self.doors()
            .into_iter()
            .enumerate()
            .filter_map(|(i, door)| {
                if door {
                    self.next_neighbor(DIRECTIONS[i])
                } else {
                    None
                }
            })
            .collect()
    }

    fn is_goal(&self) -> bool {
        self.x == 3 && self.y == 0
    }
}

fn main() {
    let input = get_input_text(DAY);
    let passcode = input.trim();

    let solution1: Solution = {
        let result = bfs::bfs(
            &State::new(passcode),
            |state| state.neighbors(),
            |state| state.is_goal(),
        )
        .unwrap();

        result.last().unwrap().code.clone()
    };

    let solution2: Solution = {
        let mut paths = vec![];
        let mut queue = BinaryHeap::new();
        queue.push(State::new(passcode));

        while let Some(state) = queue.pop() {
            if state.is_goal() {
                paths.push(state.code.len());
                continue;
            }

            state
                .neighbors()
                .into_iter()
                .for_each(|state| queue.push(state));
        }

        paths.iter().max().unwrap().to_string()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
