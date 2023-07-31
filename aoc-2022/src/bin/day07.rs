use std::collections::HashMap;

use aoc_2022::*;

const DAY: i32 = 7;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);

    let fs = {
        let mut cwd = String::new();
        let mut fs = HashMap::new();
        for line in input.lines().filter(|&s| s != "$ ls" && &s[0..3] != "dir") {
            match line {
                "$ cd /" => {
                    cwd.push('.');
                }
                "$ cd .." => {
                    cwd.truncate(cwd.rfind('/').unwrap());
                }
                _ if line.starts_with("$ cd") => {
                    cwd.push('/');
                    cwd.push_str(&line[5..]);
                }
                _ => {
                    let fsize: Solution = line
                        .split_ascii_whitespace()
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();
                    let dsize = fs.entry(cwd.clone()).or_insert(0);
                    *dsize += fsize;
                    cwd.match_indices('/').for_each(|(i, _)| {
                        let dsize = fs.entry(cwd[0..i].to_owned()).or_insert(0);
                        *dsize += fsize;
                    });
                }
            }
        }

        fs
    };

    const THRESHOLD: usize = 100000;
    const TOTAL: usize = 70000000;
    const UPDATE: usize = 30000000;
    let solution1: Solution = fs.values().filter(|&&v| v <= THRESHOLD).sum();

    let solution2: Solution = {
        let needed = UPDATE - (TOTAL - fs["."]);
        fs.values()
            .filter(|&&v| v >= needed)
            .min()
            .unwrap()
            .to_owned()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
