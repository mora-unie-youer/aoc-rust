use std::collections::HashSet;

use aoc_2019::*;

const DAY: i32 = 24;
type Solution = u32;

fn get_tile(map: u32, i: usize) -> bool {
    (map & (1 << i)) != 0
}

fn neighbors(map: u32, i: usize) -> u32 {
    let mut neighbors = 0;
    let (r, c) = (i / 5, i % 5);

    // Left
    neighbors += if c > 0 { get_tile(map, i - 1) as _ } else { 0 };
    // Right
    neighbors += if c < 4 { get_tile(map, i + 1) as _ } else { 0 };
    // Top
    neighbors += if r > 0 { get_tile(map, i - 5) as _ } else { 0 };
    // Bottom
    neighbors += if r < 4 { get_tile(map, i + 5) as _ } else { 0 };

    neighbors
}

fn neighbors_recursive(maps: [u32; 3], i: usize) -> u32 {
    let [outer, map, inner] = maps;
    let mut neighbors = neighbors(map, i);
    let (r, c) = (i / 5, i % 5);

    // Outer
    neighbors += if r == 0 { get_tile(outer, 7) as _ } else { 0 };
    neighbors += if r == 4 { get_tile(outer, 17) as _ } else { 0 };
    neighbors += if c == 0 { get_tile(outer, 11) as _ } else { 0 };
    neighbors += if c == 4 { get_tile(outer, 13) as _ } else { 0 };

    // Inner
    neighbors += match (r, c) {
        (1, 2) => (inner & 0b00000_00000_00000_00000_11111).count_ones(),
        (2, 1) => (inner & 0b00001_00001_00001_00001_00001).count_ones(),
        (2, 3) => (inner & 0b10000_10000_10000_10000_10000).count_ones(),
        (3, 2) => (inner & 0b11111_00000_00000_00000_00000).count_ones(),
        _ => 0,
    };

    neighbors
}

fn tick<F>(map: u32, get_neighbors: F) -> u32
where
    F: Fn(usize) -> u32,
{
    let mut new_map = 0;
    for i in 0..25 {
        let tile = get_tile(map, i);
        let neighbors = get_neighbors(i);

        new_map |= match tile {
            false if neighbors == 1 || neighbors == 2 => 1 << i,
            true if neighbors == 1 => 1 << i,
            _ => 0,
        };
    }

    new_map
}

fn grow(map: u32) -> u32 {
    tick(map, |i| neighbors(map, i))
}

fn grow_recursive(maps: [u32; 3]) -> u32 {
    tick(maps[1], |i| neighbors_recursive(maps, i))
}

fn main() {
    let input = get_input_text(DAY);

    let map = input
        .chars()
        .filter(|&ch| ch != '\n')
        .rev()
        .fold(0, |map, ch| {
            let shifted_map = map << 1;
            let tile = (ch == '#') as u32;
            shifted_map | tile
        });

    let solution1: Solution = {
        let mut layouts = HashSet::new();
        let mut map = map;
        layouts.insert(map);

        loop {
            map = grow(map);
            if !layouts.insert(map) {
                break map;
            }
        }
    };

    let solution2: Solution = {
        const MASK: u32 = 0b11111_11111_11011_11111_11111;
        let mut levels = vec![map & MASK];

        for _ in 0..200 {
            let mut buffer = vec![0];
            buffer.extend(levels);
            buffer.push(0);

            let mut new_levels = vec![];
            new_levels.push(grow_recursive([0, buffer[0], buffer[1]]) & MASK);

            for i in 1..buffer.len() - 1 {
                new_levels.push(grow_recursive([buffer[i - 1], buffer[i], buffer[i + 1]]) & MASK);
            }

            let i = buffer.len() - 1;
            new_levels.push(grow_recursive([buffer[i - 1], buffer[i], 0]) & MASK);
            levels = new_levels;
        }

        levels.into_iter().map(|level| level.count_ones()).sum()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::{get_tile, neighbors};

    #[test]
    fn test_get_tile() {
        // #####
        // .#.#.
        // #.#.#
        // .#.#.
        // #.#.#
        let map = 0b10101_10101_10101_01010_11111;
        assert!(get_tile(map, 0));
        assert!(get_tile(map, 1));
        assert!(get_tile(map, 2));
        assert!(get_tile(map, 3));
        assert!(get_tile(map, 4));
        assert!(!get_tile(map, 5));
    }

    #[test]
    fn test_neighbors() {
        // #####
        // .#.#.
        // #.#.#
        // .#.#.
        // #.#.#
        let map = 0b10101_01010_10101_01010_11111;
        assert_eq!(neighbors(map, 0), 1);
        assert_eq!(neighbors(map, 1), 3);
        assert_eq!(neighbors(map, 2), 2);
        assert_eq!(neighbors(map, 10), 0);
        assert_eq!(neighbors(map, 24), 0);
    }
}
