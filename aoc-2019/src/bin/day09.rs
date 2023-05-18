use aoc_2019::*;

// Took code from day 7
const DAY: i32 = 9;
type Solution = String;

#[derive(Clone, Debug)]
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

    fn run(&mut self) {
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
                        break;
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
                99 => break,
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

fn solve(input: &str, part2: bool) -> Solution {
    let data: Vec<_> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let mut program = Program::new(&data, &[if part2 { 2 } else { 1 }]);
    program.run();
    format!("{:?}", program.output)
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
    fn test_solve() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        assert_eq!(
            solve(input, false),
            "[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]"
        );
        let input = "1102,34915192,34915192,7,4,7,99,0";
        assert_eq!(solve(input, false).len(), 18);
        let input = "104,1125899906842624,99";
        assert_eq!(solve(input, false), "[1125899906842624]");
    }
}
