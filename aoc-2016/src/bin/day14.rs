#![feature(iter_array_chunks)]

use std::collections::VecDeque;

use aoc_2016::*;
use crypto::{digest::Digest, md5::Md5};

const DAY: i32 = 14;
type Solution = usize;

trait HasNChars {
    fn has3chars(&self) -> Option<char>;
    fn has5chars(&self, ch: char) -> bool;
}

impl HasNChars for String {
    fn has3chars(&self) -> Option<char> {
        self.chars().collect::<Vec<_>>().windows(3).find_map(|c| {
            if c[0] == c[1] && c[1] == c[2] {
                Some(c[0])
            } else {
                None
            }
        })
    }

    fn has5chars(&self, ch: char) -> bool {
        self.chars()
            .collect::<Vec<_>>()
            .windows(5)
            .any(|chars| chars.iter().all(|&c| c == ch))
    }
}

fn get_hash(salt: &str, n: usize, rounds: usize, digest: &mut Md5) -> String {
    let mut input = format!("{}{}", salt, n);
    for _ in 0..rounds {
        digest.input_str(&input);
        input = digest.result_str();
        digest.reset();
    }

    input
}

fn solve(salt: &str, rounds: usize) -> usize {
    let mut digest = Md5::new();
    let mut hashes: VecDeque<_> = (0..1000)
        .map(|i| get_hash(salt, i, rounds, &mut digest))
        .collect();

    let mut n = 0;
    let mut key = 0;
    while key < 64 {
        let current_hash = hashes.pop_front().unwrap();
        // Generate next hash
        hashes.push_back(get_hash(salt, n + 1000, rounds, &mut digest));

        // Checking if current hash has 3 chars
        if let Some(ch) = current_hash.has3chars() {
            if hashes.iter().any(|hash| hash.has5chars(ch)) {
                key += 1;
            }
        }

        // Moving to the next hash
        n += 1;
    }

    n - 1
}

fn main() {
    let input = get_input_text(DAY);
    let salt = input.trim();

    let solution1: Solution = solve(salt, 1);
    let solution2: Solution = solve(salt, 1 + 2016);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
