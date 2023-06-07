use aoc_2020::*;

const DAY: i32 = 3;
type Solution = usize;

fn collisions(map: &[Vec<bool>], dx: usize, dy: usize) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut count = 0;

    while y < map.len() {
        count += map[y][x] as usize;
        x = (x + dx) % map[0].len();
        y += dy;
    }

    count
}

fn main() {
    let input = get_input_text(DAY);
    let map: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch == '#').collect())
        .collect();

    let solution1: Solution = collisions(&map, 3, 1);
    let solution2: Solution = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .into_iter()
        .map(|(dy, dx)| collisions(&map, dx, dy))
        .product();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
