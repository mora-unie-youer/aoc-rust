use std::collections::{HashSet, VecDeque};

use aoc_2020::*;

const DAY: i32 = 22;
type Solution = usize;

fn player_score(player: &VecDeque<usize>) -> usize {
    player
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v)
        .sum()
}

fn game(player1: &mut VecDeque<usize>, player2: &mut VecDeque<usize>, part2: bool) -> bool {
    let mut states = HashSet::new();

    while !player1.is_empty() && !player2.is_empty() {
        if part2 && !states.insert((player_score(player1), player_score(player2))) {
            // If this card state already was in the game -> player 1 win
            return false;
        }

        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        if part2 && card1 <= player1.len() && card2 <= player2.len() {
            let mut new_player1 = player1.iter().take(card1).cloned().collect();
            let mut new_player2 = player2.iter().take(card2).cloned().collect();
            if game(&mut new_player1, &mut new_player2, part2) {
                // Player 2 won
                player2.push_back(card2);
                player2.push_back(card1);
            } else {
                // Player 1 won
                player1.push_back(card1);
                player1.push_back(card2);
            }
        } else if card1 > card2 {
            // Player 1 won
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            // Player 2 won
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }

    // If player 2 won -> true, otherwise -> false
    player1.is_empty()
}

fn solve(mut player1: VecDeque<usize>, mut player2: VecDeque<usize>, part2: bool) -> Solution {
    let winner = game(&mut player1, &mut player2, part2);
    let winner = if winner { player2 } else { player1 };

    winner
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v)
        .sum()
}

fn main() {
    let input = get_input_text(DAY);
    let (player1, player2) = input.trim().split_once("\n\n").unwrap();

    let player1: VecDeque<usize> = player1
        .lines()
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect();
    let player2: VecDeque<usize> = player2
        .lines()
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect();

    let solution1: Solution = solve(player1.clone(), player2.clone(), false);
    let solution2: Solution = solve(player1, player2, true);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
