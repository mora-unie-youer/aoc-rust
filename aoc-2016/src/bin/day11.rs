#![feature(once_cell)]

use aoc_2016::*;
use itertools::Itertools;
use pathfinding::directed::astar;

const DAY: i32 = 11;
type Solution = usize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Device {
    Generator(String),
    Microchip(String),
}

trait IsFloorSafe {
    fn is_floor_safe(&self) -> bool;
}

impl IsFloorSafe for Vec<Device> {
    fn is_floor_safe(&self) -> bool {
        let mut generators = vec![];
        let mut microchips = vec![];

        self.iter().for_each(|device| {
            match device {
                Device::Generator(material) => generators.push(material),
                Device::Microchip(material) => microchips.push(material),
            };
        });

        generators.is_empty() || microchips.iter().all(|m| generators.iter().any(|g| m == g))
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct State {
    current_floor: usize,
    floors: Vec<Vec<Device>>,
}

impl From<&str> for State {
    fn from(input: &str) -> Self {
        let device_regex = regex!(r"a ([a-z]+)(-[a-z]+)? ([a-z]+)");
        let mut floors = vec![];
        for line in input.lines() {
            let mut floor = vec![];
            for device in device_regex.captures_iter(line) {
                let material = device.get(1).unwrap().as_str();
                let class = device.get(3).unwrap().as_str();

                let device = match class {
                    "generator" => Device::Generator(material.into()),
                    "microchip" => Device::Microchip(material.into()),
                    _ => unreachable!(),
                };

                floor.push(device);
            }

            floor.sort();
            floors.push(floor);
        }

        Self {
            current_floor: 0,
            floors,
        }
    }
}

impl State {
    fn try_grab(&self, dest_floor: usize, mut grab: Vec<Device>) -> Option<Self> {
        let new_src: Vec<Device> = self.floors[self.current_floor]
            .iter()
            .cloned()
            .filter(|device| !grab.contains(device))
            .collect();
        if !new_src.is_floor_safe() {
            return None;
        }

        let mut new_dest: Vec<Device> = self.floors[dest_floor].clone();
        new_dest.append(&mut grab);
        if !new_dest.is_floor_safe() {
            return None;
        }

        let mut floors = self.floors.clone();
        new_dest.sort();
        floors[self.current_floor] = new_src;
        floors[dest_floor] = new_dest;

        Some(Self {
            current_floor: dest_floor,
            floors,
        })
    }

    fn neighbors(&self) -> Vec<(State, usize)> {
        let mut neighbors = vec![];

        for direction in &[-1, 1] {
            let dest_floor = self.current_floor as isize + direction;
            if dest_floor < 0 || dest_floor >= self.floors.len() as isize {
                continue;
            }
            let dest_floor = dest_floor as usize;

            let floor = &self.floors[self.current_floor];
            // Cases when we take one device
            for device in floor.iter().cloned() {
                if let Some(state) = self.try_grab(dest_floor, vec![device]) {
                    neighbors.push((state, 1));
                }
            }

            // Cases when we take two devices
            for devices in floor.iter().cloned().combinations(2) {
                if let Some(state) = self.try_grab(dest_floor, devices) {
                    neighbors.push((state, 1));
                }
            }
        }

        neighbors
    }

    fn is_goal(&self) -> bool {
        // We must be on the last floor
        if self.current_floor != self.floors.len() - 1 {
            return false;
        }

        // All items must be on the last floor
        for floor in self.floors.iter().rev().skip(1) {
            if !floor.is_empty() {
                return false;
            }
        }

        true
    }

    // The bigger count of items on lower floors, the worse it becomes
    fn cost(&self) -> usize {
        self.floors.iter().enumerate().fold(0, |cost, (i, floor)| {
            cost + (self.floors.len() - i - 1) * floor.len()
        })
    }
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = {
        let state = State::from(input.trim());
        let result = astar::astar(
            &state,
            |state| state.neighbors(),
            |state| state.cost(),
            |state| state.is_goal(),
        )
        .expect("No solution was found");

        result.1
    };

    //// TAKES TOO LONG, but can (or not :P) solve your task
    // let solution2: Solution = {
    //     let mut state = State::from(input.trim());
    //     state.floors[0].push(Device::Generator("elerium".into()));
    //     state.floors[0].push(Device::Microchip("elerium".into()));
    //     state.floors[0].push(Device::Generator("dilithium".into()));
    //     state.floors[0].push(Device::Microchip("dilithium".into()));
    //     state.floors[0].sort();
    //
    //     let result = astar::astar(
    //         &state,
    //         |state| state.neighbors(),
    //         |state| state.cost(),
    //         |state| state.is_goal(),
    //     )
    //     .expect("No solution was found");
    //
    //     result.1
    // };

    show_solution(DAY, solution1);
    // show_solution(DAY, solution2);
}
