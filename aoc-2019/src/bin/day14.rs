use std::collections::HashMap;

use aoc_2019::*;

const DAY: i32 = 14;
type Solution = usize;

type Chemical<'input> = &'input str;
struct Reaction<'input> {
    inputs: HashMap<Chemical<'input>, usize>,
    output: (Chemical<'input>, usize),
}

fn parse_reactions(input: &str) -> HashMap<Chemical, Reaction> {
    let mut reactions = HashMap::new();

    for line in input.lines() {
        let (inputs, output) = line.split_once(" => ").unwrap();
        let inputs = inputs
            .split(", ")
            .map(|input| {
                let (quantity, chemical) = input.split_once(' ').unwrap();
                (chemical, quantity.parse().unwrap())
            })
            .collect();

        let (quantity, chemical) = output.split_once(' ').unwrap();
        let output = (chemical, quantity.parse().unwrap());

        let reaction = Reaction { inputs, output };
        reactions.insert(output.0, reaction);
    }

    reactions
}

fn solve<'input>(
    reactions: &HashMap<Chemical, Reaction<'input>>,
    chemical: &'input str,
    quantity: usize,
    surplus: &mut HashMap<Chemical<'input>, usize>,
) -> usize {
    if chemical == "ORE" {
        return quantity;
    }

    let reaction = &reactions[chemical];
    let output_quantity = reaction.output.1;
    let mut ore = 0;

    let surplus_quantity = surplus.entry(chemical).or_insert(0);
    if *surplus_quantity >= quantity {
        *surplus_quantity -= quantity;
        return 0;
    }

    let required_quantity = quantity - *surplus_quantity;
    let reaction_count = (required_quantity + output_quantity - 1) / output_quantity;

    *surplus_quantity = reaction_count * output_quantity - required_quantity;

    for (input_chemical, input_quantity) in reaction.inputs.iter() {
        ore += solve(
            reactions,
            input_chemical,
            input_quantity * reaction_count,
            surplus,
        );
    }

    ore
}

fn main() {
    let input = get_input_text(DAY);
    let reactions = parse_reactions(&input);

    let solution1: Solution = solve(&reactions, "FUEL", 1, &mut HashMap::new());

    const AVAILABLE_ORE: usize = 1_000_000_000_000;
    let solution2: Solution = {
        let (mut low, mut high) = (0, AVAILABLE_ORE);
        while low < high {
            let mid = (high + low + 1) / 2;
            let ore_required = solve(&reactions, "FUEL", mid, &mut HashMap::new());

            if ore_required > AVAILABLE_ORE {
                high = mid - 1;
            } else {
                low = mid;
            }
        }

        low
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{parse_reactions, solve};

    #[test]
    fn test_solve() {
        let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let reactions = parse_reactions(input);
        assert_eq!(solve(&reactions, "FUEL", 1, &mut HashMap::new()), 31);

        let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let reactions = parse_reactions(input);
        assert_eq!(solve(&reactions, "FUEL", 1, &mut HashMap::new()), 165);

        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let reactions = parse_reactions(input);
        assert_eq!(solve(&reactions, "FUEL", 1, &mut HashMap::new()), 13312);

        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        let reactions = parse_reactions(input);
        assert_eq!(solve(&reactions, "FUEL", 1, &mut HashMap::new()), 180697);

        let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        let reactions = parse_reactions(input);
        assert_eq!(solve(&reactions, "FUEL", 1, &mut HashMap::new()), 2210736);
    }
}
