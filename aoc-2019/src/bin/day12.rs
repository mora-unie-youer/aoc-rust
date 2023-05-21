use std::{cmp::Ordering, ops::AddAssign};

use aoc_2019::*;

const DAY: i32 = 12;
type Solution = usize;

#[derive(Default, Clone, Copy, Debug)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl From<&str> for Vector {
    fn from(value: &str) -> Self {
        let coords = value.split(['<', '>']).nth(1).unwrap();
        let mut coords = coords.split(", ").map(|v| v[2..].parse().unwrap());

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

#[derive(Clone, Debug)]
struct Moon {
    position: Vector,
    velocity: Vector,
}

impl From<&str> for Moon {
    fn from(value: &str) -> Self {
        Self {
            position: value.into(),
            velocity: Vector::default(),
        }
    }
}

impl Moon {
    fn potential_energy(&self) -> usize {
        let Vector { x, y, z } = self.position;
        x.unsigned_abs() + y.unsigned_abs() + z.unsigned_abs()
    }

    fn kinetic_energy(&self) -> usize {
        let Vector { x, y, z } = self.velocity;
        x.unsigned_abs() + y.unsigned_abs() + z.unsigned_abs()
    }

    fn energy(&self) -> usize {
        self.potential_energy() * self.kinetic_energy()
    }

    fn apply_gravity(&mut self, neighbor: Moon) {
        self.velocity.x += compare_axis(self.position.x, neighbor.position.x);
        self.velocity.y += compare_axis(self.position.y, neighbor.position.y);
        self.velocity.z += compare_axis(self.position.z, neighbor.position.z);
    }
}

fn compare_axis(a: isize, b: isize) -> isize {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

fn find_steps_on_axis(mut positions: Vec<isize>) -> usize {
    let mut velocities = vec![0; positions.len()];

    let mut steps = 0;
    loop {
        // Applying gravity
        for (i, &a) in positions.iter().enumerate() {
            for (j, &b) in positions.iter().enumerate().skip(i + 1) {
                velocities[i] += compare_axis(a, b);
                velocities[j] += compare_axis(b, a);
            }
        }

        // Applying velocities
        positions.iter_mut().zip(&velocities).for_each(|(p, v)| *p += v);

        steps += 1;
        if velocities.iter().all(|&v| v == 0) {
            // We are only on a half of a cycle
            break steps * 2;
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    let input = get_input_text(DAY);
    let moons: Vec<_> = input.lines().map(Moon::from).collect();

    let solution1: Solution = {
        let mut moons = moons.clone();

        for _ in 0..1000 {
            // Applying gravity
            for i in 0..moons.len() {
                let mut neighbors = moons.clone();
                neighbors.remove(i);
                neighbors
                    .into_iter()
                    .for_each(|neighbor| moons[i].apply_gravity(neighbor));
            }

            // Applying velocity
            moons
                .iter_mut()
                .for_each(|moon| moon.position += moon.velocity);
        }

        moons.into_iter().map(|moon| moon.energy()).sum()
    };

    let solution2: Solution = {
        let xs = moons.iter().map(|moon| moon.position.x).collect();
        let ys = moons.iter().map(|moon| moon.position.y).collect();
        let zs = moons.iter().map(|moon| moon.position.z).collect();

        let x_steps = find_steps_on_axis(xs);
        let y_steps = find_steps_on_axis(ys);
        let z_steps = find_steps_on_axis(zs);
        lcm(x_steps, lcm(y_steps, z_steps))
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
