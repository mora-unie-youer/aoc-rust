use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

use aoc_2024::*;
use itertools::Itertools;

const DAY: i32 = 9;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let disk: BTreeMap<(usize, usize), Option<usize>> = {
        let mut disk = BTreeMap::new();

        let mut current = 0;
        for (i, ch) in input.trim().chars().enumerate() {
            let size = (ch as u8 - b'0') as usize;
            if size == 0 {
                continue;
            }

            let block_id = (i % 2 == 0).then_some(i / 2);
            disk.insert((current, current + size - 1), block_id);
            current += size;
        }

        disk
    };

    let solution1: Solution = {
        let mut disk = disk.clone();

        let total_size: usize = disk
            .iter()
            .filter(|(_, v)| v.is_some())
            .map(|((start, end), _)| end - start + 1)
            .sum();
        let mut used_blocks = disk
            .iter()
            .filter(|(_, v)| v.is_some())
            .map(|(k, v)| (*k, *v))
            .collect_vec();
        let mut free_blocks = disk
            .iter()
            .filter(|(_, v)| v.is_none())
            .rev()
            .map(|(k, _)| *k)
            .collect_vec();

        while let Some(&(free_start, free_end)) = free_blocks.last() {
            if free_start >= total_size {
                // Otherwise we would place data to wrong place
                break;
            }

            let ((used_start, used_end), used_id) = *used_blocks.last().unwrap();
            let free_size = free_end - free_start + 1;
            let used_size = used_end - used_start + 1;

            match free_size.cmp(&used_size) {
                Ordering::Less => {
                    disk.insert((free_start, free_end), used_id);
                    disk.remove(&(used_start, used_end));
                    disk.insert((used_start, used_end - free_size), used_id);
                    free_blocks.pop();
                    used_blocks.pop();
                    used_blocks.push(((used_start, used_end - free_size), used_id));
                }

                Ordering::Equal => {
                    disk.insert((free_start, free_end), used_id);
                    disk.remove(&(used_start, used_end));
                    free_blocks.pop();
                    used_blocks.pop();
                }

                Ordering::Greater => {
                    disk.remove(&(free_start, free_end));
                    disk.remove(&(used_start, used_end));
                    disk.insert((free_start, free_start + used_size - 1), used_id);
                    disk.insert((free_start + used_size, free_end), None);
                    free_blocks.pop();
                    free_blocks.push((free_start + used_size, free_end));
                    used_blocks.pop();
                }
            }
        }

        disk.iter()
            .filter(|(_, v)| v.is_some())
            .map(|(&(a, b), v)| (a..=b).map(|i| i * v.unwrap()).sum::<usize>())
            .sum()
    };

    let solution2: Solution = {
        let mut disk = disk;

        let mut used_blocks = disk
            .iter()
            .filter(|(_, v)| v.is_some())
            .map(|(k, v)| (*k, *v))
            .collect_vec();
        used_blocks.sort_by_key(|(_, id)| *id);

        let mut free_blocks: BTreeSet<(usize, usize)> = disk
            .iter()
            .filter(|(_, v)| v.is_none())
            .map(|(k, _)| *k)
            .collect();

        while let Some(((used_start, used_end), used_id)) = used_blocks.pop() {
            let free_block = free_blocks
                .iter()
                .filter(|&(_, end)| *end < used_start)
                .find(|(start, end)| used_end - used_start <= end - start);
            if let Some(&(free_start, free_end)) = free_block {
                let used_size = used_end - used_start + 1;
                let free_size = free_end - free_start + 1;

                if used_size == free_size {
                    // No need to restore free block as lower ID can't be there
                    disk.remove(&(used_start, used_end));
                    disk.insert((free_start, free_end), used_id);
                    free_blocks.remove(&(free_start, free_end));
                } else {
                    disk.remove(&(free_start, free_end));
                    disk.remove(&(used_start, used_end));
                    disk.insert((free_start, free_start + used_size - 1), used_id);
                    disk.insert((free_start + used_size, free_end), None);
                    free_blocks.remove(&(free_start, free_end));
                    free_blocks.insert((free_start + used_size, free_end));
                }
            }
        }

        disk.iter()
            .filter(|(_, v)| v.is_some())
            .map(|(&(a, b), v)| (a..=b).map(|i| i * v.unwrap()).sum::<usize>())
            .sum()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
