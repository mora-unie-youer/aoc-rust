use std::ops::RangeInclusive;

use aoc_2020::*;

const DAY: i32 = 16;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let (rules, my_ticket, other_tickets) = {
        let mut parts = input.split("\n\n");
        let rules = parts.next().unwrap();
        let my_ticket = parts.next().unwrap().lines().nth(1).unwrap().trim();
        let other_tickets = parts.next().unwrap();
        (rules, my_ticket, other_tickets)
    };

    let rules: Vec<(&str, [RangeInclusive<usize>; 2])> = rules
        .lines()
        .map(|line| {
            let (name, ranges) = line.split_once(": ").unwrap();
            let (r1, r2) = ranges.split_once(" or ").unwrap();

            let r1 = r1
                .split_once('-')
                .map(|(a, b)| a.parse().unwrap()..=b.parse().unwrap())
                .unwrap();
            let r2 = r2
                .split_once('-')
                .map(|(a, b)| a.parse().unwrap()..=b.parse().unwrap())
                .unwrap();

            (name, [r1, r2])
        })
        .collect();

    let my_ticket: Vec<usize> = my_ticket.split(',').map(|v| v.parse().unwrap()).collect();
    let other_tickets: Vec<Vec<usize>> = other_tickets
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|v| v.parse().unwrap()).collect())
        .collect();

    let solution1: Solution = other_tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter(|value| {
                    rules
                        .iter()
                        .all(|(_, rule)| rule.iter().all(|range| !range.contains(value)))
                })
                .sum::<usize>()
        })
        .sum();

    let solution2: Solution = {
        let other_tickets: Vec<_> = other_tickets
            .iter()
            .filter(|ticket| {
                ticket.iter().all(|value| {
                    rules
                        .iter()
                        .any(|(_, rule)| rule.iter().any(|range| range.contains(value)))
                })
            })
            .collect();

        let mut possible_rules: Vec<Vec<usize>> = rules
            .iter()
            .map(|(_, rule)| {
                (0..rules.len())
                    .filter(|&i| {
                        other_tickets
                            .iter()
                            .all(|ticket| rule.iter().any(|range| range.contains(&ticket[i])))
                    })
                    .collect()
            })
            .collect();

        let mut mapped_rules = vec![0; rules.len()];
        while let Some(i) = possible_rules.iter().position(|fields| fields.len() == 1) {
            let v = possible_rules[i][0];
            mapped_rules[i] = v;
            possible_rules.iter_mut().for_each(|fields| {
                fields.retain(|&field| field != v);
            });
        }

        rules
            .iter()
            .enumerate()
            .filter(|(_, (name, _))| name.starts_with("departure"))
            .map(|(i, _)| my_ticket[mapped_rules[i]])
            .product()
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
