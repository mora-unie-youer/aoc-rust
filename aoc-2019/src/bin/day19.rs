use std::collections::VecDeque;

use aoc_2019::*;

// Took code from day 11
const DAY: i32 = 19;
type Solution = isize;

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

fn check_coordinate(data: &[isize], x: isize, y: isize) -> bool {
    let mut program = Program::new(data, &[x, y]);
    program.run();
    program.output[0] == 1
}

fn main() {
    let input = get_input_text(DAY);
    let data: Vec<_> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let solution1: Solution = {
        let mut count = 0;

        for y in 0..50 {
            for x in 0..50 {
                count += check_coordinate(&data, x, y) as isize;
            }
        }

        count
    };

    let solution2: Solution = {
        let mut start_y = 0;
        let mut rows = VecDeque::new();

        // Find a starting point in 30x30
        'search: for y in 0..30 {
            for x in 0..30 {
                let top_left = check_coordinate(&data, x, y);
                let top = check_coordinate(&data, x + 1, y);
                let left = check_coordinate(&data, x, y + 1);

                if top_left && top && left {
                    let (mut x, mut y) = (x, y);

                    while check_coordinate(&data, x, y - 1) {
                        y -= 1;
                    }
                    while check_coordinate(&data, x - 1, y) {
                        x -= 1;
                    }

                    let start = x;
                    while check_coordinate(&data, x + 1, y) {
                        x += 1;
                    }
                    let end = x;

                    start_y = y;
                    rows.push_back(start..=end);
                    break 'search;
                }
            }
        }

        // Looking at next rows and search for result
        let mut result = 0;
        for y in start_y + 1.. {
            let prev_row = rows.back().unwrap();

            let mut x = *prev_row.start();
            while !check_coordinate(&data, x, y) {
                x += 1;
            }
            let start = x;

            let mut x = *prev_row.end();
            while check_coordinate(&data, x + 1, y) {
                x += 1;
            }
            let end = x;

            // Storing new row
            rows.push_back(start..=end);
            if rows.len() == 101 {
                rows.pop_front();
            } else {
                // Skip until we have 100 rows
                continue;
            }

            let length = *rows.front().unwrap().end() - start + 1;
            if length >= 100 {
                result = start * 10000 + y - 99;
                break;
            }
        }

        result
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
