use std::cmp::Ordering;

use aoc_2022::*;

const DAY: i32 = 13;
type Solution = usize;

fn compare(left: &[u8], right: &[u8]) -> Ordering {
    match (left[0], right[0]) {
        (a, b) if a == b => compare(&left[1..], &right[1..]),
        (_, b']') => Ordering::Greater,
        (b']', _) => Ordering::Less,
        (b'[', _) => {
            let right = [&[right[0], b']'], &right[1..]].concat();
            compare(&left[1..], &right)
        }
        (_, b'[') => {
            let left = [&[left[0], b']'], &left[1..]].concat();
            compare(&left, &right[1..])
        }
        (a, b) => a.cmp(&b),
    }
}

#[test]
#[rustfmt::skip]
fn test_compare() {
    assert_eq!(compare("[1,1,3,1,1]".as_bytes(), "[1,1,5,1,1]".as_bytes()), Ordering::Less);
    assert_eq!(compare("[[1],[2,3,4]]".as_bytes(), "[[1],4]".as_bytes()), Ordering::Less);
    assert_eq!(compare("[9]".as_bytes(), "[[8,7,6]]".as_bytes()), Ordering::Greater);
    assert_eq!(compare("[[4,4],4,4]".as_bytes(), "[[4,4],4,4,4]".as_bytes()), Ordering::Less);
    assert_eq!(compare("[7,7,7,7]".as_bytes(), "[7,7,7]".as_bytes()), Ordering::Greater);
    assert_eq!(compare("[]".as_bytes(), "[3]".as_bytes()), Ordering::Less);
    assert_eq!(compare("[[[]]]".as_bytes(), "[[]]".as_bytes()), Ordering::Greater);
    assert_eq!(compare("[[1,[2,[3,[4,[5,6,7]]]],8,9]]".as_bytes(), "[1,[2,[3,[4,[5,6,0]]]],8,9]".as_bytes()), Ordering::Greater);
}

fn main() {
    let input = get_input_text(DAY);
    let input = input.replace("10", "A"); // Replace two-digit number with one-char string

    let solution1: Solution = input
        .split("\n\n")
        .map(|pair| {
            let (first, second) = pair.split_once('\n').unwrap();
            let result = compare(first.as_bytes(), second.as_bytes());
            result == Ordering::Less
        })
        .enumerate()
        .filter_map(|(i, v)| match v {
            true => Some(i + 1),
            _ => None,
        })
        .sum();

    let solution2: Solution = {
        let mut input = input.replace("\n\n", "\n");
        input.push_str("[[2]]\n");
        input.push_str("[[6]]");
        let mut packets: Vec<_> = input.lines().collect();
        packets.sort_by(|a, b| compare(a.as_bytes(), b.as_bytes()));
        let first = packets.iter().position(|&p| p == "[[2]]").unwrap() + 1;
        let second = packets.iter().position(|&p| p == "[[6]]").unwrap() + 1;
        first * second
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
