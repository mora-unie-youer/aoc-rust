use std::collections::{HashMap, HashSet};

use aoc_2024::*;
use itertools::Itertools;

const DAY: i32 = 8;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().len() as isize;

    let antennas = {
        let mut antennas: HashMap<char, Vec<_>> = HashMap::new();

        for (i, line) in input.lines().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                if ch != '.' {
                    let (i, j) = (i as isize, j as isize);
                    antennas.entry(ch).or_default().push((i, j));
                }
            }
        }

        antennas
    };

    let solution1: Solution = {
        let mut antinodes = HashSet::new();

        for positions in antennas.values() {
            for ab in positions.iter().combinations(2) {
                let (ay, ax) = *ab[0];
                let (by, bx) = *ab[1];
                let (dy, dx) = (ay - by, ax - bx);
                antinodes.insert((ay + dy, ax + dx));
                antinodes.insert((by - dy, bx - dx));
            }
        }

        antinodes
            .iter()
            .filter(|&(y, x)| (0..height).contains(y) && (0..width).contains(x))
            .count()
    };

    let solution2: Solution = {
        let mut antinodes = HashSet::new();

        for positions in antennas.values() {
            for ab in positions.iter().combinations(2) {
                let (ay, ax) = *ab[0];
                let (by, bx) = *ab[1];
                let (dy, dx) = (ay - by, ax - bx);

                let mut i = 0;
                loop {
                    let (nay, nax) = (ay + dy * i, ax + dx * i);
                    let (nby, nbx) = (by - dy * i, bx - dx * i);

                    let pos = [(nay, nax), (nby, nbx)];
                    if pos
                        .iter()
                        .all(|&(y, x)| y < 0 || y >= height || x < 0 || x >= width)
                    {
                        break;
                    }

                    antinodes.extend(pos);
                    i += 1;
                }
            }
        }

        antinodes
            .iter()
            .filter(|&&(y, x)| y >= 0 && y < height && x >= 0 && x < width)
            .count()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
