use aoc_2024::*;
use itertools::Itertools;

const DAY: i32 = 17;
type Solution = String;

fn run(program: &[usize], mut a: usize, mut b: usize, mut c: usize) -> Vec<usize> {
    let size = program.len();
    let mut ip = 0;
    let mut output = Vec::new();
    while ip < size {
        let opcode = program[ip];

        let literal = program[ip + 1];
        let combo = match literal {
            v @ 0..=3 => v,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable!(),
        };

        match opcode {
            0 => a /= 2usize.pow(combo as u32),
            1 => b ^= literal,
            2 => b = combo % 8,

            3 => {
                if a != 0 {
                    ip = literal;
                    continue;
                }
            }

            4 => b ^= c,
            5 => output.push(combo % 8),
            6 => b = a / 2usize.pow(combo as u32),
            7 => c = a / 2usize.pow(combo as u32),
            _ => unreachable!(),
        }

        ip += 2;
    }

    output
}

fn main() {
    let input = get_input_text(DAY);

    let (regs, program) = input.split_once("\n\n").unwrap();
    let mut regs = regs
        .lines()
        .flat_map(|line| line.split_ascii_whitespace().nth(2))
        .map(|v| v.parse().unwrap());
    let a: usize = regs.next().unwrap();
    let b = regs.next().unwrap();
    let c = regs.next().unwrap();

    let program_str = program.trim().split_once(' ').unwrap().1;
    let program: Vec<usize> = program_str.split(',').map(|v| v.parse().unwrap()).collect();

    let solution1: Solution = run(&program, a, b, c)
        .into_iter()
        .map(|v| v.to_string())
        .join(",");

    let solution2: Solution = {
        // Here we need to reverse program in input
        //
        // This is what I got on input:
        // start:
        //   b = a % 8
        //   b = b ^ 1
        //   c = a >> b
        //   b = b ^ 5
        //   b = b ^ c
        //   a = a >> 3
        //   out.push(b % 8)
        //   if a != 0 {
        //     goto start
        //   }
        //
        // Now we can to reverse the result to get required A
        // NOTE: i couldn't figure out how to reverse it properly,
        // bruteforcing it by 3-bit

        let mut valid = vec![0];
        for (i, _) in program.iter().enumerate().rev() {
            let mut next = vec![];

            for a in valid {
                for x in 0..8 {
                    let na = (a << 3) | x;
                    let result = run(&program, na, b, c);

                    if result[..] == program[i..] {
                        next.push(na);
                    }
                }
            }

            valid = next;
        }

        let answer = valid.into_iter().min().unwrap();
        answer.to_string()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
