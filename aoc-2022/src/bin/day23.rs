use std::collections::{HashMap, HashSet, VecDeque};

use aoc_2022::*;
use itertools::Itertools;

const DAY: i32 = 23;
type Solution = usize;

type Elf = (isize, isize);
fn step(elf @ (x, y): Elf, map: &HashSet<Elf>, directions: &VecDeque<usize>) -> Elf {
    let n = !map.contains(&(x, y - 1));
    let s = !map.contains(&(x, y + 1));
    let w = !map.contains(&(x - 1, y));
    let e = !map.contains(&(x + 1, y));

    let nw = !map.contains(&(x - 1, y - 1));
    let ne = !map.contains(&(x + 1, y - 1));
    let sw = !map.contains(&(x - 1, y + 1));
    let se = !map.contains(&(x + 1, y + 1));

    if n && s && w && e && nw && ne && sw && se {
        return elf;
    }

    for direction in directions {
        match direction {
            0 if n && nw && ne => return (x, y - 1),
            1 if s && sw && se => return (x, y + 1),
            2 if w && nw && sw => return (x - 1, y),
            3 if e && ne && se => return (x + 1, y),
            _ => (),
        }
    }

    elf
}

fn main() {
    let input = get_input_text(DAY);

    let elves: HashSet<Elf> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(_, ch)| ch == b'#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect();

    let solution1: Solution = {
        let mut directions = VecDeque::from([0, 1, 2, 3]);
        let mut old;
        let mut new = elves.clone();

        let mut i = 0;
        while i != 10 {
            i += 1;
            old = new;
            new = HashSet::with_capacity(old.len());

            let mut to_move: HashMap<Elf, Vec<Elf>> = HashMap::with_capacity(old.len());
            for &elf in &old {
                let new_position = step(elf, &old, &directions);
                to_move.entry(new_position).or_default().push(elf);
            }

            for (pos, elves) in to_move {
                if elves.len() == 1 {
                    new.insert(pos);
                } else {
                    new.extend(elves);
                }
            }

            directions.rotate_left(1);
        }

        let (min_x, max_x) = new.iter().map(|&(x, _)| x).minmax().into_option().unwrap();
        let (min_y, max_y) = new.iter().map(|&(_, y)| y).minmax().into_option().unwrap();

        let area = (max_x - min_x + 1) * (max_y - min_y + 1);
        area as usize - elves.len()
    };

    let solution2: Solution = {
        let mut directions = VecDeque::from([0, 1, 2, 3]);
        let mut old = HashSet::new();
        let mut new = elves;

        let mut i = 0;
        while new != old {
            i += 1;
            old = new;
            new = HashSet::with_capacity(old.len());

            let mut to_move: HashMap<Elf, Vec<Elf>> = HashMap::with_capacity(old.len());
            for &elf in &old {
                let new_position = step(elf, &old, &directions);
                to_move.entry(new_position).or_default().push(elf);
            }

            for (pos, elves) in to_move {
                if elves.len() == 1 {
                    new.insert(pos);
                } else {
                    new.extend(elves);
                }
            }

            directions.rotate_left(1);
        }

        i
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
