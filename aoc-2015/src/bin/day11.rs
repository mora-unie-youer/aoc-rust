use std::ops::ControlFlow;

use aoc_2015::*;

const DAY: i32 = 11;
type Solution = String;

trait PasswordChar {
    fn is_forbidden(&self) -> bool;
    fn next_char(&self) -> char;
    fn next_valid_char(&self) -> char;
}

impl PasswordChar for char {
    fn is_forbidden(&self) -> bool {
        match self {
            'i' | 'l' | 'o' => true,
            _ => false,
        }
    }

    fn next_char(&self) -> char {
        match self {
            'z' => 'a',
            _ => std::char::from_u32(*self as u32 + 1).unwrap(),
        }
    }

    fn next_valid_char(&self) -> char {
        let ch = self.next_char();
        match ch.is_forbidden() {
            true => ch.next_char(),
            _ => ch,
        }
    }
}

trait NextPassword {
    fn valid(&self) -> bool;
    fn next_password(&self) -> Self;
    fn next_valid_password(&self) -> Self;
}

impl NextPassword for String {
    fn valid(&self) -> bool {
        let chars: Vec<_> = self.chars().map(|ch| ch as u8).collect();

        let has_forbidden = chars
            .iter()
            .any(|&ch| ch == b'i' || ch == b'l' || ch == b'o');
        if has_forbidden {
            return false;
        }

        let has_straight = chars
            .windows(3)
            .any(|triad| triad[0] + 1 == triad[1] && triad[1] + 1 == triad[2]);
        if !has_straight {
            return false;
        }

        let has_doubles = chars
            .windows(2)
            .try_fold(None, |prev_pair, pair| match prev_pair {
                Some(v) if pair[0] == pair[1] && pair[0] != v => ControlFlow::Break(true),
                Some(v) if pair[0] == pair[1] && pair[0] == v => ControlFlow::Continue(None),
                None if pair[0] == pair[1] => ControlFlow::Continue(Some(pair[0])),
                _ => ControlFlow::Continue(prev_pair),
            });

        has_doubles.is_break()
    }

    fn next_password(&self) -> Self {
        let mut chars: Vec<_> = self.chars().collect();
        let mut carry = true;
        for ch in chars.iter_mut().rev() {
            if !carry {
                break;
            }

            *ch = ch.next_valid_char();
            carry = *ch == 'a';
        }

        chars.iter().collect()
    }

    fn next_valid_password(&self) -> Self {
        let mut new_password: String = self.next_password();
        while !new_password.valid() {
            new_password = new_password.next_password();
        }
        new_password
    }
}

fn main() {
    let input = get_input_text(DAY);
    let password = input.trim();

    let solution1: Solution = password.to_string().next_valid_password();
    let solution2: Solution = solution1.next_valid_password();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::NextPassword;

    #[test]
    fn test_valid_password() {
        assert_eq!(String::from("abcdefgh").valid(), false);
        assert_eq!(String::from("abcdffaa").valid(), true);
        assert_eq!(String::from("ghijklmn").valid(), false);
        assert_eq!(String::from("ghjaabcc").valid(), true);
    }

    #[test]
    fn test_next_valid_password() {
        assert_eq!(
            String::from("abcdefgh").next_valid_password(),
            String::from("abcdffaa")
        );
        assert_eq!(
            String::from("ghijklmn").next_valid_password(),
            String::from("ghjaabcc")
        );
    }
}
