use std::cmp::Ordering;

use aoc_2021::*;
use pathfinding::directed::dijkstra;

const DAY: i32 = 23;
type Solution = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl TryFrom<usize> for Amphipod {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Amber),
            1 => Ok(Self::Bronze),
            2 => Ok(Self::Copper),
            3 => Ok(Self::Desert),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Amphipod {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Amber),
            'B' => Ok(Self::Bronze),
            'C' => Ok(Self::Copper),
            'D' => Ok(Self::Desert),
            _ => Err(()),
        }
    }
}

impl Amphipod {
    fn energy(&self) -> usize {
        10usize.pow(*self as u32)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State<const RS: usize> {
    hallway: [Option<Amphipod>; 11],
    rooms: [[Option<Amphipod>; RS]; 4],
}

impl<const RS: usize> State<RS> {
    fn new(rooms: [[Option<Amphipod>; RS]; 4]) -> Self {
        Self {
            hallway: Default::default(),
            rooms,
        }
    }

    fn is_goal(&self) -> bool {
        self.rooms.iter().enumerate().all(|(i, room)| {
            let room_amphipod = Amphipod::try_from(i).ok();
            room.iter().all(|amphipod| amphipod == &room_amphipod)
        })
    }

    fn is_room_enterable(&self, room_index: usize) -> bool {
        self.rooms[room_index].iter().all(|space| match space {
            &Some(amphipod) => amphipod as usize == room_index,
            _ => true,
        })
    }

    fn is_above_room(&self, x: usize) -> bool {
        (x - 2) % 2 == 0 && (x - 2) / 2 < self.rooms.len()
    }

    fn empty_spaces(&self, start_x: usize) -> impl Iterator<Item = usize> + '_ {
        let left_side = (0..start_x)
            .rev()
            .take_while(|&x| self.hallway[x].is_none());
        let right_side =
            ((start_x + 1)..self.hallway.len()).take_while(|&x| self.hallway[x].is_none());
        left_side.chain(right_side)
    }

    fn is_hallway_clear(&self, start_x: usize, target_x: usize) -> bool {
        let slice = match start_x.cmp(&target_x) {
            Ordering::Equal => {
                return true;
            }
            Ordering::Less => &self.hallway[start_x + 1..=target_x],
            Ordering::Greater => &self.hallway[target_x..start_x],
        };

        slice.iter().all(|space| space.is_none())
    }

    fn next_states(&self) -> Vec<(Self, usize)> {
        let mut next_states = self.room_to_hallway_states();
        next_states.append(&mut self.hallway_to_room_states());
        next_states
    }

    fn room_to_hallway_states(&self) -> Vec<(Self, usize)> {
        self.rooms
            .iter()
            .enumerate()
            .filter(|&(i, _)| !self.is_room_enterable(i))
            .flat_map(|(i, room)| {
                let (depth, amphipod) = room
                    .iter()
                    .enumerate()
                    .find_map(|(depth, space)| space.map(|amphipod| (depth, amphipod)))
                    .unwrap();

                let current_x = 2 + i * 2;

                self.empty_spaces(current_x)
                    .filter(|&target_x| !self.is_above_room(target_x))
                    .map(move |target_x| {
                        let diff = current_x as isize - target_x as isize;
                        let steps = depth + 1 + diff.unsigned_abs();
                        let energy = steps * amphipod.energy();

                        let mut state = *self;
                        std::mem::swap(&mut state.rooms[i][depth], &mut state.hallway[target_x]);

                        (state, energy)
                    })
            })
            .collect()
    }

    fn hallway_to_room_states(&self) -> Vec<(Self, usize)> {
        self.hallway
            .iter()
            .enumerate()
            .filter_map(|(current_x, space)| space.map(|amphipod| (current_x, amphipod)))
            .filter_map(|(current_x, amphipod)| {
                let target_room = amphipod as usize;
                if !self.is_room_enterable(target_room) {
                    return None;
                }

                let target_x = 2 + target_room * 2;
                if !self.is_hallway_clear(current_x, target_x) {
                    return None;
                }

                let depth = self.rooms[target_room]
                    .iter()
                    .rposition(|space| space.is_none())
                    .unwrap();

                let diff = current_x as isize - target_x as isize;
                let steps = depth + 1 + diff.unsigned_abs();
                let energy = steps * amphipod.energy();

                let mut state = *self;
                std::mem::swap(
                    &mut state.rooms[target_room][depth],
                    &mut state.hallway[current_x],
                );

                Some((state, energy))
            })
            .collect()
    }
}

fn solve<const RS: usize>(initial_state: State<RS>) -> Solution {
    let (_, cost) = dijkstra::dijkstra(
        &initial_state,
        |state| state.next_states(),
        |state| state.is_goal(),
    )
    .unwrap();

    cost
}

fn main() {
    let input = get_input_text(DAY);

    let amphipods: Vec<Amphipod> = input
        .chars()
        .filter_map(|ch| Amphipod::try_from(ch).ok())
        .collect();

    let solution1: Solution = {
        let rooms = (0..amphipods.len() / 2)
            .map(|i| [Some(amphipods[i]), Some(amphipods[i + amphipods.len() / 2])])
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap();

        solve(State::new(rooms))
    };

    let solution2: Solution = {
        let inserted = [
            [Amphipod::Desert, Amphipod::Desert],
            [Amphipod::Copper, Amphipod::Bronze],
            [Amphipod::Bronze, Amphipod::Amber],
            [Amphipod::Amber, Amphipod::Copper],
        ];

        let rooms = (0..amphipods.len() / 2)
            .map(|i| {
                [
                    Some(amphipods[i]),
                    Some(inserted[i][0]),
                    Some(inserted[i][1]),
                    Some(amphipods[i + amphipods.len() / 2]),
                ]
            })
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap();

        solve(State::new(rooms))
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
