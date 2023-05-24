use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use aoc_2019::*;
use pathfinding::directed::{bfs, dijkstra};

// Took code from day 11
const DAY: i32 = 15;
type Solution = usize;

#[derive(Clone)]
struct Program {
    ip: isize,
    relative_base: isize,

    data: Vec<isize>,

    input: Vec<isize>,
    output: Vec<isize>,
}

impl Program {
    fn new(data: &[isize], input: &[isize]) -> Self {
        Self {
            ip: 0,
            relative_base: 0,

            data: data.to_vec(),

            input: input.to_vec(),
            output: Vec::new(),
        }
    }

    fn run(&mut self) -> bool {
        loop {
            let opcode = self.data[self.ip as usize] % 100;
            match opcode {
                1 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    let out = self.get_offset(3) as usize;
                    *self.get_value_mut(out) = op1 + op2;
                    self.ip += 4;
                }
                2 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    let out = self.get_offset(3) as usize;
                    *self.get_value_mut(out) = op1 * op2;
                    self.ip += 4;
                }
                3 => {
                    if self.input.is_empty() {
                        // If there's no input -> pausing
                        break false;
                    }

                    let out = self.get_offset(1) as usize;
                    *self.get_value_mut(out) = self.input.remove(0);
                    self.ip += 2;
                }
                4 => {
                    let op = self.get_parameter(1);
                    self.output.push(op);
                    self.ip += 2;
                }
                5 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    if op1 != 0 {
                        self.ip = op2;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    if op1 == 0 {
                        self.ip = op2;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    let out = self.get_offset(3) as usize;
                    *self.get_value_mut(out) = (op1 < op2) as isize;
                    self.ip += 4;
                }
                8 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    let out = self.get_offset(3) as usize;
                    *self.get_value_mut(out) = (op1 == op2) as isize;
                    self.ip += 4;
                }
                9 => {
                    let op1 = self.get_parameter(1);
                    self.relative_base += op1;
                    self.ip += 2;
                }
                99 => break true,
                _ => panic!("Something went wrong"),
            }
        }
    }

    fn get_mode(&self, parameter: isize) -> u32 {
        self.data[self.ip as usize]
            .to_string()
            .chars()
            .rev()
            .nth((parameter + 1) as usize)
            .unwrap_or('0')
            .to_digit(10)
            .unwrap()
    }

    fn get_offset_with_mode(&self, offset: isize, mode: u32) -> isize {
        match mode {
            2 => self.data[(self.ip + offset) as usize] + self.relative_base,
            _ => self.data[(self.ip + offset) as usize],
        }
    }

    fn get_offset(&self, offset: isize) -> isize {
        let mode = self.get_mode(offset);
        self.get_offset_with_mode(offset, mode)
    }

    fn get_value(&mut self, address: usize) -> isize {
        // Now we usually need to allocate memory
        if self.data.len() <= address {
            self.data.resize(address + 1, 0);
        }

        self.data[address]
    }

    fn get_value_mut(&mut self, address: usize) -> &mut isize {
        // Now we usually need to allocate memory
        if self.data.len() <= address {
            self.data.resize(address + 1, 0);
        }

        &mut self.data[address]
    }

    fn get_parameter(&mut self, parameter: isize) -> isize {
        let mode = self.get_mode(parameter);
        let parameter = self.get_offset_with_mode(parameter, mode);
        match mode {
            0 | 2 => self.get_value(parameter as usize),
            1 => parameter,
            _ => unreachable!(),
        }
    }
}

struct State(usize, (isize, isize), Program);

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn explore(input: &str) -> HashMap<(isize, isize), isize> {
    let data: Vec<_> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();
    let program = Program::new(&data, &[]);

    let mut map = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(State(0, (0, 0), program));
    while let Some(State(steps, (x, y), program)) = queue.pop() {
        let neighbors = [
            (1, (x, y - 1)),
            (2, (x, y + 1)),
            (3, (x - 1, y)),
            (4, (x + 1, y)),
        ];

        for (dir, pos) in neighbors {
            if map.contains_key(&pos) {
                continue;
            }

            let mut new_program = program.clone();
            new_program.input.push(dir);
            new_program.run();

            let result = new_program.output.pop().unwrap();
            map.insert(pos, result);
            match result {
                0 => continue,
                1 | 2 => queue.push(State(steps + 1, pos, new_program)),
                _ => unreachable!(),
            }
        }
    }

    map
}

fn main() {
    let input = get_input_text(DAY);
    let map = explore(&input);

    let solution1: Solution = {
        let map = &map;
        let start_point = (0, 0);
        let result = bfs::bfs(
            &start_point,
            |&(x, y)| {
                [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .into_iter()
                    .filter(|p| map[p] != 0)
            },
            |pos| map[pos] == 2,
        )
        .unwrap();

        result.len() - 1
    };

    let solution2: Solution = {
        let map = &map;
        let start_point = map.iter().find(|&(_, &tile)| tile == 2).unwrap().0;
        let result = dijkstra::dijkstra_all(start_point, |&(x, y)| {
            [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .into_iter()
                .filter(|p| map[p] != 0)
                .map(|p| (p, 1))
        });

        result
            .into_iter()
            .map(|(_, (_, steps))| steps)
            .max()
            .unwrap()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
