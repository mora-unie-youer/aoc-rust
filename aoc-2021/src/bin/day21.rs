use std::collections::HashMap;

use aoc_2021::*;
use itertools::Itertools;

const DAY: i32 = 21;
type Solution = usize;

type Player = (usize, usize);
type State = (bool, Player, Player);
type Wins = (usize, usize);
fn solve_part2(
    step_count: usize,
    player1: Player,
    player2: Player,
    visited: &mut HashMap<State, Wins>,
    rolls: &[(usize, usize)],
) -> Wins {
    let state = (step_count % 2 == 0, player1, player2);
    if visited.contains_key(&state) {
        return visited[&state];
    }

    let (player1_score, player2_score) = (player1.1, player2.1);
    if player1_score >= 21 || player2_score >= 21 {
        return (
            (player1_score > player2_score) as usize,
            (player1_score < player2_score) as usize,
        );
    }

    let (mut wins1, mut wins2) = (0, 0);
    if state.0 {
        let (pos, score) = player1;
        for &(step, count) in rolls {
            let new_pos = (pos + step) % 10;
            let new_score = score + new_pos + 1;

            let new_state = (!state.0, (new_pos, new_score), player2);
            let wins = solve_part2(step_count + 1, new_state.1, new_state.2, visited, rolls);

            wins1 += count * wins.0;
            wins2 += count * wins.1;
        }
    } else {
        let (pos, score) = player2;
        for &(step, count) in rolls {
            let new_pos = (pos + step) % 10;
            let new_score = score + new_pos + 1;

            let new_state = (!state.0, player1, (new_pos, new_score));
            let wins = solve_part2(step_count + 1, new_state.1, new_state.2, visited, rolls);

            wins1 += count * wins.0;
            wins2 += count * wins.1;
        }
    }

    visited.insert(state, (wins1, wins2));
    (wins1, wins2)
}

fn main() {
    let input = get_input_text(DAY);

    let (player1, player2) = input.trim().split_once('\n').unwrap();
    let player1: usize = player1.rsplit_once(' ').unwrap().1.parse().unwrap();
    let player2: usize = player2.rsplit_once(' ').unwrap().1.parse().unwrap();

    let solution1: Solution = {
        let (mut player1_pos, mut player2_pos) = (player1 - 1, player2 - 1);
        let (mut player1_score, mut player2_score) = (0, 0);
        let mut dice = (1..=100).cycle();

        let mut iteration = 0;
        while player1_score < 1000 && player2_score < 1000 {
            let (a, b, c) = (
                dice.next().unwrap(),
                dice.next().unwrap(),
                dice.next().unwrap(),
            );
            let step = a + b + c;

            if iteration % 2 == 0 {
                // First player move
                player1_pos = (player1_pos + step) % 10;
                player1_score += player1_pos + 1;
            } else {
                // Second player move
                player2_pos = (player2_pos + step) % 10;
                player2_score += player2_pos + 1;
            }

            iteration += 1;
        }

        let dice_rolls = iteration * 3;
        let losing_player = player1_score.min(player2_score);
        losing_player * dice_rolls
    };

    let solution2: Solution = {
        const ROLLS: [usize; 9] = [1, 1, 1, 2, 2, 2, 3, 3, 3];

        let groups = ROLLS
            .into_iter()
            .permutations(3)
            .sorted()
            .dedup()
            .map(|v| v.into_iter().sum::<usize>())
            .sorted()
            .group_by(|&v| v);

        let mut rolls = Vec::new();
        for (step, steps) in &groups {
            rolls.push((step, steps.count()));
        }

        let (player1, player2) = solve_part2(
            0,
            (player1 - 1, 0),
            (player2 - 1, 0),
            &mut HashMap::new(),
            &rolls,
        );
        player1.max(player2)
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
