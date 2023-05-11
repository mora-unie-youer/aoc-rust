use aoc_2019::*;

const DAY: i32 = 2;
type Solution = usize;

struct Program {
    data: Vec<usize>,
}

impl Program {
    fn run(&mut self) {
        let mut offset = 0;

        loop {
            let command = &self.data[offset..offset + 4];
            match command[0] {
                1 => {
                    let op1 = self.data[command[1]];
                    let op2 = self.data[command[2]];
                    let out = command[3];
                    self.data[out] = op1 + op2;
                }
                2 => {
                    let op1 = self.data[command[1]];
                    let op2 = self.data[command[2]];
                    let out = command[3];
                    self.data[out] = op1 * op2;
                }
                99 => break,
                _ => panic!("Something went wrong"),
            }

            offset += 4;
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
        let mut program = Program { data: data.clone() };
        program.data[1] = 12;
        program.data[2] = 2;
        program.run();
        program.data[0]
    };

    let solution2: Solution = {
        let mut result = (0, 0);

        'main: for noun in 0..100 {
            for verb in 0..100 {
                let mut program = Program { data: data.clone() };
                program.data[1] = noun;
                program.data[2] = verb;
                program.run();

                if program.data[0] == 19690720 {
                    result = (noun, verb);
                    break 'main;
                }
            }
        }

        result.0 * 100 + result.1
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
