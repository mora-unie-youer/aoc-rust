use aoc_2016::*;
use crypto::{digest::Digest, md5::Md5};

const DAY: i32 = 5;
type Solution = String;

fn main() {
    let input = get_input_text(DAY);
    let door_id = input.trim();

    let mut password1 = String::new();
    let mut password2 = ['-'; 8];

    let mut n = 0;
    let mut found = 0;
    let mut digest = Md5::new();

    // password.len() < 8 && found < 8 -> found < 8 will always be later
    while found < 8 {
        digest.input_str(door_id);
        digest.input_str(&n.to_string());
        let result = digest.result_str();
        digest.reset();

        if result.starts_with("00000") {
            if password1.len() < 8 {
                password1.push(result.chars().nth(5).unwrap());
            }

            let position = result.chars().nth(5).iter().collect::<String>();
            let position = usize::from_str_radix(&position, 16).unwrap();
            if position < 8 && password2[position] == '-' {
                password2[position] = result.chars().nth(6).unwrap();
                found += 1;
            }
        }

        n += 1;
    }

    let solution1: Solution = password1;
    let solution2: Solution = password2.iter().collect();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
