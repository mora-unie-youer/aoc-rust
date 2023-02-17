use std::collections::HashSet;

use aoc_2015::*;

const DAY: i32 = 19;
type Solution = usize;

fn main() {
    let input = get_input_text(DAY);
    let (replacements, molecule) = input.trim().split_once("\n\n").unwrap();

    let solution1: Solution = {
        let replacements: Vec<(&str, &str)> = replacements
            .lines()
            .map(|line| line.split_once(" => ").unwrap())
            .collect();

        let mut molecules = HashSet::new();
        for (from, to) in replacements.iter() {
            for (i, _) in molecule.match_indices(from) {
                let mut new_molecule = String::new();
                new_molecule.push_str(&molecule[..i]);
                new_molecule.push_str(to);
                new_molecule.push_str(&molecule[i + from.len()..]);
                molecules.insert(new_molecule);
            }
        }
        molecules.len()
    };

    // 1. e => <a><b>
    // 2. <a> => <b><c>
    // 3. <a> => <b> Rn <c> Ar
    // 4. <a> => <b> Rn <c> Y <d> Ar
    // 5. <a> => <b> Rn <c> Y <d> Y <e> Ar
    let solution2: Solution = {
        let atoms = molecule.match_indices(char::is_uppercase).map(|(i, ch)| {
            let mut atom = ch.to_string();
            match molecule.chars().nth(i + 1) {
                Some('e') | None => (),
                Some(v) if v.is_lowercase() => atom.push(v),
                _ => (),
            }
            atom
        });
        
        let rn_ar = atoms.clone().filter(|atom| atom == "Rn" || atom == "Ar");
        let y = atoms.clone().filter(|atom| atom == "Y");

        atoms.count() - rn_ar.count() - 2 * y.count() - 1
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
