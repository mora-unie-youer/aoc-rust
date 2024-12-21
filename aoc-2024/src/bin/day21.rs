use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use aoc_2024::*;
use itertools::Itertools;

const DAY: i32 = 21;
type Solution = usize;

fn robot(
    pad: &HashMap<char, (isize, isize)>,
    moves: &[char],
    start: (isize, isize),
) -> Vec<Vec<char>> {
    let pad_positions: HashSet<_> = pad.values().collect();

    let mut new_moves = vec![];
    let mut current_position = start;
    for ch in moves {
        let (cx, cy) = current_position;
        let (x, y) = *pad.get(ch).unwrap();

        let (dx, dy) = (x - cx, y - cy);

        let vertical = std::iter::once(if dy > 0 { 'v' } else { '^' })
            .cycle()
            .take(dy.unsigned_abs());
        let horizontal = std::iter::once(if dx > 0 { '>' } else { '<' })
            .cycle()
            .take(dx.unsigned_abs());

        let mut step = vec![];
        if dx > 0 && pad_positions.contains(&(cx, y)) {
            step.extend(vertical);
            step.extend(horizontal);
        } else if pad_positions.contains(&(x, cy)) {
            step.extend(horizontal);
            step.extend(vertical);
        } else {
            step.extend(vertical);
            step.extend(horizontal);
        }

        step.push('A');
        new_moves.push(step);
        current_position = (x, y);
    }

    new_moves
}

fn code_robot(code: &[char]) -> Vec<Vec<char>> {
    // NOTE: could use hf_map here to be compile-time, but who cares
    static PAD: LazyLock<HashMap<char, (isize, isize)>> = LazyLock::new(|| {
        HashMap::from_iter([
            // ch, (x, y)
            ('7', (0, 0)),
            ('8', (1, 0)),
            ('9', (2, 0)),
            ('4', (0, 1)),
            ('5', (1, 1)),
            ('6', (2, 1)),
            ('1', (0, 2)),
            ('2', (1, 2)),
            ('3', (2, 2)),
            ('0', (1, 3)),
            ('A', (2, 3)),
        ])
    });

    robot(&PAD, code, (2, 3))
}

fn movement_robot(moves: &[char]) -> Vec<Vec<char>> {
    static PAD: LazyLock<HashMap<char, (isize, isize)>> = LazyLock::new(|| {
        HashMap::from_iter([
            // ch, (x, y)
            ('^', (1, 0)),
            ('A', (2, 0)),
            ('<', (0, 1)),
            ('v', (1, 1)),
            ('>', (2, 1)),
        ])
    });

    robot(&PAD, moves, (2, 0))
}

fn solve<const C: usize>(line: &str) -> Solution {
    let number: usize = line[..3].parse().unwrap();
    let code = line.chars().collect_vec();

    let mut moves: HashMap<Vec<char>, usize> =
        code_robot(&code).into_iter().map(|v| (v, 1)).collect();

    for _ in 0..C {
        let mut new_moves = HashMap::new();

        for (m, count) in moves {
            for new_move in movement_robot(&m) {
                *new_moves.entry(new_move).or_default() += count;
            }
        }

        moves = new_moves;
    }

    number * moves.into_iter().map(|(k, v)| k.len() * v).sum::<usize>()
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input.lines().map(solve::<2>).sum();
    let solution2: Solution = input.lines().map(solve::<25>).sum();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
