use aoc_2021::*;

const DAY: i32 = 16;
type Solution = usize;

trait ToNumber {
    fn to_number(&self) -> usize;
}

impl ToNumber for [u8] {
    fn to_number(&self) -> usize {
        self.iter().fold(0, |acc, &bit| (acc << 1) | bit as usize)
    }
}

enum Data {
    Literal(usize),
    Operator(Vec<Packet>),
}

impl Data {
    fn parse(input: &[u8], packet_type: u8) -> (&[u8], Self) {
        match packet_type {
            4 => Self::parse_literal(input),
            _ => Self::parse_operator(input),
        }
    }

    fn parse_literal(mut input: &[u8]) -> (&[u8], Self) {
        let mut value = 0;

        loop {
            // Parse bit group
            let last = input[0] == 0;
            let group = input[1..5].to_number();
            // Shift input to next group
            input = &input[5..];
            // Append to value
            value = (value << 4) | group;

            // If this was last group, leaving the loop
            if last {
                break;
            }
        }

        (input, Self::Literal(value))
    }

    fn parse_operator(input: &[u8]) -> (&[u8], Self) {
        let length_type = input[0];
        let input = &input[1..];

        if length_type == 1 {
            // If length is defined by number of packets
            let count = input[0..11].to_number();
            let mut input = &input[11..];

            // Parsing inner packets
            let mut packets = vec![];
            for _ in 0..count {
                // Parse packet
                let (new_input, packet) = Packet::parse(input);
                packets.push(packet);
                // Shift input to next packet
                input = new_input;
            }

            (input, Self::Operator(packets))
        } else {
            // If length is defined by total length
            let length = input[0..15].to_number();
            let input = &input[15..];

            // Splitting input into two parts
            let (mut packet_input, input) = input.split_at(length);

            // Parsing inner packets
            let mut packets = vec![];
            while !packet_input.is_empty() {
                // Parse packet
                let (new_input, packet) = Packet::parse(packet_input);
                packets.push(packet);
                // Shift input to next packet
                packet_input = new_input;
            }

            (input, Self::Operator(packets))
        }
    }
}

struct Packet {
    packet_version: u8,
    packet_type: u8,
    data: Data,
}

impl Packet {
    fn parse(input: &[u8]) -> (&[u8], Self) {
        let packet_version = input[0..3].to_number() as u8;
        let packet_type = input[3..6].to_number() as u8;
        let (input, data) = Data::parse(&input[6..], packet_type);

        let packet = Self {
            packet_version,
            packet_type,
            data,
        };

        (input, packet)
    }

    fn versions_sum(&self) -> usize {
        let data_version_sum = match &self.data {
            Data::Literal(_) => 0,
            Data::Operator(packets) => packets.iter().map(Packet::versions_sum).sum(),
        };

        self.packet_version as usize + data_version_sum
    }

    fn eval(self) -> usize {
        match self.data {
            Data::Literal(v) => v,
            Data::Operator(packets) => {
                let values: Vec<usize> = packets.into_iter().map(|packet| packet.eval()).collect();
                match self.packet_type {
                    0 => values.into_iter().sum(),
                    1 => values.into_iter().product(),
                    2 => values.into_iter().min().unwrap(),
                    3 => values.into_iter().max().unwrap(),
                    5 => (values[0] > values[1]) as usize,
                    6 => (values[0] < values[1]) as usize,
                    7 => (values[0] == values[1]) as usize,
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn main() {
    let input = get_input_text(DAY);

    let bits: Vec<u8> = input
        .trim()
        .chars()
        .flat_map(|ch| match ch {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            _ => unreachable!(),
        })
        .collect();

    let (_, packet) = Packet::parse(&bits);

    let solution1: Solution = packet.versions_sum();
    let solution2: Solution = packet.eval();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}
