use std::hash::{Hash, Hasher};

use aoc_2019::*;
use pathfinding::directed::bfs;

// Took code from day 11
const DAY: i32 = 25;
type Solution = String;

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
        // NOTE: optimized this as it was slow as fuck
        let value = self.data[self.ip as usize] as u32;
        let exp = parameter as u32 + 1;
        value / 10u32.pow(exp) % 10
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

    fn output_string(&self) -> String {
        self.output.iter().map(|&v| v as u8 as char).collect()
    }
}

#[allow(dead_code)]
fn interactive(input: &str) -> Solution {
    let data: Vec<_> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();
    let mut program = Program::new(&data, &[]);

    loop {
        // If halted -> break
        let halted = program.run();
        let output: String = program.output.iter().map(|&v| v as u8 as char).collect();
        print!("{}", output);

        if halted {
            break;
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        program.input = input.chars().map(|v| v as isize).collect();
    }

    String::new()
}

#[derive(Clone)]
struct State {
    room: String,
    items: Vec<String>,
    program: Program,
}

impl State {
    fn new(mut program: Program) -> Self {
        program.run(); // We need to run program to parse first output
        let output = program.output_string();
        let output = output.trim();

        let room = output.lines().next().unwrap();
        Self {
            room: room.into(),
            items: vec![],
            program,
        }
    }

    fn successors(&self) -> Vec<State> {
        // "fake" run to get halted state (program should wait for input or halt)
        if self.program.clone().run() {
            return vec![];
        }

        let (_, room_doors, room_items) = self.parse_output();
        let mut successors = vec![];

        // Move to another door
        successors.extend(room_doors.iter().map(|door| {
            let mut new_state = self.clone();
            // Clear output before move
            new_state.program.output.clear();

            // Create input command for moving
            let input = format!("{}\n", door);
            new_state.program.input = input.chars().map(|ch| ch as isize).collect();

            // Run program to fetch new title
            new_state.program.run();

            // Fetching title
            let (title, _, _) = new_state.parse_output();
            new_state.room = title;

            new_state
        }));

        // Take item
        successors.extend(
            room_items
                .iter()
                // Blacklist for avoiding spinlock
                .filter(|item| item != &"infinite loop")
                .filter(|item| !self.items.contains(item))
                .map(|item| {
                    let mut new_state = self.clone();

                    // Create input command for taking item
                    let input = format!("take {}\n", item);
                    new_state.program.input = input.chars().map(|ch| ch as isize).collect();

                    // Run program to take item
                    new_state.program.run();

                    // Take item
                    new_state.items.push(item.clone());
                    new_state.items.sort();

                    new_state
                }),
        );

        // NOTE: Dropping items is unnecessary, as we have all the item variants
        successors
    }

    fn goal(&self) -> bool {
        self.program.output_string().contains("hello")
    }

    fn parse_output(&self) -> (String, Vec<String>, Vec<String>) {
        let output = self.program.output_string();
        let output = output.trim();

        let mut parts = output.split("\n\n");

        let description = parts.next().unwrap(); // Must be everywhere
        let title = description
            .lines()
            .next()
            .filter(|v| v.starts_with("=="))
            .map(|v| v.to_owned())
            .unwrap_or(self.room.clone());

        let room_doors = parts
            .next()
            .filter(|v| v.starts_with("Doors"))
            .unwrap_or(""); // Shown after move
        let room_items = parts
            .next()
            .filter(|v| v.starts_with("Items"))
            .unwrap_or(""); // Shown after move

        let room_doors: Vec<_> = room_doors
            .lines()
            .skip(1)
            .map(|v| v[2..].to_owned())
            .collect();
        let room_items: Vec<_> = room_items
            .lines()
            .skip(1)
            .map(|v| v[2..].to_owned())
            .collect();
        (title, room_doors, room_items)
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.room == other.room && self.items == other.items
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.room.hash(state);
        self.items.hash(state);
    }
}

fn solve(input: &str) -> Solution {
    let data: Vec<_> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();
    let program = Program::new(&data, &[]);

    let explore_path = bfs::bfs(
        &State::new(program),
        |state| state.successors(),
        |state| state.goal(),
    )
    .unwrap();

    let room = explore_path.last().unwrap();
    let output = room.program.output_string();
    output
        .split_whitespace()
        .find(|v| v.parse::<usize>().is_ok())
        .unwrap()
        .to_owned()
}

fn main() {
    let input = get_input_text(DAY);
    let solution1: Solution = solve(&input);
    show_solution(DAY, solution1);
}
