use aoc_2017::*;

const DAY: i32 = 17;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let steps: usize = input.trim().parse().unwrap();

    let solution1: Solution = {
        let mut buffer = vec![0];
        let mut position = 0;
        // 0..2017
        while buffer.len() < 2018 {
            position = (position + steps) % buffer.len() + 1;
            buffer.insert(position, buffer.len());
        }

        let i = buffer.iter().position(|&v| v == 2017).unwrap();
        buffer[i + 1]
    };

    let solution2: Solution = {
        let mut length = 1;
        let mut position = 0;
        let mut result = 0;
        for n in 1..=50_000_000 {
            position = (position + steps) % length + 1;
            length += 1;

            if position == 1 {
                result = n;
            }
        }

        result
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
