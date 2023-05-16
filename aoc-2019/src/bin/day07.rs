use std::collections::HashSet;

use aoc_2019::*;
use itertools::Itertools;

// Took code from day 5
const DAY: i32 = 7;
type Solution = isize;

#[derive(Clone, Debug)]
struct Program {
    ip: isize,
    data: Vec<isize>,

    input: Vec<isize>,
    output: Vec<isize>,
}

impl Program {
    fn new(data: &[isize], input: &[isize]) -> Self {
        Self {
            ip: 0,
            data: data.to_vec(),

            input: input.to_vec(),
            output: Vec::new(),
        }
    }

    fn run(&mut self) {
        loop {
            let opcode = self.get_offset(0) % 100;
            match opcode {
                1 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    let out = self.get_offset(3) as usize;
                    self.data[out] = op1 + op2;
                    self.ip += 4;
                }
                2 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    let out = self.get_offset(3) as usize;
                    self.data[out] = op1 * op2;
                    self.ip += 4;
                }
                3 => {
                    if self.input.is_empty() {
                        // If there's no input -> pausing
                        break;
                    }

                    let out = self.get_offset(1) as usize;
                    self.data[out] = self.input.remove(0);
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
                    self.data[out] = (op1 < op2) as isize;
                    self.ip += 4;
                }
                8 => {
                    let op1 = self.get_parameter(1);
                    let op2 = self.get_parameter(2);
                    let out = self.get_offset(3) as usize;
                    self.data[out] = (op1 == op2) as isize;
                    self.ip += 4;
                }
                99 => break,
                _ => panic!("Something went wrong"),
            }
        }
    }

    fn get_offset(&self, offset: isize) -> isize {
        self.data[(self.ip + offset) as usize]
    }

    fn get_parameter(&self, parameter: isize) -> isize {
        let mode = self.data[self.ip as usize]
            .to_string()
            .chars()
            .rev()
            .nth((parameter + 1) as usize)
            .unwrap_or('0')
            .to_digit(10)
            .unwrap();

        let parameter = self.get_offset(parameter);
        match mode {
            0 => self.data[parameter as usize],
            1 => parameter,
            _ => unreachable!(),
        }
    }
}

fn solve(input: &str, part2: bool) -> Solution {
    let data: Vec<_> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    if !part2 {
        (0..5)
            .permutations(5)
            .map(|configuration| {
                configuration
                    .into_iter()
                    .map(|phase| Program::new(&data, &[phase, 0]))
                    .fold(0, |prev, mut program| {
                        program.input[1] = prev;
                        program.run();
                        program.output[0]
                    })
            })
            .max()
            .unwrap()
    } else {
        (5..10)
            .permutations(5)
            .map(|configuration| {
                let mut programs: Vec<_> = configuration
                    .into_iter()
                    .map(|phase| Program::new(&data, &[phase]))
                    .collect();

                let mut values = HashSet::new();
                let mut last_output = 0;
                loop {
                    let new_output = programs.iter_mut().fold(last_output, |prev, program| {
                        program.input.push(prev);
                        program.run();
                        *program.output.last().unwrap()
                    });

                    if !values.insert(new_output) {
                        break;
                    }

                    last_output = new_output;
                }

                values.into_iter().max().unwrap()
            })
            .max()
            .unwrap()
    }
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = solve(&input, false);
    let solution2: Solution = solve(&input, true);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_part1() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        assert_eq!(solve(input, false), 43210);
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        assert_eq!(solve(input, false), 54321);
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        assert_eq!(solve(input, false), 65210);
    }

    #[test]
    fn test_part2() {
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        assert_eq!(solve(input, true), 139629729);
        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        assert_eq!(solve(input, true), 18216);
    }
}
