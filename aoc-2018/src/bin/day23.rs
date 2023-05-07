use std::collections::BTreeMap;

use aoc_2018::*;

const DAY: i32 = 23;
type Solution = isize;

#[derive(Clone, Copy)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl From<&str> for Vector {
    fn from(value: &str) -> Self {
        let coords = value.split(['<', '>']).nth(1).unwrap();
        let mut coords = coords.split(',').map(|v| v.trim().parse().unwrap());

        Self {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
        }
    }
}

impl Vector {
    fn distance(&self, other: &Self) -> usize {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx.unsigned_abs() + dy.unsigned_abs() + dz.unsigned_abs()
    }
}

struct Nanobot {
    position: Vector,
    radius: isize,
}

impl From<&str> for Nanobot {
    fn from(value: &str) -> Self {
        let mut parts = value.split(", ");

        Self {
            position: parts.next().unwrap().into(),
            radius: parts.next().unwrap()[2..].parse().unwrap(),
        }
    }
}

impl Nanobot {
    fn is_in_radius(&self, other: &Self) -> bool {
        self.position.distance(&other.position) <= self.radius as usize
    }
}

fn main() {
    let input = get_input_text(DAY);
    let nanobots: Vec<_> = input.lines().map(Nanobot::from).collect();

    let solution1: Solution = {
        let strongest = nanobots.iter().max_by_key(|bot| bot.radius).unwrap();
        nanobots
            .iter()
            .filter(|bot| strongest.is_in_radius(bot))
            .count() as _
    };

    let solution2: Solution = {
        let mut distances = BTreeMap::new();
        for bot in &nanobots {
            let distance = bot.position.distance(&Vector { x: 0, y: 0, z: 0 }) as isize;
            *distances.entry(distance - bot.radius).or_insert(0) += 1;
            *distances.entry(distance + bot.radius + 1).or_insert(0) -= 1;
        }

        let running: Vec<_> = distances
            .iter()
            .scan(0, |state, (distance, count)| {
                *state += count;
                Some((distance, *state))
            })
            .collect();
        let max_count = running.iter().max_by_key(|&&(_, state)| state).unwrap().1;
        let ranges: Vec<_> = running
            .iter()
            .zip(running.iter().skip(1))
            .filter(|&(&(_, state), _)| state == max_count)
            .map(|(&(&start, _), &(&end, _))| (start, end - 1))
            .collect();

        ranges
            .iter()
            .map(|&(start, end)| if end < 0 { -end } else { start })
            .min()
            .unwrap()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
