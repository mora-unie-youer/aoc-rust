use aoc_2019::*;
use pathfinding::{directed::bfs, prelude::Matrix};

const DAY: i32 = 20;
type Solution = usize;

#[derive(Clone, PartialEq, Eq)]
enum Tile {
    Space,
    Wall,
    Passage,
    // Portal can be linked and can be not
    //     name    connected         outer
    Portal(String, Option<Position>, bool),
}

type Position = (usize, usize);
fn get_map(input: &str) -> (Matrix<Tile>, Position, Position) {
    let char_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // let mut map = vec![];
    let mut map = Matrix::new(char_map.len(), char_map[0].len(), Tile::Space);
    let mut portals = vec![];
    for (y, line) in char_map.iter().enumerate() {
        // let mut row = vec![];
        for (x, tile) in line.iter().enumerate() {
            if tile.is_ascii_uppercase() {
                // If we have a letter -> place wall here
                map[(y, x)] = Tile::Wall;
                if x < line.len() - 1 && line[x + 1].is_ascii_uppercase() {
                    // Horizontal portal
                    let portal_name = format!("{}{}", tile, line[x + 1]);
                    if x < line.len() - 2 && line[x + 2] == '.' {
                        // Portal on the right
                        portals.push((x + 2, y, portal_name));
                    } else if x > 1 && line[x - 1] == '.' {
                        // Portal on the left
                        portals.push((x - 1, y, portal_name));
                    } else {
                        unreachable!();
                    }
                } else if y < char_map.len() - 1 && char_map[y + 1][x].is_ascii_uppercase() {
                    // Vertical portal
                    let portal_name = format!("{}{}", tile, char_map[y + 1][x]);
                    if y < char_map.len() - 2 && char_map[y + 2][x] == '.' {
                        // Portal on the bottom
                        portals.push((x, y + 2, portal_name));
                    } else if x > 1 && char_map[y - 1][x] == '.' {
                        // Portal on the top
                        portals.push((x, y - 1, portal_name));
                    } else {
                        unreachable!();
                    }
                }
            } else {
                // Any other tile
                map[(y, x)] = match tile {
                    ' ' => Tile::Space,
                    '#' => Tile::Wall,
                    '.' => Tile::Passage,
                    _ => unreachable!(),
                };
            }
        }

        // map.push(row);
    }

    // Create portal tiles on map
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    for (x, y, name) in &portals {
        // If they are not connected -> they are endpoints
        let connected = portals
            .iter()
            .find(|(px, py, pname)| pname == name && (px != x || py != y))
            .map(|&(x, y, _)| (y, x));

        // Check for outer bound
        let top_outer = *y == 2;
        let bottom_outer = *y == map.rows - 3;
        let left_outer = *x == 2;
        let right_outer = *x == map.columns - 3;
        let outer = top_outer || bottom_outer || left_outer || right_outer;
        map[(*y, *x)] = Tile::Portal(name.clone(), connected, outer);

        if name == "AA" {
            start_pos = (*y, *x);
        } else if name == "ZZ" {
            end_pos = (*y, *x);
        }
    }

    (map, start_pos, end_pos)
}

fn main() {
    let input = get_input_text(DAY);
    let (map, start_pos, end_pos) = get_map(&input);

    let solution1: Solution = {
        let path = bfs::bfs(
            &start_pos,
            |&pos| {
                let mut neighbors: Vec<_> = map.neighbours(pos, false).collect();
                if let Tile::Portal(_, Some(connected), _) = map[pos] {
                    neighbors.push(connected);
                }

                neighbors.retain(|&pos| map[pos] != Tile::Space && map[pos] != Tile::Wall);
                neighbors
            },
            |&pos| pos == end_pos,
        )
        .unwrap();

        path.len() - 1
    };

    let solution2: Solution = {
        let path = bfs::bfs(
            &(start_pos, 0),
            |&(pos, level)| {
                let mut neighbors: Vec<_> =
                    map.neighbours(pos, false).map(|pos| (pos, level)).collect();

                if let &Tile::Portal(_, Some(connected), outer) = &map[pos] {
                    if outer && level > 0 {
                        neighbors.push((connected, level - 1));
                    } else if !outer {
                        neighbors.push((connected, level + 1));
                    }
                }

                neighbors.retain(|&(pos, _)| map[pos] != Tile::Space && map[pos] != Tile::Wall);
                neighbors
            },
            |&(pos, level)| pos == end_pos && level == 0,
        )
        .unwrap();

        path.len() - 1
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
