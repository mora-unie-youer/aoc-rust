use aoc_2018::*;

const DAY: i32 = 5;
type Solution = usize;

fn are_opposite(a: char, b: char) -> bool {
    // Some hack -> 0x41 ^ 0x61 == 0b00100000
    (a as u8) ^ (b as u8) == 0b00100000
}

fn react(input: &str) -> String {
    input.chars().fold(String::new(), |mut acc, ch| {
        if let Some(end) = acc.pop() {
            if !are_opposite(ch, end) {
                acc.push(end);
                acc.push(ch);
            }
        } else {
            acc.push(ch);
        }

        acc
    })
}

fn full_react(mut input: String) -> String {
    loop {
        let new_input = react(&input);
        if input.len() == new_input.len() {
            break input;
        }

        input = new_input;
    }
}

fn main() {
    let input = get_input_text(DAY);
    let input = input.trim().to_owned();

    let solution1: Solution = full_react(input.clone()).len();
    let solution2: Solution = {
        let mut min_length = std::usize::MAX;

        for rch in 'a'..='z' {
            let upper_rch = rch.to_uppercase().next().unwrap();
            let input: String = input
                .chars()
                .filter(|&ch| ch != rch && ch != upper_rch)
                .collect();
            min_length = min_length.min(full_react(input).len());
        }

        min_length
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
