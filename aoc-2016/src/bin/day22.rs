use aoc_2016::*;

const DAY: i32 = 22;
type Solution = usize;

struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
}

impl Node {
    fn is_viable_pair(&self, other: &Node) -> bool {
        self.used > 0 && self.used <= other.size - other.used
    }
}

fn main() {
    let input = get_input_text(DAY);

    let nodes: Vec<_> = input
        .lines()
        .skip(2)
        .map(|line| {
            let words: Vec<_> = line.split_whitespace().collect();
            let name: Vec<_> = words[0].split(['-', 'x', 'y']).collect();
            let x: usize = name[2].parse().unwrap();
            let y: usize = name[4].parse().unwrap();
            let size = words[1].trim_end_matches('T').parse().unwrap();
            let used = words[2].trim_end_matches('T').parse().unwrap();
            Node { x, y, size, used }
        })
        .collect();

    let solution1: Solution = {
        let mut pairs = 0;
        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                if i != j && nodes[i].is_viable_pair(&nodes[j]) {
                    pairs += 1;
                }
            }
        }

        pairs
    };

    let solution2: Solution = {
        let width = nodes.iter().map(|node| node.x).max().unwrap();

        let empty_node = nodes.iter().find(|node| node.used == 0).unwrap();
        let first_wall_node = nodes
            .iter()
            .find(|node| node.used > empty_node.size)
            .unwrap();
        let start_node = nodes
            .iter()
            .find(|node| node.x == width && node.y == 0)
            .unwrap();

        let avoid_wall_moves = empty_node.x - first_wall_node.x + 1;
        let to_first_row_moves = empty_node.y;
        let to_start_moves = start_node.x - first_wall_node.x + 1;
        let to_end_moves = 5 * (start_node.x - 1);
        avoid_wall_moves + to_first_row_moves + to_start_moves + to_end_moves
    };

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
