#![feature(array_chunks)]
#![feature(slice_flatten)]

use aoc_2017::*;

const DAY: i32 = 14;
type Solution = usize;

// Took code from day 10
fn reverse_sublist(list: &mut [usize], start: usize, length: usize) {
    let len = list.len();
    let end = (start + length - 1) % len;
    for i in 0..length / 2 {
        let a = (start + i) % len;
        let b = (end + len - i) % len;
        list.swap(a, b);
    }
}

fn hard_hash(input: &str) -> Vec<usize> {
    let mut lengths = input.trim().as_bytes().to_vec();
    lengths.extend([17, 31, 73, 47, 23]);

    let mut current_pos = 0;
    let mut list: Vec<_> = (0..=255).collect();
    let mut skip_size = 0;

    for _ in 0..64 {
        for &length in lengths.iter() {
            reverse_sublist(&mut list, current_pos, length as usize);
            current_pos = (current_pos + length as usize + skip_size) % list.len();
            skip_size += 1;
        }
    }

    let sparse_hash = list;
    let dense_hash: Vec<_> = sparse_hash
        .array_chunks()
        .map(|nums: &[usize; 16]| nums.iter().fold(0, |acc, v| acc ^ v))
        .collect();

    dense_hash
}

struct Grid([[bool; 128]; 128]);
impl Grid {
    fn new(input: &str) -> Self {
        let mut grid = [[false; 128]; 128];
        for (i, row) in grid.iter_mut().enumerate() {
            let row_hash = hard_hash(&format!("{input}-{i}"));
            for (j, byte) in row_hash.iter().enumerate() {
                for k in 0..8 {
                    if byte & (1 << (7 - k)) != 0 {
                        row[j * 8 + k] = true;
                    }
                }
            }
        }

        Self(grid)
    }

    fn defrag_region(&mut self, row: usize, col: usize) {
        if row > 127 || col > 127 {
            return;
        }

        let value = self.0[row][col];
        if !value {
            return;
        }

        self.0[row][col] = false;
        self.defrag_region(row + 1, col);
        self.defrag_region(row, col + 1);

        if row > 0 {
            self.defrag_region(row - 1, col);
        }

        if col > 0 {
            self.defrag_region(row, col - 1);
        }
    }

    fn count_used(&self) -> usize {
        self.0.flatten().iter().filter(|&&cell| cell).count()
    }
}

fn main() {
    let input = get_input_text(DAY);
    let mut grid = Grid::new(input.trim());

    let solution1: Solution = grid.count_used();
    let solution2: Solution = {
        let mut regions = 0;
        for i in 0..128 {
            for j in 0..128 {
                if grid.0[i][j] {
                    regions += 1;
                    grid.defrag_region(i, j);
                }
            }
        }

        regions
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
