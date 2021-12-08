use std::collections::BTreeSet;

use itertools::Itertools;

#[must_use]
pub fn solve(input: &str) -> usize {
    input
        .lines()
        .filter_map(|s| s.split(" | ").collect_tuple())
        .map(|(input, output)| {
            let mut digits = vec![BTreeSet::new(); 10];
            let mut len6: Vec<BTreeSet<u8>> = vec![];
            let mut len5: Vec<BTreeSet<u8>> = vec![];

            for s in input.split_ascii_whitespace() {
                let d = s.bytes().collect();
                match s.len() {
                    2 => digits[1] = d,
                    4 => digits[4] = d,
                    3 => digits[7] = d,
                    7 => digits[8] = d,
                    5 => len5.push(d),
                    6 => len6.push(d),
                    _ => unreachable!(),
                }
            }

            for d in len6 {
                if !d.is_superset(&digits[1]) {
                    digits[6] = d;
                } else if !d.is_superset(&digits[4]) {
                    digits[0] = d;
                } else {
                    digits[9] = d;
                }
            }

            for d in len5 {
                if d.is_subset(&digits[6]) {
                    digits[5] = d;
                } else if d.is_subset(&digits[9]) {
                    digits[3] = d;
                } else {
                    digits[2] = d;
                }
            }

            output.split_ascii_whitespace().fold(0, |acc, s| {
                let signal: BTreeSet<_> = s.bytes().collect();
                let digit = digits.iter().position(|d| *d == signal).unwrap();

                acc * 10 + digit
            })
        })
        .sum()
}
