use aoc_2018::*;

const DAY: i32 = 8;
type Solution = usize;

struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn parse_node(mut data: &[usize]) -> (&[usize], Node) {
    let children_count = data[0];
    let metadata_size = data[1];

    // Children section
    data = &data[2..];
    let mut children = Vec::new();
    for _ in 0..children_count {
        let (shifted_data, child) = parse_node(data);
        children.push(child);
        data = shifted_data;
    }

    // Metadata section
    let metadata = data[..metadata_size].to_vec();
    (&data[metadata_size..], Node { children, metadata })
}

fn solve_part1(node: &Node) -> Solution {
    let node_value: Solution = node.metadata.iter().sum();
    let children_value: Solution = node.children.iter().map(solve_part1).sum();
    node_value + children_value
}

fn solve_part2(node: &Node) -> Solution {
    if node.children.is_empty() {
        return node.metadata.iter().sum();
    }

    node.metadata
        .iter()
        .filter_map(|&i| node.children.get(i - 1))
        .map(solve_part2)
        .sum()
}

fn main() {
    let input = get_input_text(DAY);
    let input: Vec<_> = input
        .trim()
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let (_, root_node) = parse_node(&input);

    let solution1: Solution = solve_part1(&root_node);
    let solution2: Solution = solve_part2(&root_node);

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::{parse_node, solve_part1, solve_part2};

    #[test]
    fn test_solve_part1() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let input: Vec<_> = input
            .trim()
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let (_, root_node) = parse_node(&input);
        assert_eq!(solve_part1(&root_node), 138);
    }

    #[test]
    fn test_solve_part2() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let input: Vec<_> = input
            .trim()
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let (_, root_node) = parse_node(&input);
        assert_eq!(solve_part2(&root_node), 66);
    }
}
