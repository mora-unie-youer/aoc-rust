use aoc_2016::*;
use crypto::{md5::Md5, digest::Digest};

const DAY: i32 = 5;
type Solution = String;

fn main() {
    let input = get_input_text(DAY);
    let door_id = input.trim();

    let solution1: Solution = {
        let mut n = 0;
        let mut password: String = String::new();
        let mut digest = Md5::new();

        while password.len() < 8 {
            digest.input_str(door_id);
            digest.input_str(&n.to_string());
            let result = digest.result_str();
            digest.reset();

            if result.starts_with("00000") {
                password.push(result.chars().nth(5).unwrap());
            }

            n += 1;
        }

        password
    };

    let solution2: Solution = {
        let mut n = 0;
        let mut found = 0;
        let mut password = ['-'; 8];
        let mut digest = Md5::new();

        while found < 8 {
            digest.input_str(door_id);
            digest.input_str(&n.to_string());
            let result = digest.result_str();
            digest.reset();

            if result.starts_with("00000") {
                let position = result.chars().nth(5).iter().collect::<String>();
                let position = usize::from_str_radix(&position, 16).unwrap();
                if position < 8 && password[position] == '-' {
                    password[position] = result.chars().nth(6).unwrap();
                    found += 1;
                }
            }

            n += 1;
        }

        password.iter().collect()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
