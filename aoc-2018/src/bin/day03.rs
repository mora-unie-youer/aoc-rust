use std::collections::{HashMap, HashSet};

use aoc_2018::*;

const DAY: i32 = 3;
type Solution = usize;

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl From<&str> for Claim {
    fn from(value: &str) -> Self {
        let parts: Vec<_> = value
            .split(|c| c == ' ' || c == ',' || c == 'x' || c == ':')
            .collect();

        let id = parts[0][1..].parse().unwrap();
        let x = parts[2].parse().unwrap();
        let y = parts[3].parse().unwrap();
        let width = parts[5].parse().unwrap();
        let height = parts[6].parse().unwrap();

        Claim {
            id,
            x,
            y,
            width,
            height,
        }
    }
}

fn main() {
    let input = get_input_text(DAY);
    let claims: Vec<_> = input.lines().map(Claim::from).collect();

    let overlaps = {
        let mut map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
        for claim in &claims {
            for x in claim.x..claim.x + claim.width {
                for y in claim.y..claim.y + claim.height {
                    map.entry((x, y)).or_default().push(claim.id);
                }
            }
        }

        map
    };

    let solution1: Solution = overlaps.values().filter(|ids| ids.len() > 1).count();

    let solution2: Solution = {
        let mut ids: HashSet<usize> = (1..=claims.last().unwrap().id).collect();
        for claimed_ids in overlaps.values() {
            if claimed_ids.len() > 1 {
                for id in claimed_ids {
                    ids.remove(id);
                }
            }
        }

        *ids.iter().next().unwrap()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
