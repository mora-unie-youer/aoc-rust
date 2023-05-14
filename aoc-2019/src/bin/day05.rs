use aoc_2019::*;

// Took code from day 2
const DAY: i32 = 5;
type Solution = String;

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

fn main() {
    let input = get_input_text(DAY);
    dbg!(&input);
    let data: Vec<_> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let solution1: Solution = {
        let mut program = Program::new(&data, &[1]);
        program.run();
        format!("{:?}", program.output)
    };

    let solution2: Solution = {
        let mut program = Program::new(&data, &[5]);
        program.run();
        format!("{:?}", program.output)
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
