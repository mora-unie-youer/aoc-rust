use std::collections::HashSet;

use aoc_2022::*;
use itertools::Itertools;

const DAY: i32 = 24;
type Solution = usize;

fn tick(
    blizzards: Vec<(usize, usize, char)>,
    max_x: usize,
    max_y: usize,
) -> Vec<(usize, usize, char)> {
    let mut new_blizzards = Vec::with_capacity(blizzards.len());

    for (mut x, mut y, ch) in blizzards {
        match ch {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y -= 1,
            'v' => y += 1,
            _ => unreachable!(),
        }

        if x == 0 {
            x = max_x - 1;
        } else if x == max_x {
            x = 1;
        }

        if y == 0 {
            y = max_y - 1;
        } else if y == max_y {
            y = 1;
        }

        new_blizzards.push((x, y, ch));
    }

    new_blizzards
}

fn main() {
    let input = get_input_text(DAY);

    let max_x = input.lines().next().unwrap().len() - 1;
    let max_y = input.lines().count() - 1;

    // Start position
    let start = (1, 0);
    // Position where our journey ends (next move must be "win")
    let goal = (max_x - 1, max_y);

    let blizzards = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| !matches!(ch, '.' | '#'))
                .map(move |(x, ch)| (x, y, ch))
        })
        .collect_vec();

    let can_move_to = |x: usize, y: usize, blizzards: &[(usize, usize, char)]| {
        if y == 0 {
            x == 1
        } else if y == max_y {
            x == max_x - 1
        } else {
            x >= 1 && x < max_x && !blizzards.iter().any(|&(bx, by, _)| bx == x && by == y)
        }
    };

    let solution1: Solution = {
        let mut blizzards = blizzards.clone();
        let mut possible_moves = HashSet::new();
        possible_moves.insert(start);

        let mut i = 1;
        'main: loop {
            blizzards = tick(blizzards, max_x, max_y);
            let mut new_possible_moves = HashSet::new();

            for pos @ (x, y) in possible_moves {
                if pos == goal {
                    break 'main;
                }

                if can_move_to(x, y, &blizzards) {
                    new_possible_moves.insert(pos);
                }

                if can_move_to(x - 1, y, &blizzards) {
                    new_possible_moves.insert((x - 1, y));
                }

                if can_move_to(x + 1, y, &blizzards) {
                    new_possible_moves.insert((x + 1, y));
                }

                if y > 0 && can_move_to(x, y - 1, &blizzards) {
                    new_possible_moves.insert((x, y - 1));
                }

                if can_move_to(x, y + 1, &blizzards) {
                    new_possible_moves.insert((x, y + 1));
                }
            }

            possible_moves = new_possible_moves;
            i += 1;
        }

        i
    };

    let solution2: Solution = {
        let mut blizzards = blizzards;
        let mut possible_moves = HashSet::new();
        possible_moves.insert(start);

        let mut i = 0;
        let mut stage = 0;
        'main: loop {
            i += 1;
            blizzards = tick(blizzards, max_x, max_y);
            let mut new_possible_moves = HashSet::new();

            for pos @ (x, y) in possible_moves {
                if can_move_to(x, y, &blizzards) {
                    new_possible_moves.insert(pos);
                }

                if can_move_to(x - 1, y, &blizzards) {
                    new_possible_moves.insert((x - 1, y));
                }

                if can_move_to(x + 1, y, &blizzards) {
                    new_possible_moves.insert((x + 1, y));
                }

                if y > 0 && can_move_to(x, y - 1, &blizzards) {
                    new_possible_moves.insert((x, y - 1));
                }

                if can_move_to(x, y + 1, &blizzards) {
                    new_possible_moves.insert((x, y + 1));
                }
            }

            match stage {
                0 if new_possible_moves.contains(&goal) => {
                    stage = 1;
                    possible_moves = HashSet::from_iter([goal]);
                    continue 'main;
                }
                1 if new_possible_moves.contains(&start) => {
                    stage = 2;
                    possible_moves = HashSet::from_iter([start]);
                    continue 'main;
                }
                2 if new_possible_moves.contains(&goal) => {
                    break 'main;
                }

                _ => possible_moves = new_possible_moves,
            }
        }

        i
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
