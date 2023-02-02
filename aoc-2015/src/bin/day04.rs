use aoc_2015::*;
use crypto::{digest::Digest, md5::Md5};

const DAY: i32 = 4;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let input = input.trim();

    let solution1: Solution = {
        let mut number = 0;
        let mut digest = Md5::new();
        let mut result = [0; 16];

        loop {
            let s = format!("{}{}", input, number);
            digest.input(s.as_bytes());
            digest.result(&mut result);
            digest.reset();
            if result[0] == 0 && result[1] == 0 && result[2] <= 0xf {
                break;
            }

            number += 1;
        }

        number
    };

    let solution2: Solution = {
        let mut number = 0;
        let mut digest = Md5::new();
        let mut result = [0; 16];

        loop {
            let s = format!("{}{}", input, number);
            digest.input(s.as_bytes());
            digest.result(&mut result);
            digest.reset();
            if result[0] == 0 && result[1] == 0 && result[2] == 0 {
                break;
            }

            number += 1;
        }

        number
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
