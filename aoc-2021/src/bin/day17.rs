use std::ops::RangeInclusive;

use aoc_2021::*;

const DAY: i32 = 17;
type Solution = isize;

fn simulate(
    xrange: &RangeInclusive<isize>,
    yrange: &RangeInclusive<isize>,
    mut vx: isize,
    mut vy: isize,
) -> bool {
    let (mut x, mut y) = (0, 0);

    loop {
        // Changing position
        x += vx;
        y += vy;
        // Changing velocity
        vx -= vx.signum();
        vy -= 1;

        // Checking if we finished
        match (xrange.contains(&x), yrange.contains(&y)) {
            (true, true) => return true,
            (false, _) if vx == 0 => return false,
            (_, false) if y < *yrange.start() => return false,
            _ => {}
        }
    }
}

fn main() {
    let input = get_input_text(DAY);

    let (_, ranges) = input.trim().split_once(": ").unwrap();
    let (xrange, yrange) = ranges.split_once(", ").unwrap();

    let xrange = xrange[2..]
        .split_once("..")
        .map(|(a, b)| (a.parse::<isize>().unwrap()..=b.parse().unwrap()))
        .unwrap();
    let yrange = yrange[2..]
        .split_once("..")
        .map(|(a, b)| (a.parse::<isize>().unwrap()..=b.parse().unwrap()))
        .unwrap();

    let solution1: Solution = {
        let min_y: isize = yrange.start().abs();
        min_y * (min_y - 1) / 2
    };

    let solution2: Solution = {
        let max_x: isize = xrange.end().abs();
        let min_y: isize = yrange.start().abs();

        let mut count = 0;
        for vy in -min_y..min_y {
            for vx in 0..=max_x {
                count += simulate(&xrange, &yrange, vx, vy) as isize;
            }
        }

        count
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
