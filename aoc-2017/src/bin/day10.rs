#![feature(array_chunks)]

use aoc_2017::*;

const DAY: i32 = 10;
type Solution = String;

fn reverse_sublist(list: &mut [usize], start: usize, length: usize) {
    let len = list.len();
    let end = (start + length - 1) % len;
    for i in 0..length / 2 {
        let a = (start + i) % len;
        let b = (end + len - i) % len;
        list.swap(a, b);
    }
}

fn knot_hash(input: &str) -> Solution {
    let lengths: Vec<usize> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let mut list: Vec<_> = (0..=255).collect();
    let mut current_pos = 0;
    let mut skip_size = 0;
    for length in lengths {
        reverse_sublist(&mut list, current_pos, length);
        current_pos = (current_pos + length + skip_size) % list.len();
        skip_size += 1;
    }

    (list[0] * list[1]).to_string()
}

fn hard_hash(input: &str) -> Solution {
    let mut lengths = input.trim().as_bytes().to_vec();
    lengths.extend([17, 31, 73, 47, 23]);

    let mut current_pos = 0;
    let mut list: Vec<_> = (0..=255).collect();
    let mut skip_size = 0;

    for _ in 0..64 {
        for &length in lengths.iter() {
            reverse_sublist(&mut list, current_pos, length as usize);
            current_pos = (current_pos + length as usize + skip_size) % list.len();
            skip_size += 1;
        }
    };

    let sparse_hash = list;
    let dense_hash: Vec<_> = sparse_hash
        .array_chunks()
        .map(|nums: &[usize; 16]| nums.iter().fold(0, |acc, v| acc ^ v))
        .collect();

    dense_hash.iter().map(|&b| format!("{:02x}", b)).collect()
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = knot_hash(&input);
    let solution2: Solution = hard_hash(&input);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::hard_hash;

    #[test]
    fn test_hard_hash() {
        assert_eq!(hard_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(hard_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(hard_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(hard_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
