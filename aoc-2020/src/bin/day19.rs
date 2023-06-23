use std::collections::HashMap;

use aoc_2020::*;

const DAY: i32 = 19;
type Solution = usize;

enum Rule {
    // Char rule
    Char(char),
    // Sequence rule
    Sequence(Vec<usize>),
    // Choice rule
    Choice(Vec<usize>, Vec<usize>),
}

impl From<&str> for Rule {
    fn from(rule: &str) -> Self {
        if rule.contains('"') {
            let ch = rule.chars().nth(1).unwrap();
            Self::Char(ch)
        } else if rule.contains('|') {
            let sequences: Vec<Vec<usize>> = rule
                .split(" | ")
                .map(|seq| {
                    seq.split_ascii_whitespace()
                        .map(|v| v.parse().unwrap())
                        .collect()
                })
                .collect();
            Self::Choice(sequences[0].clone(), sequences[1].clone())
        } else {
            let sequence = rule
                .split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            Self::Sequence(sequence)
        }
    }
}

fn match_char(rules: &HashMap<usize, Rule>, ch: char, input: &str, queue: &mut Vec<usize>) -> bool {
    match input.chars().next() {
        Some(ch1) if ch == ch1 => matches_all(rules, &input[1..], queue),
        _ => false,
    }
}

fn match_seq(
    rules: &HashMap<usize, Rule>,
    seq: &[usize],
    input: &str,
    queue: &mut Vec<usize>,
) -> bool {
    seq.iter().rev().for_each(|r| queue.push(*r));
    matches_all(rules, input, queue)
}

fn matches_all(rules: &HashMap<usize, Rule>, input: &str, queue: &mut Vec<usize>) -> bool {
    if queue.is_empty() && input.is_empty() {
        return true;
    }

    if queue.is_empty() || input.is_empty() {
        return false;
    }

    let current = &rules[&queue.pop().unwrap()];
    match current {
        Rule::Char(ch) => match_char(rules, *ch, input, queue),
        Rule::Sequence(seq) => match_seq(rules, seq, input, queue),
        Rule::Choice(seq1, seq2) => {
            match_seq(rules, seq1, input, &mut queue.clone())
                || match_seq(rules, seq2, input, &mut queue.clone())
        }
    }
}

fn matches(rules: &HashMap<usize, Rule>, input: &str) -> bool {
    matches_all(rules, input, &mut vec![0])
}

fn main() {
    let input = get_input_text(DAY);
    let (rules, input) = input.split_once("\n\n").unwrap();

    let rules: HashMap<usize, Rule> = rules
        .lines()
        .map(|line| {
            let (id, rule) = line.split_once(": ").unwrap();
            let id = id.parse().unwrap();
            (id, Rule::from(rule))
        })
        .collect();

    let solution1: Solution = input.lines().filter(|input| matches(&rules, input)).count();

    let solution2: Solution = {
        let mut rules = rules;
        rules
            .entry(8)
            .and_modify(|v| *v = Rule::Choice(vec![42], vec![42, 8]));
        rules
            .entry(11)
            .and_modify(|v| *v = Rule::Choice(vec![42, 31], vec![42, 11, 31]));

        input.lines().filter(|input| matches(&rules, input)).count()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
