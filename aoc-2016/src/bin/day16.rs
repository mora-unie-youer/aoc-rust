#![feature(iter_array_chunks)]

use aoc_2016::*;

const DAY: i32 = 16;
type Solution = String;

trait Flip {
    fn flip(&self) -> Self;
}

impl Flip for char {
    fn flip(&self) -> Self {
        if *self == '0' {
            '1'
        } else {
            '0'
        }
    }
}

trait DragonCurve {
    fn dragon_curve(&self) -> Self;
}

impl DragonCurve for String {
    fn dragon_curve(&self) -> Self {
        let mut result = self.clone();
        let part2: String = self.chars().map(|ch| ch.flip()).rev().collect();
        result.push('0');
        result.push_str(&part2);
        result
    }
}

trait Checksum {
    fn checksum(&self, data_length: usize) -> Self;
}

impl Checksum for String {
    fn checksum(&self, data_length: usize) -> Self {
        let mut result = self[..data_length].to_string();
        while result.len() % 2 == 0 {
            result = result
                .chars()
                .array_chunks()
                .map(|[a, b]| if a == b { '1' } else { '0' })
                .collect();
        }

        result
    }
}

fn solve(mut s: String, data_length: usize) -> Solution {
    while s.len() < data_length {
        s = s.dragon_curve();
    }

    s.checksum(data_length)
}

fn main() {
    let input = get_input_text(DAY);
    let input = input.trim().to_string();

    let solution1: Solution = solve(input.clone(), 272);
    let solution2: Solution = solve(input, 35651584);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::{Checksum, DragonCurve};

    #[test]
    fn test_dragon_curve() {
        assert_eq!("1".to_string().dragon_curve(), "100");
        assert_eq!("0".to_string().dragon_curve(), "001");
        assert_eq!("11111".to_string().dragon_curve(), "11111000000");
        assert_eq!(
            "111100001010".to_string().dragon_curve(),
            "1111000010100101011110000"
        );
    }

    #[test]
    fn test_checksum() {
        assert_eq!("110010110100".to_string().checksum(12), "100");
        assert_eq!("10000011110010000111110".to_string().checksum(20), "01100");
    }
}
