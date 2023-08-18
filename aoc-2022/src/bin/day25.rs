use aoc_2022::*;

const DAY: i32 = 25;
type Solution = String;

fn parse_snafu(s: &str) -> isize {
    let mut num = 0;

    for ch in s.chars() {
        let v = match ch {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => unreachable!(),
        };

        num = num * 5 + v;
    }

    num
}

fn to_snafu(mut v: isize) -> String {
    const DIGITS: [char; 5] = ['=', '-', '0', '1', '2'];
    const ZERO_INDEX: isize = DIGITS.len() as isize / 2;

    if v == 0 {
        return "0".to_owned();
    }

    let mut s = String::new();
    while v > 0 {
        let digit = (v + ZERO_INDEX) % 5;
        s.push(DIGITS[digit as usize]);

        v /= 5;
        v += (digit < ZERO_INDEX) as isize;
    }

    s.chars().rev().collect()
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = {
        let value: isize = input.lines().map(parse_snafu).sum();
        to_snafu(value)
    };

    show_solution(DAY, solution1);
}

#[cfg(test)]
mod tests {
    use crate::{parse_snafu, to_snafu};

    #[test]
    fn test_parse_snafu() {
        assert_eq!(parse_snafu("1=-0-2"), 1747);
        assert_eq!(parse_snafu("12111"), 906);
        assert_eq!(parse_snafu("2=0="), 198);
        assert_eq!(parse_snafu("21"), 11);
        assert_eq!(parse_snafu("2=01"), 201);
        assert_eq!(parse_snafu("111"), 31);
        assert_eq!(parse_snafu("20012"), 1257);
        assert_eq!(parse_snafu("112"), 32);
        assert_eq!(parse_snafu("1=-1="), 353);
        assert_eq!(parse_snafu("1-12"), 107);
        assert_eq!(parse_snafu("12"), 7);
        assert_eq!(parse_snafu("1="), 3);
        assert_eq!(parse_snafu("122"), 37);
    }

    #[test]
    fn test_to_snafu() {
        assert_eq!(to_snafu(1), "1");
        assert_eq!(to_snafu(2), "2");
        assert_eq!(to_snafu(3), "1=");
        assert_eq!(to_snafu(4), "1-");
        assert_eq!(to_snafu(5), "10");
        assert_eq!(to_snafu(6), "11");
        assert_eq!(to_snafu(7), "12");
        assert_eq!(to_snafu(8), "2=");
        assert_eq!(to_snafu(9), "2-");
        assert_eq!(to_snafu(10), "20");
        assert_eq!(to_snafu(15), "1=0");
        assert_eq!(to_snafu(20), "1-0");
        assert_eq!(to_snafu(2022), "1=11-2");
        assert_eq!(to_snafu(12345), "1-0---0");
        assert_eq!(to_snafu(314159265), "1121-1110-1=0");
    }
}
