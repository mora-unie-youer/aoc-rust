use aoc_2021::*;
use pathfinding::{directed::dijkstra, prelude::Matrix};

const DAY: i32 = 15;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = {
        let rows = input
            .lines()
            .map(|line| line.chars().map(|ch| (ch as u8 - b'0') as usize));
        let map: Matrix<usize> = Matrix::from_rows(rows).unwrap();

        let start = (0, 0);
        let end = (map.rows - 1, map.columns - 1);
        let (_, cost) = dijkstra::dijkstra(
            &start,
            |&pos| map.neighbours(pos, false).map(|pos| (pos, map[pos])),
            |&pos| pos == end,
        )
        .unwrap();

        cost
    };

    let solution2: Solution = {
        let rows: Vec<Vec<usize>> = input
            .lines()
            .map(|line| line.chars().map(|ch| (ch as u8 - b'0') as usize).collect())
            .collect();
        let mut map = vec![vec![0; rows.len() * 5]; rows[0].len() * 5];

        for cy in 0..5 {
            for cx in 0..5 {
                let offset_y = cy * rows.len();
                let offset_x = cx * rows[0].len();
                let increase = cy + cx;

                for y in 0..rows.len() {
                    for x in 0..rows[0].len() {
                        let value = rows[y][x] + increase;
                        map[offset_y + y][offset_x + x] = if value > 9 { value - 9 } else { value };
                    }
                }
            }
        }

        let map = Matrix::from_rows(map).unwrap();
        let start = (0, 0);
        let end = (map.rows - 1, map.columns - 1);
        let (_, cost) = dijkstra::dijkstra(
            &start,
            |&pos| map.neighbours(pos, false).map(|pos| (pos, map[pos])),
            |&pos| pos == end,
        )
        .unwrap();

        cost
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
