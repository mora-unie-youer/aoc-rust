use std::cmp::Ordering;

use aoc_2024::*;
use itertools::Itertools;

const DAY: i32 = 14;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    dbg!(&input);

    let robots = input.lines().map(|line| {
        let mut parts = line.split([' ', '=', ',']);
        let px: isize = parts.nth(1).unwrap().parse().unwrap();
        let py: isize = parts.next().unwrap().parse().unwrap();
        let vx: isize = parts.nth(1).unwrap().parse().unwrap();
        let vy: isize = parts.next().unwrap().parse().unwrap();
        (px, py, vx, vy)
    });

    const WIDTH: isize = 101;
    const HEIGHT: isize = 103;
    let solution1: Solution = robots
        .clone()
        .map(|(px, py, vx, vy)| (px + vx * 100, py + vy * 100))
        .map(|(px, py)| (px.rem_euclid(WIDTH), py.rem_euclid(HEIGHT)))
        .map(
            |(px, py)| match ((WIDTH / 2).cmp(&px), (HEIGHT / 2).cmp(&py)) {
                (Ordering::Greater, Ordering::Greater) => (1, px, py),
                (Ordering::Less, Ordering::Greater) => (2, px, py),
                (Ordering::Greater, Ordering::Less) => (3, px, py),
                (Ordering::Less, Ordering::Less) => (4, px, py),
                _ => (0, px, py), // No quadrant
            },
        )
        .sorted()
        .chunk_by(|&(quadrant, _, _)| quadrant)
        .into_iter()
        .filter_map(|(quad, robots)| (quad != 0).then_some(robots.count()))
        .product();

    let solution2: Solution = {
        let mut robots = robots.collect_vec();

        let mut iteration = 0;
        for i in 0..1000000 {
            let mut map = [['.'; WIDTH as usize]; HEIGHT as usize];
            for &(px, py, _, _) in &robots {
                map[py as usize][px as usize] = '#';
            }

            // Print map if at least one line has "a lot of robots"
            // let has_tree = (0..WIDTH as usize).any(|j| {
            //     let max_length = map
            //         .iter()
            //         .filter_map(|line| line.get(j))
            //         .chunk_by(|&&ch| ch)
            //         .into_iter()
            //         .filter(|(ch, _)| *ch == '#')
            //         .map(|(_, group)| group.count())
            //         .max()
            //         .unwrap_or(0);
            //     // Frame of tree is 30 chars high
            //     max_length >= 30
            // });

            let has_tree = map.iter().any(|line| {
                line.iter()
                    .chunk_by(|&&ch| ch)
                    .into_iter()
                    .filter(|(ch, _)| *ch == '#')
                    .map(|(_, group)| group.count())
                    .max()
                    .unwrap_or(0)
                    // Frame of tree is 30 chars high
                    >= 30
            });

            if has_tree {
                // println!("Iteration {i}");
                // map.iter()
                //     .for_each(|line| println!("{}", line.iter().collect::<String>()));
                iteration = i;
                break;
            }

            // Simulate
            for robot in &mut robots {
                robot.0 = (robot.0 + robot.2).rem_euclid(WIDTH);
                robot.1 = (robot.1 + robot.3).rem_euclid(HEIGHT);
            }
        }

        iteration
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
