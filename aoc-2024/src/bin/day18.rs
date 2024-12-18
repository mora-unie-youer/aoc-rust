use std::collections::HashSet;

use aoc_2024::*;
use pathfinding::directed::bfs;

const DAY: i32 = 18;
type Solution = String;

fn walk(fallen: &HashSet<(isize, isize)>) -> Option<Vec<(isize, isize)>> {
    const START: (isize, isize) = (0, 0);
    const END: (isize, isize) = (70, 70);

    bfs::bfs(
        &START,
        |&(x, y)| {
            let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

            neighbors
                .into_iter()
                .filter(|&(x, y)| x >= 0 && y >= 0 && x <= END.0 && y <= END.1)
                .filter(|&(x, y)| !fallen.contains(&(x, y)))
        },
        |pos| pos == &END,
    )
}

fn main() {
    let input = get_input_text(DAY);

    let fallen: Vec<(isize, isize)> = input
        .lines()
        .flat_map(|line| line.split_once(','))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let solution1: Solution = {
        let fallen: HashSet<_> = fallen.iter().take(1024).copied().collect();
        let steps = walk(&fallen).unwrap();
        (steps.len() - 1).to_string()
    };

    let solution2: Solution = {
        let (mut low, mut high) = (1024, fallen.len());
        while low < high {
            let mid = (low + high + 1) / 2;

            let fallen: HashSet<_> = fallen.iter().take(mid).copied().collect();
            match walk(&fallen) {
                Some(_) => low = mid,
                None => high = mid - 1,
            }
        }

        let (x, y) = fallen[low];
        format!("{x},{y}")
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
