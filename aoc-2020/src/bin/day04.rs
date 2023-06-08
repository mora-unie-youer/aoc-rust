use std::{collections::HashMap, ops::RangeInclusive};

use aoc_2020::*;

const DAY: i32 = 4;
type Solution = usize;

const REQUIRED: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
fn check_passport(passport: &HashMap<&str, &str>) -> bool {
    const BYR: RangeInclusive<usize> = 1920..=2002;
    const IYR: RangeInclusive<usize> = 2010..=2020;
    const EYR: RangeInclusive<usize> = 2020..=2030;
    const HGT_CM: RangeInclusive<usize> = 150..=193;
    const HGT_IN: RangeInclusive<usize> = 59..=76;
    const ECL: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    let mut valid = true;

    // BYR check
    valid &= passport
        .get("byr")
        .filter(|v| v.len() == 4)
        .and_then(|v| v.parse().ok())
        .map(|v| BYR.contains(&v))
        .unwrap_or(false);

    // IYR check
    valid &= passport
        .get("iyr")
        .filter(|v| v.len() == 4)
        .and_then(|v| v.parse().ok())
        .map(|v| IYR.contains(&v))
        .unwrap_or(false);

    // EYR check
    valid &= passport
        .get("eyr")
        .filter(|v| v.len() == 4)
        .and_then(|v| v.parse().ok())
        .map(|v| EYR.contains(&v))
        .unwrap_or(false);

    // HGT check
    valid &= if let Some(v) = passport.get("hgt") {
        if v.ends_with("cm") {
            v.trim_end_matches("cm")
                .parse()
                .ok()
                .map(|v| HGT_CM.contains(&v))
                .unwrap_or(false)
        } else if v.ends_with("in") {
            v.trim_end_matches("in")
                .parse()
                .ok()
                .map(|v| HGT_IN.contains(&v))
                .unwrap_or(false)
        } else {
            false
        }
    } else {
        false
    };

    // HCL check
    valid &= passport
        .get("hcl")
        .filter(|v| v.len() == 7)
        .filter(|v| v.starts_with('#'))
        .filter(|v| v[1..].chars().all(|ch| matches!(ch, '0'..='9' | 'a'..='f')))
        .is_some();

    // ECL check
    valid &= passport
        .get("ecl")
        .map(|v| ECL.contains(v))
        .unwrap_or(false);

    // PID check
    valid &= passport
        .get("pid")
        .filter(|v| v.len() == 9)
        .map(|v| v.parse::<usize>().is_ok())
        .unwrap_or(false);

    valid
}

fn main() {
    let input = get_input_text(DAY);
    let passports: Vec<HashMap<&str, &str>> = input
        .split("\n\n")
        .map(|passport| {
            passport
                .split_ascii_whitespace()
                .map(|field| field.split_once(':').unwrap())
                .collect()
        })
        .collect();

    let solution1: Solution = passports
        .iter()
        .filter(|passport| REQUIRED.iter().all(|field| passport.contains_key(field)))
        .count();
    let solution2: Solution = passports.into_iter().filter(check_passport).count();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
