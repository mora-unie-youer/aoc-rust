use std::collections::{HashMap, HashSet};

use aoc_2019::*;
use pathfinding::{
    directed::{bfs, dijkstra},
    prelude::Matrix,
};

const DAY: i32 = 18;
type Solution = usize;
type Position = (usize, usize);

fn get_map(input: &str, edit_map: bool) -> (Matrix<&u8>, Position) {
    let mut map = Matrix::from_rows(input.lines().map(|line| line.as_bytes())).unwrap();

    let (x, y, _) = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, tile)| (x, y, tile)))
        .find(|&(_, _, &&tile)| tile == b'@')
        .unwrap();

    if edit_map {
        // Edit map to 4 robots
        map[(y - 1, x - 1)] = &b'@';
        map[(y - 1, x)] = &b'#';
        map[(y - 1, x + 1)] = &b'@';
        map[(y, x - 1)] = &b'#';
        map[(y, x)] = &b'#';
        map[(y, x + 1)] = &b'#';
        map[(y + 1, x - 1)] = &b'@';
        map[(y + 1, x)] = &b'#';
        map[(y + 1, x + 1)] = &b'@';
        (map, (y, x))
    } else {
        (map, (y, x))
    }
}

fn reachable_keys(map: &Matrix<&u8>, robot: Position) -> HashSet<Position> {
    let mut keys = HashSet::new();

    bfs::bfs(
        &robot,
        |&pos| {
            map.neighbours(pos, false)
                .filter(|&n| match map[n] {
                    b'#' | b'@' => false,
                    tile if tile.is_ascii_lowercase() => {
                        keys.insert(n);
                        true
                    }
                    _ => true,
                })
                .collect::<Vec<_>>()
        },
        |_| false,
    );

    keys
}

fn get_path_to(map: &Matrix<&u8>, start: Position, target: Position) -> Vec<Position> {
    bfs::bfs(
        &start,
        |&pos| map.neighbours(pos, false).filter(|&n| map[n] != &b'#'),
        |&pos| pos == target,
    )
    .unwrap()
}

type Key<'input> = (Position, usize, Vec<&'input u8>);
fn get_key_distances<'input>(
    map: &Matrix<&'input u8>,
    robot: Position,
    keys: &HashSet<Position>,
) -> HashMap<Position, Vec<Key<'input>>> {
    let mut points = vec![robot];
    points.extend(keys);

    points
        .iter()
        .map(|&point| {
            let distances = points
                .iter()
                .skip(1)
                .filter(|&&p| p != point)
                .map(|&p| {
                    let path = get_path_to(map, point, p);
                    let required = path
                        .iter()
                        .map(|&pos| map[pos])
                        .filter(|tile| tile.is_ascii_uppercase())
                        .collect::<Vec<_>>();
                    (p, path.len() - 1, required)
                })
                .collect();
            (point, distances)
        })
        .collect()
}

fn solve(map: Matrix<&u8>, robots: &[Position]) -> Solution {
    let robot_keys: Vec<_> = robots
        .iter()
        .map(|&robot| reachable_keys(&map, robot))
        .collect();
    let key_count: usize = robot_keys.iter().map(|keys| keys.len()).sum();
    let all_keys = (1 << key_count) - 1;

    let distances: Vec<_> = robots
        .iter()
        .enumerate()
        .map(|(i, &robot)| get_key_distances(&map, robot, &robot_keys[i]))
        .collect();

    dijkstra::dijkstra(
        &(robots.to_vec(), 0),
        |(robots, collected_keys)| {
            robots
                .iter()
                .enumerate()
                .flat_map(|(i, robot)| {
                    let current_distances = &distances[i][robot];
                    current_distances
                        .iter()
                        .filter_map(|&(target, cost, ref required)| {
                            let key = 1 << (map[target] - b'a');
                            let already_collected = collected_keys & key != 0;
                            let no_key = required
                                .iter()
                                .any(|&&door| collected_keys & (1 << (door - b'A')) == 0);

                            if already_collected || no_key {
                                None
                            } else {
                                Some((target, cost, collected_keys | key))
                            }
                        })
                        .map(move |(target, cost, keys)| {
                            let mut new_state = robots.clone();
                            new_state[i] = target;
                            ((new_state, keys), cost)
                        })
                })
                .collect::<Vec<_>>()
        },
        |&(_, collected_keys)| collected_keys == all_keys,
    )
    .unwrap()
    .1
}

fn solve_part1(input: &str) -> Solution {
    let (map, robot) = get_map(input, false);
    let robots = &[robot];
    solve(map, robots)
}

fn solve_part2(input: &str, edit_map: bool) -> usize {
    let (map, _) = get_map(input, edit_map);
    let robots: Vec<_> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, tile)| (x, y, tile)))
        .filter(|&(_, _, &&tile)| tile == b'@')
        .map(|(x, y, _)| (y, x))
        .collect();
    solve(map, &robots)
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = solve_part1(&input);
    let solution2: Solution = solve_part2(&input, true);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::{solve_part1, solve_part2};

    #[test]
    fn test_solve_part1() {
        let input = "#########
#b.A.@.a#
#########";
        assert_eq!(solve_part1(input), 8);

        let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        assert_eq!(solve_part1(input), 86);

        let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        assert_eq!(solve_part1(input), 132);

        let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        assert_eq!(solve_part1(input), 136);

        let input = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";
        assert_eq!(solve_part1(input), 81);
    }

    #[test]
    fn test_solve_part2() {
        let input = "###############
#d.ABC.#.....a#
######@#@######
###############
######@#@######
#b.....#.....c#
###############";
        assert_eq!(solve_part2(input, false), 24);

        let input = "#############
#DcBa.#.GhKl#
#.###@#@#I###
#e#d#####j#k#
###C#@#@###J#
#fEbA.#.FgHi#
#############";
        assert_eq!(solve_part2(input, false), 32);

        let input = "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";
        assert_eq!(solve_part2(input, false), 72);
    }
}
