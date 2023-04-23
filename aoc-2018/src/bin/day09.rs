use std::collections::VecDeque;

use aoc_2018::*;

const DAY: i32 = 9;
type Solution = usize;

fn solve(players: usize, marbles: usize) -> Solution {
    let mut scores = vec![0; players];
    let mut circle = VecDeque::with_capacity(marbles);
    circle.push_back(0);

    for marble in 1..=marbles {
        if marble % 23 == 0 {
            let player = (marble - 1) % players;
            circle.rotate_right(7);
            scores[player] += marble;
            scores[player] += circle.pop_back().unwrap();
            circle.rotate_left(1);
        } else {
            circle.rotate_left(1);
            circle.push_back(marble);
        }
    }

    scores.into_iter().max().unwrap()
}

fn main() {
    let input = get_input_text(DAY);
    let (players, marbles) = {
        let mut parts = input.split_ascii_whitespace();
        let players = parts.next().unwrap();
        let marbles = parts.nth(5).unwrap();
        (players.parse().unwrap(), marbles.parse().unwrap())
    };

    let solution1: Solution = solve(players, marbles);
    let solution2: Solution = solve(players, marbles * 100);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(9, 25), 32);
        assert_eq!(solve(10, 1618), 8317);
        assert_eq!(solve(13, 7999), 146373);
        assert_eq!(solve(17, 1104), 2764);
        assert_eq!(solve(21, 6111), 54718);
        assert_eq!(solve(30, 5807), 37305);
    }
}
