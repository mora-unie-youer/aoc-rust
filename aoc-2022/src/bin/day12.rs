use aoc_2022::*;
use itertools::Itertools;
use pathfinding::{directed::bfs, prelude::Matrix};

const DAY: i32 = 12;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let map = Matrix::from_rows(input.lines().map(|line| {
        line.bytes().map(|ch| match ch {
            b'S' => b'a' - 1,
            b'E' => b'z' + 1,
            v => v,
        })
    }))
    .unwrap();

    let (start, end) = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, cell)| (i, j, cell)))
        .minmax_by_key(|&(_, _, &cell)| cell)
        .into_option()
        .map(|((miny, minx, _), (maxy, maxx, _))| ((miny, minx), (maxy, maxx)))
        .unwrap();

    let solution1: Solution = {
        let path = bfs::bfs(
            &start,
            |&pos| {
                map.neighbours(pos, false)
                    .filter(|&p| map[p] <= 1 + map[pos])
                    .collect::<Vec<_>>()
            },
            |&pos| pos == end,
        )
        .unwrap();

        path.len() - 1
    };

    let solution2: Solution = {
        let path = bfs::bfs(
            &end,
            |&pos| {
                map.neighbours(pos, false)
                    .filter(|&p| 1 + map[p] >= map[pos])
                    .collect::<Vec<_>>()
            },
            |&pos| map[pos] == b'a',
        )
        .unwrap();

        path.len() - 1
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
