use aoc_2017::*;

const DAY: i32 = 5;
type Solution = usize;

struct Cpu {
    // Instruction pointer
    ip: usize,
    // Instructions
    instructions: Vec<isize>,
    // Is part2
    part2: bool
}

impl Cpu {
    fn new(instructions: Vec<isize>, part2: bool) -> Self {
        Self {
            ip: 0,
            instructions,
            part2,
        }
    }

    fn run(&mut self) -> Solution {
        let mut steps = 0;
        while self.ip < self.instructions.len() {
            self.execute_instruction();
            steps += 1;
        }
        steps
    }

    fn execute_instruction(&mut self) {
        let current_instruction = &mut self.instructions[self.ip];
        self.ip = (self.ip as isize + *current_instruction) as usize;

        if self.part2 && *current_instruction >= 3 {
            *current_instruction -= 1;
        } else {
            *current_instruction += 1;
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let instructions: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();

    let solution1: Solution = {
        let mut cpu = Cpu::new(instructions.clone(), false);
        cpu.run()
    };

    let solution2: Solution = {
        let mut cpu = Cpu::new(instructions, true);
        cpu.run()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
