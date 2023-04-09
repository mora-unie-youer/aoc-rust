use std::{
    collections::HashMap,
    ops::AddAssign,
};

use aoc_2017::*;

const DAY: i32 = 20;
type Solution = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl From<&str> for Vector {
    fn from(value: &str) -> Self {
        let coords = value.split(['<', '>']).nth(1).unwrap();
        let mut coords = coords.split(',').map(|v| v.parse().unwrap());

        Self {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Vector {
    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

#[derive(Clone)]
struct Particle {
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
}

impl From<&str> for Particle {
    fn from(value: &str) -> Self {
        let mut vectors = value.split_whitespace().map(Vector::from);

        Self {
            position: vectors.next().unwrap(),
            velocity: vectors.next().unwrap(),
            acceleration: vectors.next().unwrap(),
        }
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Particle {
    fn tick(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }

    fn distance(&self) -> usize {
        self.position.manhattan_distance()
    }
}

const SIMULATION_TICKS: usize = 10_000;
fn main() {
    let input = get_input_text(DAY);
    let particles: Vec<_> = input.lines().map(Particle::from).collect();

    let solution1: Solution = {
        let mut particles = particles.clone();

        for _ in 0..SIMULATION_TICKS {
            particles.iter_mut().for_each(|particle| particle.tick());
        }

        particles
            .iter()
            .enumerate()
            .min_by_key(|(_, particle)| particle.distance())
            .unwrap()
            .0
    };

    let solution2: Solution = {
        let mut particles = particles;

        for _ in 0..SIMULATION_TICKS {
            let mut collisions = HashMap::new();

            particles.iter_mut().for_each(|particle| {
                particle.tick();
                let collision = collisions.entry(particle.position).or_insert(0);
                *collision += 1;
            });

            collisions
                .iter()
                .filter(|(_, &n)| n > 1)
                .for_each(|(&position, _)| {
                    particles.retain(|particle| particle.position != position)
                })
        }

        particles.len()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
