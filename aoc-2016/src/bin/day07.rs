use aoc_2016::*;

const DAY: i32 = 7;
type Solution = usize;

trait HasABBA {
    fn has_abba(&self) -> bool;
}

impl HasABBA for &str {
    fn has_abba(&self) -> bool {
        let chars: Vec<_> = self.chars().collect();
        chars
            .windows(4)
            .any(|ch| ch[0] != ch[1] && ch[0] == ch[3] && ch[1] == ch[2])
    }
}

trait HasTLS {
    fn has_tls(&self) -> bool;
}

impl HasTLS for &str {
    fn has_tls(&self) -> bool {
        let abba = self.split(['[', ']']).map(|part| part.has_abba());
        let mut supernets = abba.clone().step_by(2);
        let mut hypernets = abba.clone().skip(1).step_by(2);
        hypernets.all(|v| !v) && supernets.any(|v| v)
    }
}

trait HasSSL {
    fn has_ssl(&self) -> bool;
}

impl HasSSL for &str {
    fn has_ssl(&self) -> bool {
        let nets = self.split(['[', ']']);
        let supernets: Vec<_> = nets.clone().step_by(2).collect();
        let hypernets: Vec<_> = nets.clone().skip(1).step_by(2).collect();

        let mut bab: Vec<String> = supernets
            .iter()
            .flat_map(|supernet| {
                supernet
                    .chars()
                    .collect::<Vec<_>>()
                    .windows(3)
                    .filter(|ch| ch[0] != ch[1] && ch[0] == ch[2])
                    .map(|ch| [ch[1], ch[0], ch[1]].iter().collect())
                    .collect::<Vec<String>>()
            })
            .collect();
        bab.dedup();

        bab.iter()
            .any(|s| hypernets.iter().any(|hypernet| hypernet.contains(s)))
    }
}

fn main() {
    let input = get_input_text(DAY);

    let solution1: Solution = input.lines().filter(|line| line.has_tls()).count();
    let solution2: Solution = input.lines().filter(|line| line.has_ssl()).count();

    show_solution(DAY, solution1);
    show_solution(DAY, solution2);
}

#[cfg(test)]
mod tests {
    use crate::{HasABBA, HasSSL, HasTLS};

    #[test]
    fn test_has_abba() {
        assert!(!"aaaa".has_abba());
        assert!("abba".has_abba());
    }

    #[test]
    fn test_has_tls() {
        assert!("abba[mnop]qrst".has_tls());
        assert!(!"abcd[bddb]xyyx".has_tls());
        assert!(!"aaaa[qwer]tyui".has_tls());
        assert!("ioxxoj[asdfgh]zxcvbn".has_tls());
    }

    #[test]
    fn test_has_ssl() {
        assert!("aba[bab]xyz".has_ssl());
        assert!(!"xyx[xyx]xyx".has_ssl());
        assert!("aaa[kek]eke".has_ssl());
        assert!("zazbz[bzb]cdb".has_ssl());
    }
}
