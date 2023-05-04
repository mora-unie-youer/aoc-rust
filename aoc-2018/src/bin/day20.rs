use std::collections::HashMap;

use aoc_2018::*;

const DAY: i32 = 20;
type Solution = usize;

fn get_distances(input: &str) -> HashMap<(isize, isize), usize> {
    let mut distances: HashMap<(isize, isize), usize> = HashMap::new();
    let mut distance = 0;
    let mut stack = vec![];
    let mut position = (0, 0);
    for ch in input.chars() {
        match ch {
            '(' => stack.push((position, distance)),
            ')' => (position, distance) = stack.pop().unwrap(),
            '|' => (position, distance) = *stack.last().unwrap(),
            direction => {
                match direction {
                    'N' => position.1 -= 1,
                    'S' => position.1 += 1,
                    'W' => position.0 -= 1,
                    'E' => position.0 += 1,
                    _ => continue,
                };
                distance += 1;

                let v = distances.entry(position).or_insert(std::usize::MAX);
                *v = distance.min(*v);
            }
        }
    }

    distances
}

fn main() {
    let input = get_input_text(DAY);
    let distances = get_distances(input.trim());

    let solution1: Solution = *distances.values().max().unwrap();
    let solution2: Solution = distances
        .values()
        .filter(|&&distance| distance >= 1000)
        .count();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::get_distances;

    #[test]
    fn test_get_distances() {
        let input = "^WNE$";
        assert_eq!(get_distances(input).values().max(), Some(&3));
        let input = "^ENWWW(NEEE|SSE(EE|N))$";
        assert_eq!(get_distances(input).values().max(), Some(&10));
        let input = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        assert_eq!(get_distances(input).values().max(), Some(&18));
        let input = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        assert_eq!(get_distances(input).values().max(), Some(&23));
        let input = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        assert_eq!(get_distances(input).values().max(), Some(&31));
    }
}
