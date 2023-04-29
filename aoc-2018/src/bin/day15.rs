use std::{
    borrow::Borrow,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc_2018::*;

const DAY: i32 = 15;
type Solution = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl Position {
    fn distance(&self, other: &Self) -> usize {
        let dx = self.x as isize - other.x as isize;
        let dy = self.y as isize - other.y as isize;
        dx.unsigned_abs() + dy.unsigned_abs()
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Unit {
    position: Position,
    health: usize,
    damage: usize,
    kind: char,
}

impl Unit {
    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<bool>>,
    units: Vec<Unit>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut grid = vec![];
        let mut units = vec![];

        for (y, line) in value.lines().enumerate() {
            let mut row = vec![];
            for (x, ch) in line.char_indices() {
                if ch == 'E' || ch == 'G' {
                    let position = Position { x, y };
                    units.push(Unit {
                        position,
                        health: 200,
                        damage: 3,
                        kind: ch,
                    });
                }

                row.push(ch == '#');
            }

            grid.push(row);
        }

        Self { grid, units }
    }
}

impl Map {
    fn run(&mut self) -> Solution {
        let mut iteration = 0;
        while self.tick() {
            iteration += 1;
        }

        let units_hp: usize = self.units.iter().map(|unit| unit.health).sum();
        iteration * units_hp
    }

    fn tick(&mut self) -> bool {
        self.units.sort();
        for i in 0..self.units.len() {
            if !self.units[i].is_alive() {
                continue;
            }

            if !self.contains_enemies() {
                return false;
            }

            if let Some(new_position) = self.next_step(i) {
                self.units[i].position = new_position;
            }

            if let Some(target) = self.next_target(i) {
                let damage = self.units[i].damage;
                let target = &mut self.units[target];
                target.health = target.health.saturating_sub(damage);
            }
        }

        self.units.retain(|unit| unit.is_alive());
        true
    }

    fn contains_enemies(&self) -> bool {
        let unit = &self.units.iter().find(|u| u.is_alive()).unwrap();
        let another = self
            .units
            .iter()
            .find(|u| u.kind != unit.kind && u.is_alive());
        another.is_some()
    }

    fn elves_count(&self) -> usize {
        self.units
            .iter()
            .filter(|unit| unit.kind == 'E' && unit.is_alive())
            .count()
    }

    fn next_step(&self, id: usize) -> Option<Position> {
        let unit = &self.units[id];

        let enemy_positions: HashSet<_> = self
            .units
            .iter()
            .filter(|u| u.kind != unit.kind && u.is_alive())
            .map(|u| u.position)
            .collect();
        let units_positions: HashSet<_> = self
            .units
            .iter()
            .filter(|u| u.is_alive())
            .map(|u| u.position)
            .collect();

        let mut queue = BinaryHeap::new();
        let mut prev = HashMap::new();
        queue.push(Reverse((0, unit.position)));
        while let Some(Reverse((distance, Position { x, y }))) = queue.pop() {
            let neighbors = [
                Position { x, y: y - 1 },
                Position { x: x - 1, y },
                Position { x: x + 1, y },
                Position { x, y: y + 1 },
            ];

            for pos in neighbors {
                if !prev.contains_key(&pos) && !self.grid[pos.y][pos.x] {
                    if enemy_positions.contains(&pos) {
                        return prev.remove(&Position { x, y });
                    } else if !units_positions.contains(&pos) {
                        let prev_position = *prev.get(&Position { x, y }).unwrap_or(&pos);
                        prev.insert(pos, prev_position);
                        queue.push(Reverse((distance + 1, pos)));
                    }
                }
            }
        }

        None
    }

    fn next_target(&self, id: usize) -> Option<usize> {
        let unit = &self.units[id];
        self.units
            .iter()
            .enumerate()
            .filter(|(_, u)| u.kind != unit.kind && u.is_alive())
            .filter(|(_, u)| u.position.distance(&unit.position) == 1)
            .min_by_key(|(_, u)| (u.health, u.position))
            .map(|(i, _)| i)
    }
}

fn main() {
    let input = get_input_text(DAY);
    let map = Map::from(input.borrow());

    let solution1: Solution = {
        let mut map = map.clone();
        map.run()
    };

    let solution2: Solution = {
        let starting_elves = map.elves_count();
        let mut damage = 4;
        loop {
            let mut map = map.clone();
            map.units
                .iter_mut()
                .filter(|unit| unit.kind == 'E')
                .for_each(|unit| unit.damage = damage);

            let outcome = map.run();
            if starting_elves == map.elves_count() {
                break outcome;
            }

            damage += 1;
        }
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::Map;

    #[test]
    fn test_map() {
        let input = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        let mut map = Map::from(input);
        assert_eq!(map.run(), 27730);

        let input = "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";
        let mut map = Map::from(input);
        assert_eq!(map.run(), 36334);

        let input = "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
        let mut map = Map::from(input);
        assert_eq!(map.run(), 39514);

        let input = "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";
        let mut map = Map::from(input);
        assert_eq!(map.run(), 27755);

        let input = "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";
        let mut map = Map::from(input);
        assert_eq!(map.run(), 28944);

        let input = "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";
        let mut map = Map::from(input);
        assert_eq!(map.run(), 18740);
    }
}
