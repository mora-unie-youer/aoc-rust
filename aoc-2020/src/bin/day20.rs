use std::collections::{HashMap, HashSet, VecDeque};

use aoc_2020::*;

const DAY: i32 = 20;
type Solution = usize;

type Edges = HashMap<Vec<bool>, Vec<usize>>;
const TILE_SIZE: usize = 10;

#[derive(Default, Clone)]
struct Tile {
    id: usize,
    grid: Vec<Vec<bool>>,
}

impl From<&str> for Tile {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let title = lines.next().unwrap();

        let id = title[5..title.len() - 1].parse().unwrap();
        let grid = lines
            .map(|line| line.chars().map(|ch| ch == '#').collect())
            .collect();

        Self { id, grid }
    }
}

impl Tile {
    /// Returns [up, down, left, right] edges.
    fn get_edges(&self) -> [Vec<bool>; 4] {
        let up = self.grid[0].clone();
        let down = self.grid[TILE_SIZE - 1].clone();

        let (mut left, mut right) = (vec![], vec![]);
        for i in 0..10 {
            left.push(self.grid[i][0]);
            right.push(self.grid[i][TILE_SIZE - 1]);
        }

        [up, down, left, right]
    }

    /// Returns neighbor's id on direction
    fn get_neighbor(&self, edges: &Edges, dir: usize) -> Option<usize> {
        let ids = &edges[&self.get_edges()[dir]];
        ids.iter().find(|&&id| id != self.id).copied()
    }

    /// Rotates tile image
    fn rotate(&mut self) {
        self.grid = rotate(&self.grid);
    }

    /// Flip tile image horizontally/vertically
    fn flip(&mut self, vert: bool) {
        if vert {
            for row in &mut self.grid {
                row.reverse();
            }
        } else {
            for y in 0..5 {
                self.grid.swap(y, TILE_SIZE - y - 1);
            }
        }
    }
}

fn rotate(data: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; data.len()]; data[0].len()];

    for (y, row) in data.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            result[x][data[0].len() - y - 1] = cell;
        }
    }

    result
}

fn place_tiles(
    tiles: &HashMap<usize, Tile>,
    edges: &Edges,
    starting: usize,
    grid_size: usize,
) -> HashMap<(usize, usize), Tile> {
    let mut starting_corner = tiles[&starting].clone();
    while [0, 2]
        .into_iter()
        .any(|dir| starting_corner.get_neighbor(edges, dir).is_some())
    {
        starting_corner.rotate();
    }

    let mut placed_tiles = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    placed_tiles.insert((0, 0), starting_corner.clone());
    visited.insert((0, 0));
    queue.push_back(((0, 0), starting_corner));

    while let Some((pos, tile)) = queue.pop_front() {
        let positions = [(1, (pos.0 + 1, pos.1)), (3, (pos.0, pos.1 + 1))];
        for (dir, new_pos) in positions {
            if new_pos.0 >= grid_size || new_pos.1 >= grid_size {
                continue;
            }

            if !visited.contains(&new_pos) {
                let neighbor_id = tile.get_neighbor(edges, dir).unwrap();
                let mut neighbor = tiles[&neighbor_id].clone();

                // Rotating while it doesn't match on opposite side
                while neighbor.get_neighbor(edges, dir - 1) != Some(tile.id) {
                    neighbor.rotate();
                }

                // If sides are not equal -> flip
                if dir == 1 && tile.grid[9] != neighbor.grid[0] {
                    neighbor.flip(true);
                } else if dir == 3
                    && (0..TILE_SIZE).any(|y| tile.grid[y][TILE_SIZE - 1] != neighbor.grid[y][0])
                {
                    neighbor.flip(false);
                }

                placed_tiles.insert(new_pos, neighbor.clone());
                queue.push_back((new_pos, neighbor));
                visited.insert(new_pos);
            }
        }
    }

    placed_tiles
}

fn main() {
    let input = get_input_text(DAY);
    let tiles: Vec<_> = input.trim().split("\n\n").map(Tile::from).collect();

    let mut edges: Edges = HashMap::new();
    for tile in &tiles {
        let tile_edges = tile.get_edges();
        for mut edge in tile_edges {
            edges.entry(edge.clone()).or_default().push(tile.id);
            edge.reverse();
            edges.entry(edge).or_default().push(tile.id);
        }
    }

    let mut counts: HashMap<usize, usize> = HashMap::new();
    for ids in edges.values().filter(|ids| ids.len() == 1) {
        *counts.entry(ids[0]).or_default() += 1;
    }

    let corners: Vec<_> = counts
        .iter()
        .filter(|&(_, &count)| count == 4)
        .map(|(&id, _)| id)
        .collect();

    let solution1: Solution = corners.iter().product();

    const MONSTER: &str = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ";
    let solution2: Solution = {
        let tiles: HashMap<usize, Tile> = tiles.into_iter().map(|tile| (tile.id, tile)).collect();
        let len = tiles.values().count();
        let size = (1..).find(|v| v * v == len).unwrap();
        let placed_tiles = place_tiles(&tiles, &edges, corners[0], size);

        let tile_size = TILE_SIZE - 2;
        let image_size = size * tile_size;
        let mut image = vec![vec![false; image_size + 1]; image_size + 1];
        for ty in 0..size {
            for tx in 0..size {
                let tile = &placed_tiles[&(ty, tx)];
                for y in 1..TILE_SIZE - 1 {
                    for x in 1..TILE_SIZE - 1 {
                        image[ty * tile_size + y][tx * tile_size + x] = tile.grid[y][x];
                    }
                }
            }
        }

        let monster_coords: Vec<_> = MONSTER
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, ch)| (y, x, ch)))
            .filter(|&(_, _, ch)| ch == '#')
            .map(|(y, x, _)| (y, x))
            .collect();
        let monster_max_y = monster_coords.iter().map(|coord| coord.0).max().unwrap();
        let monster_max_x = monster_coords.iter().map(|coord| coord.1).max().unwrap();

        let mut monster_count = 0;
        for attempt in 0..8 {
            for y in 0..image.len() - monster_max_y {
                for x in 0..image[0].len() - monster_max_x {
                    let found = monster_coords
                        .iter()
                        .map(|(my, mx)| (y + my, x + mx))
                        .all(|(y, x)| image[y][x]);
                    monster_count += found as usize;
                }
            }

            if monster_count != 0 {
                break;
            } else if attempt == 3 {
                for row in &mut image {
                    row.reverse();
                }
            } else {
                image = rotate(&image);
            }
        }

        image.iter().flatten().filter(|&&cell| cell).count() - monster_count * monster_coords.len()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
