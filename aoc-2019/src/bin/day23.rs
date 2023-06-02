#![feature(array_chunks)]

use aoc_2019::*;

// Took code from day 11
const DAY: i32 = 23;
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
                    let was_empty = self.input.is_empty();
                    let out = self.get_offset(1) as usize;
                    *self.get_value_mut(out) = if was_empty { -1 } else { self.input.remove(0) };
                    self.ip += 2;

                    if self.input.is_empty() {
                        // If there was no input -> pausing
                        break false;
                    }
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

fn main() {
    let input = get_input_text(DAY);
    let data: Vec<_> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let solution1: Solution = {
        let mut programs: Vec<_> = (0..50).map(|v| Program::new(&data, &[v])).collect();

        let mut queue: Vec<Vec<(isize, isize)>> = vec![vec![]; 50];
        'main: loop {
            let mut halted = true;
            for program in &mut programs {
                halted &= program.run();

                // Handle output
                for &[target, x, y] in program.output.array_chunks() {
                    if target == 255 {
                        break 'main y;
                    }

                    queue[target as usize].push((x, y));
                }

                // Clear output buffer
                program.output.clear();
            }

            for (i, program_queue) in queue.iter_mut().enumerate() {
                // Handle input
                for message in program_queue.iter_mut() {
                    let (x, y) = *message;
                    programs[i].input.push(x);
                    programs[i].input.push(y);
                }

                // Clear input queue
                program_queue.clear();
            }

            if halted {
                unreachable!();
            }
        }
    };

    let solution2: Solution = {
        let mut programs: Vec<_> = (0..50).map(|v| Program::new(&data, &[v])).collect();

        let mut queue: Vec<Vec<(isize, isize)>> = vec![vec![]; 50];
        let mut nat_queue: Option<(isize, isize)> = None;
        let mut last_nat = None;
        let mut idle_timer = 0;
        loop {
            let mut halted = true;
            for program in &mut programs {
                halted &= program.run();

                // Handle output
                for &[target, x, y] in program.output.array_chunks() {
                    if target == 255 {
                        nat_queue = Some((x, y));
                    } else {
                        queue[target as usize].push((x, y));
                    }
                }

                // Clear output buffer
                program.output.clear();
            }

            for (i, program_queue) in queue.iter_mut().enumerate() {
                // Handle input
                for message in program_queue.iter_mut() {
                    let (x, y) = *message;
                    programs[i].input.push(x);
                    programs[i].input.push(y);
                }

                // Clear input queue
                program_queue.clear();
            }

            if programs.iter().all(|program| program.input.is_empty()) {
                if idle_timer < 2 {
                    idle_timer += 1;
                } else {
                    let program = &mut programs[0];
                    let (x, y) = nat_queue.unwrap();

                    if last_nat == nat_queue {
                        break y;
                    }

                    last_nat = nat_queue;
                    program.input.push(x);
                    program.input.push(y);
                    idle_timer = 0;
                }
            } else {
                idle_timer = 0;
            }

            if halted {
                break 0;
            }
        }
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
