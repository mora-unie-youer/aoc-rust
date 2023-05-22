#![feature(array_chunks)]

use std::collections::HashMap;

use aoc_2019::*;

// Took code from day 11
const DAY: i32 = 13;
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

const BLOCK: isize = 2;
const PADDLE: isize = 3;
const BALL: isize = 4;
fn main() {
    let input = get_input_text(DAY);
    let data: Vec<_> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let solution1: Solution = {
        let mut program = Program::new(&data, &[]);
        program.run();

        program
            .output
            .iter()
            .skip(2)
            .step_by(3)
            .filter(|&&tile| tile == BLOCK)
            .count() as _
    };

    let solution2: Solution = {
        let mut data = data;
        data[0] = 2;

        let mut program = Program::new(&data, &[]);
        program.run(); // Fetching tiles
        let mut tiles: HashMap<(isize, isize), isize> = program
            .output
            .array_chunks()
            .map(|&[x, y, tile]| ((x, y), tile))
            .collect();
        program.output.clear();

        // This is Pong game, as I found out from output
        // We need to move paddle (joystick) to correspond ball coordinate
        let mut score = 0;
        loop {
            let paddle = tiles.iter().find(|&(_, &tile)| tile == PADDLE).unwrap();
            let ball = tiles.iter().find(|&(_, &tile)| tile == BALL).unwrap();

            // Calculating input and running program further
            let ((paddle_x, _), (ball_x, _)) = (paddle.0, ball.0);
            let direction = (ball_x - paddle_x).signum();
            program.input.push(direction);
            let halt = program.run();

            // Processing program output
            for &[x, y, tile] in program.output.array_chunks() {
                if x == -1 && y == 0 {
                    score = tile;
                } else {
                    tiles.insert((x, y), tile);
                }
            }

            // If program halted, break the loop
            if halt {
                break;
            }

            // Clear program output
            program.output.clear();
        }

        score
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
